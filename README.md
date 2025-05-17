# cb

cb is a lightweight, no-frills command-line tool designed to interact with your system clipboard. It provides an incredibly simple interface—just one command—to copy, paste, and redirect clipboard data without any extra noise.

## Features

- **Ultra-Simple:** One command does it all.
- **Intuitive I/O:** Use shell redirection for file operations.
- **No Dependencies:** Minimal setup with zero configuration.
- **Cross-Platform:** Works on your favorite OS/shell.

## Usage

```shell
# paste to stdout
cb

# paste to output.txt
cb > output.txt

# copy from input.txt
cb < input.txt

# copy output of "ls"
ls | cb

# show size of clipboard content
cb size
```

And, that's it!

## Install

Install cb using the latest installer.

```shell
# On macOS and Linux.
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nahco314/cb/releases/latest/download/cb-installer.sh | sh
```

```shell
# On Windows.
powershell -ExecutionPolicy ByPass -c "irm https://github.com/nahco314/cb/releases/latest/download/cb-installer.ps1 | iex"
```

Or, use cargo to install cb.

```shell
cargo install cb --git https://github.com/nahco314/cb
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License.
