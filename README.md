# 🛠 Language Revamp

![Rust](https://img.shields.io/badge/Rust-1.56+-orange)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/language-revamp.svg)](https://crates.io/crates/language-revamp)
[![Platform Support](https://img.shields.io/badge/platforms-Windows%20%7C%20Linux%20%7C%20macOS-blueviolet)]()
[![Build Status](https://img.shields.io/github/actions/workflow/status/Dokuqui/LanguageRevamp/.github/workflows/rust.yml?branch=main)](https://github.com/Dokuqui/LanguageRevamp/actions)
[![GitHub Last Commit](https://img.shields.io/github/last-commit/Dokuqui/LanguageRevamp.svg)](https://github.com/Dokuqui/LanguageRevamp/commits/main)
[![GitHub Code Size](https://img.shields.io/github/languages/code-size/Dokuqui/LanguageRevamp)](https://github.com/Dokuqui/LanguageRevamp)

![LanguageRevamp](https://socialify.git.ci/Dokuqui/LanguageRevamp/image?description=1&font=Source+Code+Pro&forks=1&issues=1&language=1&name=1&owner=1&pattern=Floating+Cogs&pulls=1&stargazers=1&theme=Auto)

**Language Revamp** is a cross-platform CLI tool for managing programming language installations and updates.  
Currently, it supports **Go**, **Rust**, and **Python** with commands to **check**, **update**, and **install** these languages.

## 🚀 Features

- ✅ **Check** installed versions of Go, Rust, and Python.
- 🔄 **Update** existing installations to the latest version.
- 📦 **Install** missing languages if not detected.
- 🖥️ Supports **Windows** and **Linux (Debian, Ubuntu, WSL)**, also should work on **MacOS**.

## 🛑

- **JAVA** functionality not yet fully tested.

## 📥 Installation (Local Development)

To use **Language Revamp** locally:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Dokuqui/LanguageRevamp.git
   cd LanguageRevamp
   ```
2. **Build the project**:
    ```bash
   cargo build
    ```
3. **Run the project**:
    ```bash
   cargo run -- <language> <command>
    ```
   
    **Example**:
    ```bash
   cargo run -- python --check
    ```

## 🔧 Install from Crates.io (After Publishing)
Once **Language Revamp** is published, you will be able to install it using:

#### **Cargo**
```bash
    cargo install language-revamp
```

## 🏗️ Download Pre-built Binaries (After Release)

1. **Download the Correct Binary for Your OS**:
    - **Windows**: `language-revamp-windows.exe`
    - **Linux**: `language-revamp-linux`

2. **Move the Binary to a System Path**:
   Use the following commands to move the binary and make it executable (for Linux):

   ```bash
   mv language-revamp-linux /usr/local/bin/language-revamp
   chmod +x /usr/local/bin/language-revamp
   ```
   
3. **Verify installation**:
   ```bash
   language-revamp --help
   ```
   
4. **📌 Windows Users**:
   Move the .exe to a folder included in your PATH, or manually add it to the PATH variable.

🛠 Commands and Usage after Install
---------------------

### 🔍 Check Installed Versions

Check if a language is installed and its current version.
```bash
    language-revamp <language> --check
```

**Example**:
```bash
    language-revamp go --check
```

### 📦 Install a Language

Install the selected language if not found on the system.
```bash
    language-revamp <language> --install
```

**Example**:
```bash
    language-revamp rust --install
```

### 🔄 Update a Language

Update the selected language to the latest version.
```bash
    language-revamp <language> --update
```

Example:
```bash
    language-revamp python --update
```

🔧 Supported Languages
----------------------

| Language    | Check | Install | Update |
|-------------| --- | --- | --- |
| **Go**      | ✅ | ✅ | ✅ |
| **Rust**    | ✅ | ✅ | ✅ |
| **Python**  | ✅ | ✅ | ✅ |
| **Node JS** | ✅ | ✅ | ✅ |
| **JAVA**    | ✅ | ✅ | ✅ |

🚀 Future Installation Methods (Planned)
----------------------

We aim to make Language Revamp easier to install by supporting:

- **Windows**: **scoop** or **winget**
- **MacOS**: **brew**
- **Linux**: **.deb** and **.rpm** packages

📝 Roadmap
----------------------

- 📌 Add support for more languages.
- 📦 Improve error handling and logging.
- 🚀 Publish as a standalone executable.
- 📜 Add detailed documentation and examples.

👨‍💻 Contributing
----------------------
Pull requests are welcome! If you find a bug or want to request a feature, open an issue.
