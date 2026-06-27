# 数独 · Sudoku

A fully offline, cross-platform Sudoku game built with [Rust](https://www.rust-lang.org/) and [egui](https://github.com/emilk/egui).

![screenshot](https://github.com/user-attachments/assets/placeholder)

## Features

- **4 difficulty levels** — Easy (40 holes), Medium (50), Hard (57), Expert (62)
- **Unique solution guaranteed** — Every puzzle is verified to have exactly one solution using a backtracking solver
- **Pencil marks** — Toggle annotation mode and mark candidates manually, or enable auto-pencil-marks
- **Undo / Redo** — Full move history (up to 500 steps) with Ctrl+Z / Ctrl+Y
- **Conflict highlighting** — Duplicate values in rows, columns, or 3×3 boxes are highlighted in red
- **Hint system** — Reveal the correct value for any empty cell
- **Timer** — Track your solving time
- **Dark & Light themes** — Toggle between dark and light color schemes (persisted across sessions)
- **Keyboard navigation** — Arrow keys to move, digit keys to fill, Backspace/Delete to clear
- **Game persistence** — Your current game and theme preference are saved automatically
- **Offline** — No network connection required

## Screenshots

<! -- TODO: Add actual screenshots -->
| Dark Theme | Light Theme |
|---|---|
| _screenshot coming soon_ | _screenshot coming soon_ |

## Quick Start

### Prerequisites

- Rust toolchain (stable): <https://rustup.rs/>
- A CJK font installed on your system (for Chinese UI text rendering)

### Build & Run

```bash
git clone https://github.com/YOUR_USERNAME/sudoku.git
cd sudoku
cargo run --release
```

The `--release` flag enables LTO (Link-Time Optimization) for better performance.

## Architecture

```
src/
├── main.rs        # Entry point: window setup, font loading
├── lib.rs         # Crate root, module declarations
├── game.rs        # Core game state, cell status, undo/redo, conflict detection
├── generator.rs   # Puzzle generation with uniqueness verification
├── i18n.rs        # User-facing string resources (Chinese, default)
├── solver.rs      # Backtracking solver (MRV heuristic), solution counter
└── ui.rs          # egui UI: grid rendering, controls, themes, keyboard input
```

### Generation pipeline

1. **`fill_board()`** — Randomised backtracking fills an empty board with a valid solution
2. **`remove_cells()`** — Iteratively removes cells while checking uniqueness via `solve_all()`; uses multi-pass retry to maximise hole count
3. **`solve_all(limit=2)`** — Counts solutions up to 2 (early exit when a second solution is found)

The solver uses the **Minimum Remaining Values (MRV)** heuristic to select the most constrained empty cell first, reducing the search tree for hard puzzles.

## Difficulty levels

| Level | Holes | Clues | Target time (casual) |
|---|---|---|---|
| Easy | 40 | 41 | ~5 min |
| Medium | 50 | 31 | ~10 min |
| Hard | 57 | 24 | ~20 min |
| Expert | 62 | 19 | ~30+ min |

## Tech Stack

| Component | Library |
|---|---|
| GUI framework | [egui](https://github.com/emilk/egui) / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) |
| Persistence | eframe storage (serde) |
| RNG | [rand](https://crates.io/crates/rand) |
| Serialisation | [serde](https://serde.rs/) |

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License — see the [LICENSE-MIT](LICENSE-MIT) file for details.
