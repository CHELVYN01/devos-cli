# 🚀 DevOS - Odoo Developer Operations System

**DevOS** is a blazing-fast CLI tool built in Rust for managing multiple Odoo development environments with ease. Say goodbye to slow VS Code launches and hello to instant project switching!

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Odoo](https://img.shields.io/badge/Odoo-714B67?style=for-the-badge&logo=odoo&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)

## ✨ Features

- ⚡ **Lightning Fast** - Start Odoo projects in under 1 second
- 🧹 **Auto Cache Cleaning** - No more stale CSS/JS issues
- 🐛 **Debug Mode** - Seamless VS Code debugging integration
- 🔧 **Runtime Arguments** - Override configs on-the-fly without editing files
- 📦 **Multi-Project Management** - Switch between projects instantly
- 🪶 **Lightweight** - Single ~5MB executable, no dependencies
- 🎯 **Native Terminal** - Full terminal control with scrolling, searching, and copy-paste

## 🎯 Why DevOS?

| Without DevOS | With DevOS |
|---------------|------------|
| Open VS Code → Select Project → Click Run → Wait | `devos run mesa` → Done! |
| ~10 seconds startup | < 1 second |
| Limited terminal control | Full native terminal |
| Manual cache clearing | `--clean` flag auto-clears |
| Edit config files for testing | Runtime args: `--dev=xml` |

## 📦 Installation

### Option 1: Download Pre-built Binary (Recommended) ⚡

**No Rust installation required!**

1. **Download the latest release**
   - Go to [Releases](https://github.com/YOUR_USERNAME/devos-cli/releases)
   - Download `devos-windows-x64.zip`
   - Extract the ZIP file

2. **Install to system**
   ```powershell
   # Create installation directory
   New-Item -ItemType Directory -Force -Path "C:\devos"
   
   # Copy files (adjust path to your download location)
   Copy-Item "Downloads\devos.exe" "C:\devos\devos.exe"
   Copy-Item "Downloads\projects.json" "C:\devos\projects.json"
   
   # Add to PATH (run as Administrator)
   $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
   [Environment]::SetEnvironmentVariable("Path", "$currentPath;C:\devos", "User")
   ```

3. **Restart your terminal** and verify:
   ```bash
   devos --help
   ```

### Option 2: Build from Source (For Developers)

**Prerequisites:**
- **Rust** (install from https://rustup.rs)
- **Git**

**Steps:**

1. **Clone the repository**
   ```bash
   git clone https://github.com/YOUR_USERNAME/devos-cli.git
   cd devos-cli
   ```

2. **Build the release binary**
   ```bash
   cargo build --release
   ```

3. **Install to system**
   ```powershell
   # Create installation directory
   New-Item -ItemType Directory -Force -Path "C:\devos"
   
   # Copy executable
   Copy-Item "target\release\devos.exe" "C:\devos\devos.exe"
   
   # Copy config template
   Copy-Item "projects.json" "C:\devos\projects.json"
   
   # Add to PATH (run as Administrator)
   $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
   [Environment]::SetEnvironmentVariable("Path", "$currentPath;C:\devos", "User")
   ```

4. **Restart your terminal** and verify:
   ```bash
   devos --help
   ```

## ⚙️ Configuration

Edit `C:\devos\projects.json` to add your Odoo projects:

```json
[
  {
    "name": "My Odoo Project",
    "python": "D:/odoo/python/python.exe",
    "odoo_bin": "D:/odoo/server/odoo-bin",
    "config_file": "D:/projects/my-project/odoo.conf",
    "args": ["-d", "my_database", "-u", "my_module", "--dev=all"],
    "work_dir": "D:/projects/my-project"
  }
]
```

**Quick Edit:**
```bash
devos edit
```
This opens `projects.json` in VS Code automatically.

## 🎮 Usage

### Basic Commands

```bash
# List all available projects
devos list

# Start a project
devos run mesa

# Start with cache cleaning
devos run mesa --clean

# Start in debug mode (VS Code debugging)
devos run mesa --debug

# Combine flags
devos run mesa --clean --debug

# Add runtime arguments
devos run mesa --dev=xml --limit-time-real=99999

# Edit configuration
devos edit
```

### Debug Mode with VS Code

1. **Start DevOS in debug mode:**
   ```bash
   devos run mesa --debug
   ```

2. **In VS Code**, add this to `.vscode/launch.json`:
   ```json
   {
       "name": "Attach to Odoo (DevOS)",
       "type": "debugpy",
       "request": "attach",
       "connect": {
           "host": "localhost",
           "port": 5678
       },
       "justMyCode": false
   }
   ```

3. **Press F5** in VS Code and select "Attach to Odoo (DevOS)"

4. **Set breakpoints** and debug as usual!

### Runtime Arguments

Override or extend default arguments without editing `projects.json`:

```bash
# Change dev mode
devos run mesa --dev=xml

# Add time limits
devos run mesa --limit-time-real=99999 --limit-time-cpu=99999

# Disable specific modules
devos run mesa --load=web,base

# Combine with clean
devos run mesa --clean --dev=reload
```

## 🛠️ Troubleshooting

### "No module named debugpy"

Install debugpy in your Odoo Python environment:
```bash
D:/odoo/python/python.exe -m pip install debugpy
```

### "Could not find 'projects.json'"

Make sure `projects.json` exists in `C:\devos\` or the same directory as `devos.exe`.

### "Access is denied" when building

Close all running instances of DevOS or the old executable:
```bash
taskkill /F /IM devos.exe
cargo clean
cargo build --release
```

### Shutdown errors (daemon threads)

These are cosmetic errors from Python's threading during shutdown. They're harmless. To minimize them, press `Ctrl+C` twice quickly.

## 🏗️ Building from Source

```bash
# Clone repository
git clone https://github.com/YOUR_USERNAME/devos.git
cd devos

# Build debug version (faster compilation)
cargo build

# Build release version (optimized)
cargo build --release

# Run without installing
cargo run -- list
cargo run -- run mesa
```

## 📝 Project Structure

```
devos/
├── src/
│   └── main.rs          # Main CLI application
├── Cargo.toml           # Rust dependencies
├── projects.json        # Project configurations
└── README.md           # This file
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for maximum performance
- CLI parsing powered by [clap](https://github.com/clap-rs/clap)
- Async runtime by [tokio](https://tokio.rs/)
- Designed for [Odoo](https://www.odoo.com/) developers

## 📞 Support

If you encounter any issues or have questions:
- Open an issue on GitHub
- Check the [Troubleshooting](#-troubleshooting) section

---

**Made with ❤️ for Odoo developers who value speed and efficiency**
