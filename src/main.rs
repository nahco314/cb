use atty::Stream;
use std::env;
use std::io::{self, Read, Write};

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
    use std::process;
    use std::thread;

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

fn get_text() -> String {
    let mut clipboard = match arboard::Clipboard::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to initialize clipboard: {}", e);
            std::process::exit(1);
        }
    };

    match clipboard.get_text() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: Failed to read from clipboard: {}", e);
            std::process::exit(1);
        }
    }
}

fn format_size(bytes: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let b = bytes as f64;
    if b < KB {
        format!("{} bytes", bytes)
    } else if b < MB {
        format!("{:.2} KB", b / KB)
    } else if b < GB {
        format!("{:.2} MB", b / MB)
    } else if b < TB {
        format!("{:.2} GB", b / GB)
    } else {
        format!("{:.2} TB", b / TB)
    }
}

fn main() {
    // `cb size`
    let mut args = env::args();
    let _exe = args.next();
    if let Some(sub) = args.next() {
        if sub == "size" {
            println!("{}", format_size(get_text().as_bytes().len()));
            return;
        }
    }

    let stdin_is_piped = !atty::is(Stream::Stdin);
    let stdout_is_piped = !atty::is(Stream::Stdout);

    // `cb < file > file`
    if stdin_is_piped && stdout_is_piped {
        eprintln!("Error: Both stdin and stdout are piped");
        std::process::exit(1);
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
    let text = get_text();
    if let Err(e) = io::stdout().write_all(text.as_bytes()) {
        eprintln!("Error: Failed to write to stdout: {}", e);
        std::process::exit(1);
    }
}
