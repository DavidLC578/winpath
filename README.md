# 🛠️ WinPath

A powerful and user-friendly command-line utility for managing the **current user's** PATH environment variable in Windows, written in Rust. This tool specifically modifies the PATH stored in `HKEY_CURRENT_USER\Environment`, affecting only the current user's session.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-0078D4?style=for-the-badge&logo=windows&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)

## ✨ Features

- 📋 **List PATH entries** - View all directories in your PATH with colored output and duplicate detection
- ➕ **Add new paths** - Safely add new directories to your PATH with duplicate prevention
- 🎨 **Colored output** - Beautiful, color-coded terminal output for better readability
- 🔍 **Duplicate detection** - Automatically identifies and highlights duplicate PATH entries
- ⚡ **Fast and efficient** - Built with Rust for maximum performance and safety

## 🚀 Installation

### Prerequisites

- Rust 1.70+ (edition 2024)
- Windows operating system

### Build from source

```bash
git clone https://github.com/DavidLC578/winpath.git
cd winpath
cargo build --release
```

The compiled binary will be available in `target/release/winpath.exe`.

## 📖 Usage

WinPath provides a simple and intuitive command-line interface:

### List PATH entries

```bash
winpath list
```

This will display all directories in your PATH environment variable:

- Each entry is numbered for easy reference
- Paths are displayed in yellow for visibility
- Index numbers are shown in blue
- Duplicate entries are highlighted in red

### Add a new path

```bash
winpath add "C:\New\Program\Bin"
```

The utility will:

- Check if the path already exists (case-insensitive comparison)
- Normalize paths by removing trailing backslashes
- Add the path if it doesn't exist
- Display success or error messages with appropriate colors

## 🎯 Examples

### Listing PATH with duplicate detection

```bash
$ winpath list
1: C:\Program Files\Java\jdk-17\bin
2: C:\Program Files\nodejs\
3: C:\Users\User\AppData\Local\Microsoft\WindowsApps
4: C:\Program Files\Java\jdk-17\bin
Duplicates: C:\Program Files\Java\jdk-17\bin
```

### Adding a new path

```bash
$ winpath add "C:\New\Tool\bin"
Path C:\New\Tool\bin added to the path
```

### Adding an existing path

```bash
$ winpath add "C:\Program Files\Java\jdk-17\bin"
Path C:\Program Files\Java\jdk-17\bin is already in the path
```

## 🏗️ Technical Details

### Architecture

WinPath is built using modern Rust practices:

- **CLI Framework**: Uses `clap` with derive macros for elegant command-line parsing
- **Registry Access**: Leverages `winreg` crate for safe Windows Registry operations
- **Colored Output**: Implements `colored` crate for beautiful terminal output
- **Path Normalization**: Intelligent handling of trailing backslashes and case sensitivity

### Dependencies

| Crate     | Version | Purpose                       |
| --------- | ------- | ----------------------------- |
| `clap`    | 4.5.56  | Command-line argument parsing |
| `colored` | 3.1.1   | Terminal color formatting     |
| `winreg`  | 0.55.0  | Windows Registry access       |

### Registry Operations

WinPath operates specifically on the **current user's** environment registry key:

- **Registry Path**: `HKEY_CURRENT_USER\Environment`
- **User-specific**: Changes affect only the current user, not system-wide PATH
- **Read access**: `KEY_QUERY_VALUE` permission for listing PATH
- **Write access**: `KEY_SET_VALUE` permission for adding paths
- **Safe operations**: All registry operations include proper error handling

> **Note**: This tool modifies the user-level PATH, not the system-wide PATH (which would require `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Environment`).

## 🔧 Development

### Running in development mode

```bash
cargo run -- list
cargo run -- add "C:\Test\Path"
```

### Building for release

```bash
cargo build --release
```

### Running tests

```bash
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Guidelines

- Follow Rust best practices and idioms
- Ensure proper error handling for all registry operations
- Maintain consistent code formatting with `rustfmt`
- Add comprehensive tests for new features
- Update documentation for any API changes

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The [Rust community](https://www.rust-lang.org/community) for creating such an amazing language
- [clap](https://github.com/clap-rs/clap) for the excellent CLI argument parsing
- [winreg](https://github.com/gentz90/winreg-rs) for Windows Registry access
- [colored](https://github.com/mackwic/colored) for beautiful terminal colors

---

**Made with ❤️ and Rust**
