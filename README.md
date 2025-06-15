# NeoCASH (New Era of CASH)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![AUR](https://img.shields.io/aur/version/neocash)](https://aur.archlinux.org/packages/neocash)

Современная командная оболочка на Rust, наследник проекта CASH (Cash is Absolute SHell)

## 📦 Установка

### Arch Linux (AUR)
```bash
yay -S neocash
```

### Из исходников
```bash
cargo install --git https://github.com/ImVILLS/neocash
```

## ✨ Текущие возможности
- Базовый шелл с поддержкой команд
- История команд
- Кастомизируемый промпт
- Заготовка для автодополнения

## 🚧 Планы развития
- [ ] Система автодополнения путей/файлов
- [ ] Поддержка bash/zsh-скриптов
- [ ] Собственный скриптовый язык CAS (CASH Absolute Script)
- [ ] Цветной вывод и темы

## 🛠 Сборка
```bash
cargo build --release
strip target/release/neocash  # Для уменьшения размера
```

## 🤝 Участие
1. Форкните репозиторий
2. Создайте ветку для фичи (`git checkout -b feature/your-feature`)
3. Сделайте Pull Request

## 📜 Лицензия
MIT © 2025 [ImVILLS](https://github.com/ImVILLS)
