# </> Shelly
A high-performance Unix-style shell written in **Rust**. Built to explore systems programming, process lifecycles, and Inter-Process Communication (IPC).

## 🚀 Features
- **Process Management**: Executes external binaries using the Fork-Exec pattern.
- **Built-in Commands**: Custom implementations for `cd`, `pwd`, and `exit`.
- **I/O Redirection**: Supports output redirection to files using `>`.
- **Piping**: Full Inter-Process Communication (IPC) support using `|` to link process streams.
- **Memory Safety**: Leverages Rust's ownership model to ensure safe file descriptor handling.

## 🛠️ Technical Implementation
- **Standard Streams**: Manages `stdin`, `stdout`, and `stderr` through the `std::process` module.
- **Environment**: Manipulates the Process Environment Block (PEB) for directory navigation.
- **Tokenization**: Implements an efficient string parser using `split_whitespace` to handle command arguments.

## 📦 Installation & Usage
1. Clone the repo.
2. Build the project: `cargo build --release`
3. Run the shell: `./target/release/shelly`
