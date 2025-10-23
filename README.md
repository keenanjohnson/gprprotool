# GprProTool

A user-friendly Text User Interface (TUI) application for converting GoPro .gpr image files to common formats (JPG/PNG).

## Problem Statement

GoPro cameras produce .gpr (GoPro RAW) photo files that are not natively supported by most image viewers and editors, making it difficult to view and edit the photos. While the [GoPro GPR C++ library](https://github.com/gopro/gpr) exists for conversion, it's a command-line tool with complex options that are not user-friendly for those unfamiliar with CLIs.

## Solution

**GprProTool** wraps the GPR library in an intuitive, menu-driven TUI application that makes .gpr file conversion accessible to everyone. No command-line expertise required!

## Features

- 📁 **Browse files** - Navigate directories and select .gpr files with ease
- 📊 **View metadata** - Display camera model, dimensions, ISO, exposure, and more
- ⚙️ **Configure conversion** - Choose output format (JPEG/PNG), quality, and options
- 🎯 **Simple interface** - Menu-driven TUI with keyboard navigation
- 🎮 **Vim-style controls** - Use j/k or arrow keys for navigation
- 📸 **Multi-camera support** - Works with files from Fusion, HERO5, HERO6, HERO7, HERO9

## Quick Start

### Prerequisites
- **Rust 1.70 or later** (install from [rustup.rs](https://rustup.rs))
- **CMake 3.5 or later** (for building the GPR C++ library)
- **C++ compiler** (Clang on macOS, GCC on Linux, MSVC on Windows)

### Installation
```bash
git clone https://github.com/yourusername/gprprotool.git
cd gprprotool

# The GoPro GPR library is included as a git submodule
# It will be automatically built by the build.rs script

cargo build --release
```

The build process will:
1. Automatically compile the GoPro GPR C++ library using CMake
2. Link it with the Rust application
3. Create the final binary at `target/release/gprprotool`

### Run
```bash
cargo run
# or
./target/release/gprprotool
```

## Acknowledgments

- [GoPro GPR Library](https://github.com/gopro/gpr) - The underlying conversion library
- [Ratatui](https://github.com/ratatui/ratatui) - Excellent TUI framework
