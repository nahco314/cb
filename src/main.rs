use atty::Stream;
use std::io::{self, Read, Write};

fn get_argument() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        None
    } else if args.len() == 2 {
        Some(args[1].clone())
    } else {
        eprintln!("Error: Too many arguments");
        std::process::exit(1);
    }
}

#[cfg(not(target_os = "linux"))]
fn set_text(text: &str) {
    let mut clipboard = match arboard::Clipboard::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to initialize clipboard: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = clipboard.set_text(text) {
        eprintln!("Error: Failed to write to clipboard: {}", e);
        std::process::exit(1);
    }
}

// In X11, if the process ends immediately after writing to the clipboard,
// it will not be written correctly.
// See also https://github.com/1Password/arboard/issues/143
//
// Therefore, we will sleep long enough for the child process to write and for X11 to read it.
// It seems that eprintln, etc., are propagated properly from the child process.
#[cfg(target_os = "linux")]
fn set_text(text: &str) {
    use std::thread;
    use std::process;

    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            eprintln!("Error: fork failed");
            process::exit(1);
        } else if pid == 0 {
            // on child process
            let mut clipboard = match arboard::Clipboard::new() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error: Failed to initialize clipboard: {}", e);
                    process::exit(1);
                }
            };
            if let Err(e) = clipboard.set_text(text) {
                eprintln!("Error: Failed to write to clipboard: {}", e);
                process::exit(1);
            }
            thread::sleep(std::time::Duration::from_millis(1000));
            process::exit(0);
        }

        // on parent process, do nothing
    }
}

fn main() {
    let stdin_is_piped = !atty::is(Stream::Stdin);
    let stdout_is_piped = !atty::is(Stream::Stdout);

    let arg = get_argument();

    // Error handling

    // `cb < file > file`
    if stdin_is_piped && stdout_is_piped {
        eprintln!("Error: Both stdin and stdout are piped");
        std::process::exit(1);
    }

    // `cb "text" > file`
    if arg.is_some() && (stdin_is_piped || stdout_is_piped) {
        eprintln!("Error: Text and redirection are specified at the same time");
        std::process::exit(1);
    }

    // `cb "text"`
    if let Some(text) = arg {
        set_text(&text);
        return;
    }

    // `cb < file`
    if stdin_is_piped {
        let mut input = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut input) {
            eprintln!("Error: Failed to read from stdin: {}", e);
            std::process::exit(1);
        }

        set_text(&input);
        return;
    }

    // In else case, we just read from clipboard and write to stdout.
    let mut clipboard = match arboard::Clipboard::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to initialize clipboard: {}", e);
            std::process::exit(1);
        }
    };
    let text = match clipboard.get_text() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: Failed to read from clipboard: {}", e);
            std::process::exit(1);
        }
    };
    if let Err(e) = io::stdout().write_all(text.as_bytes()) {
        eprintln!("Error: Failed to write to stdout: {}", e);
        std::process::exit(1);
    }
}
