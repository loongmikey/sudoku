//! # Sudoku Solver & Generator Utilities
//!
//! Provides a recursive backtracking solver and puzzle generation helpers.
//! All public functions are shared between the solver and puzzle generator.

/// Find the index of the first empty cell (value == 0). Returns `None` if full.
#[allow(dead_code)]
pub fn find_empty(board: &[u8; 81]) -> Option<usize> {
    board.iter().position(|&v| v == 0)
}

/// Find the empty cell with the fewest valid candidates (MRV heuristic).
/// Returns `Some(index)` or `None` if the board is full.
/// If a cell has zero candidates, returns it immediately (dead end).
fn find_best_empty(board: &[u8; 81]) -> Option<usize> {
    let mut best_idx = None;
    let mut best_count = 10u8;

    for idx in 0..81 {
        if board[idx] != 0 {
            continue;
        }

        let row = idx / 9;
        let col = idx % 9;
        let mut count = 0u8;

        for num in 1..=9u8 {
            if is_valid(board, row, col, num) {
                count += 1;
                if count >= best_count {
                    break;
                }
            }
        }

        if count == 0 {
            return Some(idx);
        }

        if count < best_count {
            best_count = count;
            best_idx = Some(idx);
            if best_count == 1 {
                break;
            }
        }
    }

    best_idx
}

/// Check whether placing `num` at `(row, col)` is valid.
/// The target cell is assumed to be empty (0), so no self-check is needed.
pub fn is_valid(board: &[u8; 81], row: usize, col: usize, num: u8) -> bool {
    for c in 0..9 {
        if board[row * 9 + c] == num {
            return false;
        }
    }
    for r in 0..9 {
        if board[r * 9 + col] == num {
            return false;
        }
    }
    let br = (row / 3) * 3;
    let bc = (col / 3) * 3;
    for r in br..br + 3 {
        for c in bc..bc + 3 {
            if board[r * 9 + c] == num {
                return false;
            }
        }
    }
    true
}

/// Simple Fisher-Yates shuffle.
pub fn shuffle<T>(arr: &mut [T]) {
    use rand::Rng;
    let n = arr.len();
    for i in (1..n).rev() {
        let j = rand::rng().random_range(0..=i);
        arr.swap(i, j);
    }
}

/// Find all solutions up to `limit`. Returns the count of solutions found.
pub fn solve_all(board: &[u8; 81], limit: usize) -> usize {
    let mut count = 0usize;
    let mut board_copy = *board;
    _solve_all(&mut board_copy, &mut count, limit);
    count
}

fn _solve_all(board: &mut [u8; 81], count: &mut usize, limit: usize) {
    if *count >= limit {
        return;
    }

    let Some(idx) = find_best_empty(board) else {
        *count += 1;
        return;
    };

    let row = idx / 9;
    let col = idx % 9;

    for num in 1..=9u8 {
        if is_valid(board, row, col, num) {
            board[idx] = num;
            _solve_all(board, count, limit);
            board[idx] = 0; // backtrack
        }
    }
}

/// Fill the board with a valid completed Sudoku using randomized backtracking.
/// Returns `true` if the board was successfully filled.
pub fn fill_board(board: &mut [u8; 81]) -> bool {
    fn _fill(board: &mut [u8; 81]) -> bool {
        let Some(idx) = find_best_empty(board) else {
            return true;
        };

        let row = idx / 9;
        let col = idx % 9;

        let mut candidates = [0u8; 9];
        let mut len = 0;
        for num in 1..=9u8 {
            if is_valid(board, row, col, num) {
                candidates[len] = num;
                len += 1;
            }
        }

        if len == 0 {
            return false;
        }

        shuffle(&mut candidates[..len]);

        for &num in candidates[..len].iter() {
            board[idx] = num;
            if _fill(board) {
                return true;
            }
            board[idx] = 0;
        }

        false
    }

    _fill(board)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_all_uniqueness() {
        // A well-known Sudoku puzzle with a unique solution (from Wikipedia).
        let mut board = [
            5, 3, 0, 0, 7, 0, 0, 0, 0,
            6, 0, 0, 1, 9, 5, 0, 0, 0,
            0, 9, 8, 0, 0, 0, 0, 6, 0,
            8, 0, 0, 0, 6, 0, 0, 0, 3,
            4, 0, 0, 8, 0, 3, 0, 0, 1,
            7, 0, 0, 0, 2, 0, 0, 0, 6,
            0, 6, 0, 0, 0, 0, 2, 8, 0,
            0, 0, 0, 4, 1, 9, 0, 0, 5,
            0, 0, 0, 0, 8, 0, 0, 7, 9,
        ];
        assert!(fill_board(&mut board));

        let mut puzzle = board;
        let holes: [usize; 4] = [0, 1, 2, 3];
        for h in &holes {
            puzzle[*h] = 0;
        }

        let count = solve_all(&puzzle, 2);
        assert_eq!(count, 1, "Puzzle should have a unique solution");
    }
}
