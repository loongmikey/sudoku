//! # Game State
//!
//! Manages the current board state, including:
//! - The original puzzle (fixed cells) and user-filled values.
//! - Pencil marks (candidate annotations).
//! - Undo/redo history with a configurable depth limit.
//! - Timer tracking elapsed time.

use serde::{Deserialize, Serialize};

pub const SIZE: usize = 9;
pub const BOARD: usize = SIZE * SIZE;
pub const BOX: usize = 3;
const UNDO_LIMIT: usize = 500;

/// A single cell's pencil marks (candidate annotations) as a bitmask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct PencilMarks(u32);

impl PencilMarks {
    pub fn contains(&self, d: u8) -> bool {
        (1..=9).contains(&d) && self.0 & (1u32 << ((d - 1) as u32)) != 0
    }

    pub fn with_digit(mut self, d: u8) -> Self {
        if (1..=9).contains(&d) {
            self.0 |= 1u32 << ((d - 1) as u32);
        }
        self
    }

    pub fn remove(mut self, d: u8) -> Self {
        if (1..=9).contains(&d) {
            self.0 &= !(1u32 << ((d - 1) as u32));
        }
        self
    }

    /// Return a sorted list of digits present in the marks.
    pub fn digits(&self) -> ([u8; 9], usize) {
        let mut arr = [0u8; 9];
        let mut count = 0;
        for d in 1..=9u8 {
            if self.contains(d) {
                arr[count] = d;
                count += 1;
            }
        }
        (arr, count)
    }
}

/// Status of a single cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CellStatus {
    Fixed(u8),
    Filled(u8),
    Empty,
}

impl CellStatus {
    pub fn value(&self) -> Option<u8> {
        match self {
            CellStatus::Fixed(v) | CellStatus::Filled(v) => Some(*v),
            CellStatus::Empty => None,
        }
    }

    pub fn is_fixed(&self) -> bool {
        matches!(self, CellStatus::Fixed(_))
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, CellStatus::Empty)
    }
}

/// A snapshot of the board at a point in time (for undo/redo).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardSnapshot {
    pub cells: Vec<CellStatus>,
    pub pencils: Vec<PencilMarks>,
    pub elapsed_secs: f64,
}

impl Default for BoardSnapshot {
    fn default() -> Self {
        BoardSnapshot {
            cells: vec![CellStatus::Empty; BOARD],
            pencils: vec![PencilMarks::default(); BOARD],
            elapsed_secs: 0.0,
        }
    }
}

/// The complete game state, persisted across sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub puzzle: Vec<CellStatus>,
    pub current: BoardSnapshot,
    pub solution: Vec<u8>,
    pub difficulty_label: String,
    pub difficulty: u8,
    pub elapsed_secs: f64,
    pub timer_running: bool,
    undo_stack: Vec<BoardSnapshot>,
    redo_stack: Vec<BoardSnapshot>,
}

impl GameState {
    pub fn new(puzzle: [u8; 81], solution: [u8; 81], difficulty_label: String, difficulty: u8) -> Self {
        let mut cells = vec![CellStatus::Empty; BOARD];
        for (i, &v) in puzzle.iter().enumerate() {
            if v != 0 {
                cells[i] = CellStatus::Fixed(v);
            }
        }

        GameState {
            puzzle: cells,
            current: BoardSnapshot {
                cells: vec![CellStatus::Empty; BOARD],
                pencils: vec![PencilMarks::default(); BOARD],
                elapsed_secs: 0.0,
            },
            solution: solution.to_vec(),
            difficulty_label,
            difficulty,
            elapsed_secs: 0.0,
            timer_running: true,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub(crate) fn push_undo(&mut self) {
        let snapshot = BoardSnapshot {
            cells: self.current.cells.clone(),
            pencils: self.current.pencils.clone(),
            elapsed_secs: self.elapsed_secs,
        };
        self.undo_stack.push(snapshot);
        if self.undo_stack.len() > UNDO_LIMIT {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> bool {
        if self.undo_stack.is_empty() {
            return false;
        }
        let snapshot = BoardSnapshot {
            cells: self.current.cells.clone(),
            pencils: self.current.pencils.clone(),
            elapsed_secs: self.elapsed_secs,
        };
        self.redo_stack.push(snapshot);

        let prev = self.undo_stack.pop().unwrap();
        self.current.cells = prev.cells;
        self.current.pencils = prev.pencils;
        self.elapsed_secs = prev.elapsed_secs;
        true
    }

    pub fn redo(&mut self) -> bool {
        if self.redo_stack.is_empty() {
            return false;
        }
        let snapshot = BoardSnapshot {
            cells: self.current.cells.clone(),
            pencils: self.current.pencils.clone(),
            elapsed_secs: self.elapsed_secs,
        };
        self.undo_stack.push(snapshot);

        let next = self.redo_stack.pop().unwrap();
        self.current.cells = next.cells;
        self.current.pencils = next.pencils;
        self.elapsed_secs = next.elapsed_secs;
        true
    }

    pub fn is_complete(&self) -> bool {
        for (i, cell) in self.puzzle.iter().enumerate() {
            if cell.is_fixed() {
                continue;
            }
            let val = match self.current.cells[i] {
                CellStatus::Filled(v) => v,
                CellStatus::Empty => return false,
                CellStatus::Fixed(_) => return false,
            };
            if val != self.solution[i] {
                return false;
            }
        }
        true
    }

    fn all_values_grid(&self) -> [u8; BOARD] {
        let mut grid = [0u8; BOARD];
        for (i, cell) in self.puzzle.iter().enumerate() {
            if let Some(v) = cell.value() {
                grid[i] = v;
            }
        }
        for (i, cell) in self.current.cells.iter().enumerate() {
            if let Some(v) = cell.value() {
                if !self.puzzle[i].is_fixed() {
                    grid[i] = v;
                }
            }
        }
        grid
    }

    fn find_duplicates(indices: &[usize], grid: &[u8; BOARD]) -> u128 {
        let mut result: u128 = 0;
        let mut counts = [0u8; 10];
        for &idx in indices {
            let v = grid[idx];
            if v != 0 {
                counts[v as usize] += 1;
            }
        }
        for &idx in indices {
            let v = grid[idx];
            if v != 0 && counts[v as usize] > 1 {
                result |= 1u128 << idx;
            }
        }
        result
    }

    /// Get the list of user cells that have conflicts (for highlighting).
    ///
    /// Uses a u128 bitmask to efficiently track which indices have duplicates.
    /// A 9x9 board has 81 cells, so u128 provides enough bits (128 > 81).
    /// Each bit `i` set means cell `i` is part of a duplicate group.
    pub fn conflict_cells(&self) -> [bool; BOARD] {
        let grid = self.all_values_grid();
        let mut result: u128 = 0;

        for r in 0..SIZE {
            let indices: [usize; SIZE] = core::array::from_fn(|c| r * SIZE + c);
            result |= Self::find_duplicates(&indices, &grid);
        }

        for c in 0..SIZE {
            let indices: [usize; SIZE] = core::array::from_fn(|r| r * SIZE + c);
            result |= Self::find_duplicates(&indices, &grid);
        }

        for br in (0..SIZE).step_by(BOX) {
            for bc in (0..SIZE).step_by(BOX) {
                let mut indices = [0usize; SIZE];
                let mut i = 0;
                for r in br..br + BOX {
                    for c in bc..bc + BOX {
                        indices[i] = r * SIZE + c;
                        i += 1;
                    }
                }
                result |= Self::find_duplicates(&indices, &grid);
            }
        }

        let mut out = [false; BOARD];
        for (i, slot) in out.iter_mut().enumerate() {
            // Extract bit `i` from the u128 mask: 1 if set, 0 otherwise.
            *slot = (result >> i) & 1 == 1 && !self.puzzle[i].is_fixed();
        }
        out
    }

    pub fn can_hint(&self, idx: usize) -> bool {
        !self.puzzle[idx].is_fixed() && matches!(self.current.cells[idx], CellStatus::Empty)
    }

    pub fn apply_hint(&mut self, idx: usize) {
        if let Some(&v) = self.solution.get(idx).filter(|&&v| v != 0) {
            self.push_undo();
            self.current.cells[idx] = CellStatus::Filled(v);
            self.current.pencils[idx] = PencilMarks::default();
        }
    }

    pub fn auto_pencil_marks(&mut self) {
        for i in 0..BOARD {
            if matches!(self.current.cells[i], CellStatus::Empty) {
                let mut marks = PencilMarks::default();
                let row = i / SIZE;
                let col = i % SIZE;

                let mut used = [false; 10];
                for c in 0..SIZE {
                    let ri = row * SIZE + c;
                    if let Some(v) = self.puzzle[ri].value().or_else(|| self.current.cells[ri].value()) {
                        used[v as usize] = true;
                    }
                    let ci = c * SIZE + col;
                    if let Some(v) = self.puzzle[ci].value().or_else(|| self.current.cells[ci].value()) {
                        used[v as usize] = true;
                    }
                }
                let br = (row / BOX) * BOX;
                let bc = (col / BOX) * BOX;
                for r in br..br + BOX {
                    for c in bc..bc + BOX {
                        let bi = r * SIZE + c;
                        if let Some(v) = self.puzzle[bi].value().or_else(|| self.current.cells[bi].value()) {
                            used[v as usize] = true;
                        }
                    }
                }

                for d in 1..=9u8 {
                    if !used[d as usize] {
                        marks = marks.with_digit(d);
                    }
                }

                self.current.pencils[i] = marks;
            }
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Helper: create a GameState with a standard solution (1-9 repeated 9 times)
    /// for simplified testing.
    fn test_game(puzzle: [u8; 81]) -> GameState {
        let solution: [u8; 81] = core::array::from_fn(|i| (i % 9 + 1) as u8);
        GameState::new(puzzle, solution, "Test".to_string(), 0)
    }

    #[test]
    fn test_full_game_flow() {
        // Create a game with the classic Wikipedia puzzle (Easy difficulty)
        let puzzle = [
            5,3,0,0,7,0,0,0,0,
            6,0,0,1,9,5,0,0,0,
            0,9,8,0,0,0,0,6,0,
            8,0,0,0,6,0,0,0,3,
            4,0,0,8,0,3,0,0,1,
            7,0,0,0,2,0,0,0,6,
            0,6,0,0,0,0,2,8,0,
            0,0,0,4,1,9,0,0,5,
            0,0,0,0,8,0,0,7,9,
        ];

        let solution: [u8; 81] = [
            5,3,4,6,7,8,9,1,2,
            6,7,2,1,9,5,3,4,8,
            1,9,8,3,4,2,5,6,7,
            8,5,9,7,6,1,4,2,3,
            4,2,6,8,5,3,7,9,1,
            7,1,3,9,2,4,8,5,6,
            9,6,1,5,3,7,2,8,4,
            2,8,7,4,1,9,6,3,5,
            3,4,5,2,8,6,1,7,9,
        ];
        let mut game = GameState::new(puzzle, solution, "Easy".to_string(), 0);

        // Verify initial state: all puzzle cells are fixed, current is empty
        for i in 0..81 {
            if puzzle[i] != 0 {
                assert!(game.puzzle[i].value().is_some());
                assert_eq!(game.current.cells[i], CellStatus::Empty);
            } else {
                assert!(game.puzzle[i].value().is_none());
                assert_eq!(game.current.cells[i], CellStatus::Empty);
            }
        }

        // Fill a few cells manually (simulating user input)
        game.push_undo();
        game.current.cells[0] = CellStatus::Filled(5);
        game.current.cells[1] = CellStatus::Filled(3);
        game.current.pencils[2] = game.current.pencils[2].with_digit(7);

        // Undo should restore to initial state (all current cells empty)
        assert!(game.undo());
        for i in 0..81 {
            assert_eq!(game.current.cells[i], CellStatus::Empty);
        }

        // Redo should restore the filled cells
        assert!(game.redo());
        assert_eq!(game.current.cells[0], CellStatus::Filled(5));
        assert_eq!(game.current.cells[1], CellStatus::Filled(3));
        assert_eq!(game.current.pencils[2].digits().0[..1], [7]);

        // Fill the rest of the puzzle correctly (simulating solving)
        for i in 0..81 {
            if !game.puzzle[i].is_fixed() && game.current.cells[i].is_empty() {
                let val = game.solution[i];
                assert_ne!(val, 0);
                game.current.cells[i] = CellStatus::Filled(val);
            }
        }

        // Should be complete now
        assert!(game.is_complete());

        // Undo all the way back to initial state
        while game.undo() {}
        assert!(!game.undo(), "Should not be able to undo further");

        // Redo should restore everything
        while game.redo() {}
        assert!(game.is_complete());
    }

    #[test]
    fn test_pencil_marks_after_fill() {
        let mut game = test_game([0u8; 81]);

        // Fill entire row with values 1-9
        for c in 0..9 {
            game.current.cells[c] = CellStatus::Filled((c % 9 + 1) as u8);
        }

        // Auto-pencil-marks should compute empty pencil marks (no candidates left)
        game.auto_pencil_marks();

        for i in 0..9 {
            assert_eq!(game.current.pencils[i].digits().1, 0, "Cell {} pencil marks should be empty", i);
        }
    }

    #[test]
    fn test_conflict_detection_after_fill() {
        let mut game = test_game([0u8; 81]);

        // Fill a row with values 1-9 (cells 0-7 get values 1-8)
        for c in 0..8 {
            game.current.cells[c] = CellStatus::Filled((c % 9 + 1) as u8);
        }
        // Change cell 8 to 5 — duplicates cell 4 which is also 5
        game.current.cells[8] = CellStatus::Filled(5);

        let conflicts = game.conflict_cells();
        assert!(conflicts[4], "Cell 4 should be flagged (value 5)");
        assert!(conflicts[8], "Cell 8 should be flagged (value 5)");
        assert!(!conflicts[0], "Cell 0 should NOT be flagged (value 1 is unique)");
    }

    #[test]
    fn test_elapsed_secs_persistence() {
        let mut game = test_game([0u8; 81]);

        // Push undo first so we can test restoration
        game.push_undo();

        // Simulate time passing
        game.elapsed_secs = 42.5;

        // Undo should restore to saved state (elapsed_secs was 0.0 when pushed)
        assert!(game.undo());
        assert_eq!(game.elapsed_secs, 0.0);

        // Redo should restore the modified state
        assert!(game.redo());
        assert_eq!(game.elapsed_secs, 42.5);
    }

    #[test]
    fn test_undo_limit() {
        let mut game = test_game([0u8; 81]);

        // Fill all cells (more than UNDO_LIMIT)
        for i in 0..BOARD {
            if !game.puzzle[i].value().is_some() && game.current.cells[i].is_empty() {
                game.current.cells[i] = CellStatus::Filled((i % 9 + 1) as u8);
            }
        }

        // Undo until stack is empty, then undo should fail
        while game.undo() {}
        assert!(!game.undo(), "Undo on empty stack should return false");

        // Redo should restore all cells
        for i in 0..BOARD {
            if !game.puzzle[i].value().is_some() && game.current.pencils[i].digits().1 == 0 {
                let expected = (i % 9 + 1) as u8;
                assert_eq!(game.current.cells[i], CellStatus::Filled(expected), "Cell {} should be restored", i);
            }
        }

        // Redo until stack is empty, then redo should fail
        while game.redo() {}
        assert!(!game.redo(), "Redo on empty stack should return false");
    }

    #[test]
    fn test_hint_application() {
        let mut puzzle: [u8; 81] = [0; 81];
        puzzle[..27].copy_from_slice(&[
            5,3,4,6,7,8,9,1,2,
            6,7,2,1,9,5,3,4,8,
            1,9,8,3,4,2,5,6,7,
        ]);
        let solution: [u8; 81] = [
            5,3,4,6,7,8,9,1,2,
            6,7,2,1,9,5,3,4,8,
            1,9,8,3,4,2,5,6,7,
            8,5,9,7,6,1,4,2,3,
            4,2,6,8,5,3,7,9,1,
            7,1,3,9,2,4,8,5,6,
            9,6,1,5,3,7,2,8,4,
            2,8,7,4,1,9,6,3,5,
            3,4,5,2,8,6,1,7,9,
        ];
        let mut game = GameState::new(puzzle, solution, "Easy".to_string(), 0);

        // Apply hint to cell 0 (should be 5)
        game.apply_hint(0);

        assert_eq!(game.current.cells[0], CellStatus::Filled(5));
        assert_eq!(game.current.pencils[0].digits().1, 0, "Pencil marks should be cleared");

        // Undo should restore to empty (not to puzzle)
        assert!(game.undo());
        assert_eq!(game.current.cells[0], CellStatus::Empty);
    }

    #[test]
    fn test_can_hint_edge_cases() {
        let mut game = test_game([0u8; 81]);

        // Empty non-fixed cell should be hintable
        assert!(game.can_hint(0));

        // Fixed cell should not be hintable
        game.puzzle[0] = CellStatus::Fixed(5);
        assert!(!game.can_hint(0));

        // Filled cell should not be hintable
        game.current.cells[1] = CellStatus::Filled(3);
        assert!(!game.can_hint(1));

        // Pencil marks don't affect can_hint (only filled cells do)
        // Cell 2 is still non-fixed and empty, so it's hintable
        game.current.pencils[2].with_digit(7);
        assert!(game.can_hint(2), "Cell with pencil marks should still be hintable");
    }
}
