//! # 数独 · Sudoku
//! A fully offline, cross-platform Sudoku game built with Rust and egui.
//!
//! ## Modules
//! - `game` — Core game state: board representation, cell status, pencil marks, undo/redo stack.
//! - `generator` — Puzzle generation with backtracking solver; difficulty-based hole removal.
//! - `solver` — Backtracking Sudoku solver that finds all solutions (for uniqueness check).
//! - `ui` — egui UI: grid rendering, controls, theme switching, timer, status bar.

pub mod game;
pub mod generator;
pub mod solver;
pub mod ui;
