# NeoCASH (New Era of Community-Adaptive SHells)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![AUR](https://img.shields.io/aur/version/neocash)](https://aur.archlinux.org/packages/neocash)

**NeoCASH** is a modern shell written in Rust.
We are building a **new era** of shells that are **community-driven**: their development, functionality, and future are determined by the active participation, ideas, and contributions of every member of our community. It's a shell that grows with you.

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

## Configuration
By default, the configuration file is located at ~/.config/neocash/ncashrc.
### Example of the contents of the configuration file
```toml
history_size = 1000
history_file = "~/.local/share/ncash/history.txt"

[prompt]
template = "$bold$time $magenta$user$white@$yellow$host$white $reset$status_icon $bold$blue$path $reset> "
path_mode = "short-all"
status_icon_success = "$green✓$reset"
status_icon_error = "$red✗$reset"
show_time = true
show_user = true
show_host = true
default_editor = "nvim"

[colors]
reset = "\u001B[0m"
black = "\u001B[30m"
cyan = "\u001B[36m"
underline = "\u001B[4m"
bg_black = "\u001B[40m"
bg_blue = "\u001B[44m"
bg_cyan = "\u001B[46m"
italic = "\u001B[3m"
yellow = "\u001B[33m"
green = "\u001B[32m"
blue = "\u001B[34m"
magenta = "\u001B[35m"
white = "\u001B[37m"
bg_white = "\u001B[47m"
bg_yellow = "\u001B[43m"
bg_magenta = "\u001B[45m"
bg_red = "\u001B[41m"
bold = "\u001B[1m"
strikethrough = "\u001B[9m"
bg_green = "\u001B[42m"
red = "\u001B[31m"
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
