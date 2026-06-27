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

    /// A well-known Sudoku puzzle (from Wikipedia).
    const WIKI_PUZZLE: [u8; 81] = [
        5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0, 0,
        0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0, 0, 0,
        2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
    ];

    #[test]
    fn test_is_valid_row() {
        let mut board = [0u8; 81];
        board[0] = 5;
        // Same row, different column — should be invalid
        assert!(!is_valid(&board, 0, 1, 5));
        // Different value — should be valid
        assert!(is_valid(&board, 0, 1, 3));
    }

    #[test]
    fn test_is_valid_col() {
        let mut board = [0u8; 81];
        board[0] = 5; // row 0, col 0
        // Same column, different row — should be invalid
        assert!(!is_valid(&board, 4, 0, 5));
        // Different value — should be valid
        assert!(is_valid(&board, 4, 0, 3));
    }

    #[test]
    fn test_is_valid_box() {
        let mut board = [0u8; 81];
        board[0] = 5; // (0,0), box (0,0)
        // Same box — should be invalid
        assert!(!is_valid(&board, 1, 1, 5));
        // Different box and different row — should be valid
        assert!(is_valid(&board, 1, 4, 5));
    }

    #[test]
    fn test_is_valid_empty_board() {
        let board = [0u8; 81];
        for num in 1..=9u8 {
            assert!(is_valid(&board, 4, 4, num));
        }
    }

    #[test]
    fn test_find_empty_on_partial() {
        let mut board = [0u8; 81];
        board[0] = 1;
        board[1] = 2;
        assert_eq!(find_empty(&board), Some(2));
    }

    #[test]
    fn test_find_empty_on_full() {
        let board = [1u8; 81];
        assert_eq!(find_empty(&board), None);
    }

    #[test]
    fn test_solve_all_no_solution() {
        // Board where cell (0,0) has 0 valid candidates because its row,
        // column, and box collectively contain all digits 1-9.
        let mut board = [0u8; 81];
        board[1] = 1; board[2] = 2; board[3] = 3;
        board[4] = 4; board[5] = 5; board[6] = 6;
        board[7] = 7; board[8] = 8;  // Row 0: 1-8 in cols 1-8
        board[9] = 9;                // Col 0: 9 at (1,0)
        assert_eq!(solve_all(&board, 2), 0);
    }

    #[test]
    fn test_solve_all_multiple_solutions() {
        // Board with two independent clues should have multiple solutions
        let mut board = [0u8; 81];
        board[0] = 1;  // (0, 0)
        board[80] = 2; // (8, 8)
        let count = solve_all(&board, 2);
        assert!(count >= 2, "Under-constrained board should have at least 2 solutions");
    }

    #[test]
    fn test_solve_all_uniqueness() {
        let mut board = WIKI_PUZZLE;
        assert!(fill_board(&mut board));

        let mut puzzle = board;
        let holes: [usize; 4] = [0, 1, 2, 3];
        for h in &holes {
            puzzle[*h] = 0;
        }

        let count = solve_all(&puzzle, 2);
        assert_eq!(count, 1, "Puzzle should have a unique solution");
    }

    #[test]
    fn test_fill_board_empty() {
        let mut board = [0u8; 81];
        assert!(fill_board(&mut board));
        // Board should be completely filled
        for &v in &board {
            assert_ne!(v, 0, "Board should have no empty cells");
        }
        // Solution should be valid
        for r in 0..9 {
            for c in 0..9 {
                let val = board[r * 9 + c];
                board[r * 9 + c] = 0;
                assert!(is_valid(&board, r, c, val));
                board[r * 9 + c] = val;
            }
        }
    }

    #[test]
    fn test_fill_board_partial() {
        let mut board = WIKI_PUZZLE;
        assert!(fill_board(&mut board));
        for &v in &board {
            assert_ne!(v, 0);
        }
        // All original non-zero clues must be preserved
        for (i, &v) in WIKI_PUZZLE.iter().enumerate() {
            if v != 0 {
                assert_eq!(
                    board[i], v,
                    "Original clue at index {i} should be preserved"
                );
            }
        }
    }

    #[test]
    fn test_fill_board_idempotent() {
        let mut board = [0u8; 81];
        assert!(fill_board(&mut board));
        assert!(fill_board(&mut board));
        for r in 0..9 {
            for c in 0..9 {
                let val = board[r * 9 + c];
                board[r * 9 + c] = 0;
                assert!(is_valid(&board, r, c, val));
                board[r * 9 + c] = val;
            }
        }
    }
}
