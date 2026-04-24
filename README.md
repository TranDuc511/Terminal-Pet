# 🐱 Terminal Pet

A virtual pet caring game that runs entirely in your terminal — built with Rust.

Adopt a cat, keep it happy by feeding, petting, and playing with it, and watch your **Bond** grow over time. Neglect your cat and the bond will slowly fade. Your pet's state is automatically saved, and bond decays even while the app is closed — so check in regularly!

```
      /\_/\
     ( o.o )
      > ^ <
     /     \
    /       \
   /   | |   \
  (___(|_|_)__)~
```

---

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Running the App](#running-the-app)
- [How to Play](#how-to-play)
- [Color Themes](#color-themes)
- [Bond & Decay System](#bond--decay-system)
- [Save System](#save-system)
- [Project Structure](#project-structure)
- [Building from Source](#building-from-source)

---

## Features

- 🐱 **Animated ASCII cat** — your cat reacts visually to every interaction
- ❤️ **Bond system** — build a bond through feeding, patting, and playing
- 📉 **Real-time decay** — bond and happiness decrease over time, even offline
- 🍖 **Interaction cooldowns** — each action has a realistic cooldown timer
- 🎨 **5 color themes** — Red, Blue, Green, Pink, Yellow (RGB colors)
- 💾 **Auto-save & Multi-Pet** — game state is saved every 60 seconds, on quit, and when returning to the menu. Manage multiple pets and delete saves directly from the menu!
- 🌙 **Offline catch-up** — the app calculates how long you were away and applies decay accordingly
- 📅 **Streak tracker** — tracks the real-world days since your pet was adopted, displayed right in the title bar
- 📺 **Full TUI** — beautiful terminal UI with perfectly centered ASCII art, gauges, panels, and a message log

---

## Requirements

| Tool | Minimum Version | Notes |
|------|----------------|-------|
| [Rust](https://rustup.rs) | 1.75+ | Includes `cargo` |
| Terminal | Any modern terminal | Windows Terminal recommended for full RGB color support |

> **Windows users:** Use **Windows Terminal** (not `cmd.exe`) for the best experience. Classic `cmd.exe` has limited color support.

---

## Installation

### Step 1 — Install Rust

If you don't have Rust installed, go to **https://rustup.rs** and follow the instructions for your operating system.

After installation, open a **new terminal window** and verify it worked:

```sh
cargo --version
# Expected output: cargo 1.xx.x (...)
```

### Step 2 — Clone the repository

```sh
git clone https://github.com/your-username/terminal-pet.git
cd terminal-pet
```

> Or simply open the folder if you already have the source code on your machine.

---

## Running the App

### Default (uses last saved theme)

```sh
cargo run
```

### With a specific color theme

```sh
cargo run -- --color pink
cargo run -- --color red
cargo run -- --color blue
cargo run -- --color green
cargo run -- --color yellow
```

### Build a release binary (faster performance)

```sh
cargo build --release
./target/release/terminal-pet
```

---

## How to Play

### First Launch

On your very first run, you will be greeted with a **Pet Selection Menu**. You can choose your preferred companion (currently, only Cat is available, with Dog and Turtle coming soon!). Use `Up` and `Down` to select, and `Enter` to confirm.

After selecting your pet, you will be prompted to **name it**. Type a name and press `Enter`. Your companion is now alive!

```
┌─ 🐾 New Pet ──────────────────────────────┐
│                                           │
│  Welcome to Terminal Pet!  🐱             │
│                                           │
│  What would you like to name your cat?   │
│                                           │
│  > Whiskers█                              │
│                                           │
│  [Enter] Confirm   [Backspace] Delete     │
└───────────────────────────────────────────┘
```

Press `Esc` without typing to use the default name **"Whiskers"**.

### Keybindings

| Key | Action | Cooldown |
|-----|--------|----------|
| `F` | **Feed** your pet 🍖 | 10 seconds |
| `P` | **Head-pat** your pet 🤚 | 5 seconds |
| `Y` | **Play** with a toy 🧶 | 15 seconds |
| `T` | **Cycle** to the next color theme 🎨 | — |
| `M` | **Menu** exit to selection (New pet) 🐾| — |
| `H` | **Toggle** the Help overlay | — |
| `Q` | **Quit** and save | — |

### Understanding the UI

```
┌────────────────────────────────────────────────────────┐
│  🐱 Terminal Pet — "Whiskers" (Day 1)      Theme: 🔵   │  ← Title bar
├──────────────────────────────────────────────────┤
│                                                  │
│                    /\_/\                         │
│                   ( o.o )                        │  ← Your cat (animated)
│                    > ^ <      Mood shown here    │
│                   /     \                        │
│                  /       \                       │
│                 /   | |   \                      │
│                (___(|_|_)__)~                    │
│                                                  │
├──────────────┬──────────────┬────────────────────┤
│ ❤️  Bond  75% │ 🍖 Fullness 60%│ ✨ Joy  80%       │  ← Stat gauges
├──────────────┴──────────────┴────────────────────┤
│ [F] Feed │ [P] Pat │ [Y] Play │ [T] Theme │ [M] Menu │ [H] Help │ [Q] Quit │  ← Action bar
├──────────────────────────────────────────────────┤
│  ▸ You gently patted Whiskers! Bond +5           │
│  ▸ Whiskers purrs contentedly~                   │  ← Message log
│  ▸ Welcome back! Whiskers is happy to see you~   │
└──────────────────────────────────────────────────┘
```

### Cat Moods

Your cat's current mood is shown at the top of the pet panel:

| Mood | Trigger |
|------|---------|
| ✨ Ecstatic | Bond ≥ 85% |
| 😊 Happy | Bond ≥ 60%, not very hungry |
| 😌 Content | Bond ≥ 35%, not too hungry |
| 😿 Hungry | Hunger ≥ 70% |
| 😿 Sad | Happiness < 30% |
| 🌧️ Lonely | Bond < 20% |

### Interaction Effects

| Action | Bond | Fullness | Joy |
|--------|------|----------|-----|
| 🍖 Feed | +3 | +30 | +5 |
| 🤚 Head Pat | +5 | — | +15 |
| 🧶 Play | +8 | −10 (playing makes them hungry!) | +25 |

---

## Color Themes

Switch themes at any time with the `T` key, or set one at launch with `--color <name>`.

| Flag | Theme | Description |
|------|-------|-------------|
| `--color red` | 🔴 Red | Warm crimson and rose tones |
| `--color blue` | 🔵 Blue | Cool sky and ocean blues |
| `--color green` | 🟢 Green | Fresh mint and emerald greens |
| `--color pink` | 🩷 Pink | Soft magenta and blush pinks |
| `--color yellow` | 🟡 Yellow | Bright golden and amber yellows |

> Your chosen theme is saved and restored automatically on the next launch.

---

## Bond & Decay System

Bond is the core stat of the game. Here's how it works:

### While the app is running
- Bond decays at **−0.5 per real-world minute**
- Hunger increases slowly over time (your cat gets hungry naturally)
- Happiness decays if Bond drops below 40%

### While the app is closed (offline decay)
When you relaunch the app, it calculates how long you were away and applies:

| Stat | Offline Rate | Maximum Penalty |
|------|-------------|-----------------|
| Bond | −0.5 / min | −50 max |
| Fullness | −0.3 / min | −40 max |
| Joy | −0.2 / min | −30 max |

> **Example:** If you were away for 2 hours (120 min), Bond drops by 50 (capped), Hunger rises by 36, and Joy drops by 24.

The penalties are **capped** so your pet can never reach absolute zero from a single long absence — but they'll definitely be unhappy!

---

## Save System

Game state is automatically saved to a JSON file named after your pet (e.g., `save_Whiskers.json`):

| Platform | Save Location |
|----------|--------------|
| Windows | `%APPDATA%\terminal-pet\save_{name}.json` |
| macOS | `~/Library/Application Support/terminal-pet/save_{name}.json` |
| Linux | `~/.local/share/terminal-pet/save_{name}.json` |

The save file is human-readable JSON:

```json
{
  "version": "0.1.0",
  "pet": {
    "name": "Whiskers",
    "species": "Cat",
    "bond": 75.0,
    "hunger": 28.0,
    "happiness": 83.0,
    "last_interaction": "2026-04-23T13:45:00Z",
    "created_at": "2026-04-20T10:00:00Z"
  },
  "theme": "Blue",
  "saved_at": "2026-04-23T13:50:00Z"
}
```

**Auto-save triggers:**
- Every 60 seconds while the app is running
- When you press `Q` to quit
- When you press `M` to return to the menu
- On clean exit

To **delete a pet**, highlight it in the "Load saved" menu and press the `Delete` or `Backspace` key. Alternatively, you can manually delete its corresponding save file from the directory above.

---

## Project Structure

```
Terminal Pet/
│
├── Cargo.toml               # Project manifest: dependencies and metadata
│
└── src/
    ├── main.rs              # Entry point: terminal setup, arg parsing, mounts the app
    ├── app/                 # Application state and event loop
    │   ├── mod.rs           # Central app state and game loop
    │   └── handlers.rs      # User input handling and business logic
    ├── core/                # Core domain models and systems
    │   ├── pet.rs           # Pet logic: stats, interactions, decay, mood
    │   ├── event.rs         # Background thread for input and tick events
    │   ├── theme.rs         # Color themes with RGB palettes
    │   ├── save.rs          # JSON persistence and offline decay
    │   └── ascii_art.rs     # ASCII art frames and animations
    └── ui/                  # Ratatui rendering components
        ├── mod.rs           # Main UI rendering pipeline
        ├── home.rs          # Gameplay screen layout (gauges, pet, log)
        ├── menus.rs         # Pet selection and save management menus
        ├── help.rs          # Help and keyboard shortcut overlay
        └── utils.rs         # Shared rendering utilities
```

### What each module does

| Module | Responsibility |
|--------|---------------|
| `main.rs` | Terminal setup (raw mode, alternate screen), panic hook, CLI arg parsing, clean teardown |
| `app/` | Owns all state (`Pet`, `Theme`, `Screen`, log); runs the main event → update → render loop |
| `core/` | Defines core game mechanics, background event handling, themes, ASCII art, and saving/loading |
| `ui/` | Modular UI components that take the application state and render it to the terminal screen |

---

## Building from Source

```sh
# Debug build (fast to compile, includes debug info)
cargo build

# Release build (slower to compile, much faster to run)
cargo build --release

# Run tests
cargo test

# Check for common issues and style warnings
cargo clippy

# Check that the code compiles without producing a binary
cargo check
```

---

## Troubleshooting

**Colors look wrong or missing?**
- Make sure you are using **Windows Terminal**, **iTerm2**, or another modern terminal emulator that supports 24-bit (TrueColor) ANSI colors.
- Classic `cmd.exe` on Windows does not support RGB colors.

**The terminal is messed up after closing?**
- This can happen if the process was force-killed. Run `reset` (Linux/macOS) or close and reopen the terminal.
- The app installs a panic hook and a clean-exit handler specifically to prevent this, but hard kills bypass them.

**`cargo` command not found?**
- Rust is not installed, or the installation directory is not in your `PATH`.
- Install from **https://rustup.rs**, then open a fresh terminal window.

**Windows linker errors (`link.exe` failed)?**
- If you experience linking issues on Windows (especially when missing the C++ build tools), switch to the GNU toolchain:
  ```sh
  rustup toolchain install stable-x86_64-pc-windows-gnu
  rustup default stable-x86_64-pc-windows-gnu
  ```

**My pet's bond is already very low on startup?**
- This is the offline decay system working as intended! Your cat missed you 💔
- Feed and play with it right away to bring the bond back up.

---

## Changelog

### Released (v0.1.3)
- **Feature:** Added a real-time Day count (streak) tracker displayed in the title bar and Load Saved menu.
- **Feature:** Added the ability to delete save files directly from the "Load Saved" menu using the `Delete` or `Backspace` key.
- **Fix:** Smoothed out ASCII animations by dynamically adjusting animation speed based on the pet's state (e.g., sleeping is slower, eating is faster).
- **Fix:** Fixed an issue where the ASCII art would appear jagged or misaligned by perfectly centering it visually within the terminal panel.

### Released (v0.1.2)
- **Feature:** Added support for adopting multiple pets! Save files are now named after the pet (e.g., `save_Whiskers.json`).
- **Feature:** Added a "Load saved" option to the Pet Selection Menu.
- **Feature:** The game now automatically auto-saves when you press `M` to return to the menu.
- **Feature:** Automatically loads the most recently played pet on startup.

### Released (v0.1.1)
- **Feature:** Added a Pet Selection Menu on first launch to choose between Cat, Dog (soon), and Turtle (soon).
- **Feature:** Added `[M]` Menu keybinding to exit to the selection menu and create a new pet.
- **Fix:** Fixed an issue on Windows where keys would double-type due to both key press and release events being registered.

---

## License

This project is open source. Feel free to modify it, add more pet types, or extend the game mechanics!

---

*Made with 🦀 Rust and lots of ❤️*
