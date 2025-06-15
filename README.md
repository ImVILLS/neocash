# NeoCASH (New Era of CASH)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![AUR](https://img.shields.io/aur/version/neocash)](https://aur.archlinux.org/packages/neocash)

A modern command shell written in Rust, successor to the CASH project (Cash is Absolute SHell)

## 📦 Installation

### Arch Linux (AUR)
```bash
# for yay
yay -S neocash

# for paru
paru -S neocash
```

### From Source
```bash
cargo install --git https://github.com/ImVILLS/neocash
```

## ✨ Current Features
- Basic shell with command support
- Command history
- Customizable prompt
- Command autocompletion foundation

## 🚧 Roadmap
- [ ] Path/file autocompletion system
- [ ] Bash/zsh script support
- [ ] Custom scripting language CAS (CASH Absolute Script)
- [ ] Plugin supporting

## 🛠 Building
```bash
cargo build --release
strip target/release/neocash  # To reduce binary size
```

## 🤝 Contributing
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Submit a Pull Request

## 📜 License
MIT © 2025 [ImVILLS](https://github.com/ImVILLS)
