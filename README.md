# ⚙ RustForge — Ultimate Rust Game Optimizer

**Легальный** оптимизатор для игры Rust (Facepunch Studios).  
Только launch options, config файлы и официальные Windows API. **Никаких читов, никакого бана EAC.**

---

## ✨ Возможности

| Функция | Описание |
|---------|----------|
| 🚀 **Launch Options** | Автосборка оптимальной строки `-high -maxMem -gc.buffer` и т.д. |
| 🎨 **Graphics Config** | Запись `client.cfg` с оптимальными настройками (тени, вода, LOD, AA) |
| ⚡ **System Tweaks** | Ultimate Performance Power Plan, отключение Xbox Game Bar |
| 📁 **Профили** | Max FPS / Balanced PVP / High Visibility / Custom |
| 💻 **Автодетект железа** | RAM, CPU, GPU — автоподбор настроек |
| 🔄 **Бэкап/Ресторе** | Создаёт резервную копию перед любым изменением |
| ⚡ **One-Click Boost** | Одна кнопка — лучшие настройки + запуск |

## 📈 Реалистичный прирост FPS

- **Graphics Quality 6→2** — +30–50 FPS
- **Shadows OFF** — +15–30 FPS  
- **Water Quality 3→0** — +10–20 FPS
- **Ultimate Power Plan** — +5–15%
- **Launch Options комплекс** — +2–10 FPS

> ⚠️ Результаты зависят от железа. Rust — CPU-bound игра.

## 🛡️ Безопасность

- ✅ Нет чтения памяти процессов
- ✅ Нет инъекций DLL/кода  
- ✅ Только официальные Windows API (powercfg, registry)
- ✅ Не нарушает EAC (Easy Anti-Cheat)

---

## 🔨 Сборка

### Требования

- **Rust** 1.70+ (`rustup` — https://rustup.rs)
- Windows 10/11 (для полных функций) или Linux

### Локальная сборка

```bash
git clone https://github.com/your-user/rust-forge
cd rust-forge
cargo build --release
# Бинарник: target/release/rust-forge.exe
```

### Патч зависимостей (важно!)

Проект использует `[patch.crates-io]` для egui-winit без лишних зависимостей.
Патченная копия включена в репозиторий в папке `vendor/egui-winit-patched/`.

---

## 🚀 GitHub Actions — Автосборка .exe

Добавь файл `.github/workflows/release.yml` (уже включён в проект).

### Создание релиза:

```bash
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions автоматически:
1. Соберёт `.exe` под Windows x64
2. Создаст GitHub Release
3. Прикрепит `rust-forge.exe` как артефакт

---

## 📂 Структура проекта

```
rust-forge/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # Главный App struct + eframe loop
│   ├── config/          # Структуры данных (serde)
│   └── ui/
│       ├── theme.rs     # Цветовая схема (rust/постапок)
│       ├── widgets.rs   # Переиспользуемые компоненты
│       └── tabs/        # Вкладки интерфейса
│           ├── launch.rs    # Launch Options
│           ├── graphics.rs  # Графические настройки
│           ├── system.rs    # Системные твики
│           ├── profiles.rs  # Профили
│           └── about.rs     # О программе
├── Cargo.toml
└── README.md
```

## ⚙️ Настройки записываются в

| Файл | Расположение |
|------|-------------|
| `client.cfg` | `%APPDATA%/../LocalLow/Facepunch Studios/Rust/cfg/` |
| `localconfig.vdf` | `Steam/userdata/<id>/config/` |
| Конфиг RustForge | `%APPDATA%/RustForge/config.json` |

---

*Сделано с ❤️ для Rust-коммьюнити. Не аффилировано с Facepunch Studios.*
