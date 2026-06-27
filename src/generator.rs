//! # Puzzle Generator
//!
//! Generates valid Sudoku puzzles with a unique solution.
//! Algorithm: generate a complete solved board, then remove cells one by one
//! while verifying the puzzle still has exactly one solution.

use crate::solver::{fill_board, shuffle, solve_all};

/// Difficulty levels control how many cells are removed from the solved board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Difficulty {
    Easy,      // Remove ~35-40 cells (leaves 41-46 clues)
    Medium,    // Remove ~45-50 cells (leaves 31-36 clues)
    Hard,      // Remove ~52-57 cells (leaves 24-29 clues)
    Expert,    // Remove ~58-62 cells (leaves 19-23 clues)
}

impl Difficulty {
    /// Number of cells to remove for each difficulty.
    pub fn holes(&self) -> usize {
        match self {
            Difficulty::Easy => 40,
            Difficulty::Medium => 50,
            Difficulty::Hard => 57,
            Difficulty::Expert => 62,
        }
    }
}

impl From<Difficulty> for u8 {
    fn from(d: Difficulty) -> u8 {
        match d {
            Difficulty::Easy => 0,
            Difficulty::Medium => 1,
            Difficulty::Hard => 2,
            Difficulty::Expert => 3,
        }
    }
}

impl From<u8> for Difficulty {
    fn from(id: u8) -> Difficulty {
        match id {
            0 => Difficulty::Easy,
            1 => Difficulty::Medium,
            2 => Difficulty::Hard,
            3 => Difficulty::Expert,
            _ => Difficulty::Medium,
        }
    }
}

/// Generate a new puzzle with the given difficulty.
/// Returns `(puzzle, solution)` where `puzzle` has 0 for empty cells and
/// `solution` is the complete solved board.
pub fn generate(difficulty: Difficulty) -> ([u8; 81], [u8; 81]) {
    // Step 1: Generate a valid completed board using backtracking with randomization
    let mut solution = [0u8; 81];
    if !fill_board(&mut solution) {
        // Extremely rare — fall back to a known valid board
        solution = [
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
    }

    // Step 2: Copy the solution for reference
    let mut puzzle = solution;

    // Step 3: Remove cells while maintaining unique solvability
    let num_holes = difficulty.holes();
    remove_cells(&mut puzzle, num_holes);

    (puzzle, solution)
}

/// Remove `num_holes` cells from the board while ensuring unique solvability.
/// Uses multi-pass retry with re-shuffling to maximize the number of removed cells.
fn remove_cells(puzzle: &mut [u8; 81], num_holes: usize) {
    let mut removed = 0usize;

    while removed < num_holes {
        let mut indices: Vec<usize> = (0..81)
            .filter(|&i| puzzle[i] != 0)
            .collect();

        if indices.is_empty() {
            break;
        }

        shuffle(&mut indices);
        let before = removed;

        for &idx in &indices {
            if removed >= num_holes {
                return;
            }

            let original = puzzle[idx];
            puzzle[idx] = 0;

            let test_board = *puzzle;
            let sol_count = solve_all(&test_board, 2);

            if sol_count != 1 {
                puzzle[idx] = original;
            } else {
                removed += 1;
            }
        }

        if removed == before {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::solve_all;

    #[test]
    fn test_generate_easy() {
        let (puzzle, solution) = generate(Difficulty::Easy);
        // Solution should be complete
        for &v in &solution {
            assert_ne!(v, 0);
        }
        // Puzzle should have some holes
        let holes: usize = puzzle.iter().filter(|&&v| v == 0).count();
        assert!(holes > 30 && holes <= 45, "Easy should have 31-45 holes, got {holes}");

        // Puzzle cells that are non-zero must match solution
        for i in 0..81 {
            if puzzle[i] != 0 {
                assert_eq!(puzzle[i], solution[i]);
            }
        }
    }

    #[test]
    fn test_generate_all_difficulties() {
        for diff in [
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Hard,
            Difficulty::Expert,
        ] {
            let (puzzle, solution) = generate(diff);
            // Verify each puzzle has a unique solution
            let count = solve_all(&puzzle, 2);
            assert_eq!(count, 1, "Puzzle with difficulty {:?} should have exactly one solution", diff);

            // All given clues must match the solution
            for i in 0..81 {
                if puzzle[i] != 0 {
                    assert_eq!(puzzle[i], solution[i]);
                }
            }
        }
    }
}

