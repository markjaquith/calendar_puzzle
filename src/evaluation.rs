use crate::board::Board;

use crate::piece::Piece;

use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};

/// Recursively attempts to place all pieces on the board.
/// Returns a vector of boards that successfully place all pieces.
pub fn find_all_boards_placing_all_pieces(
    board: Board,
    pieces: &mut Vec<Piece>,
    found: &AtomicBool,
    first: bool,
) -> HashSet<Board> {
    // If no pieces are left, return the current board
    if pieces.is_empty() {
        // If the `--first` flag is set and a board has been found, mark it as found so other
        // threads can terminate early.
        if first {
            found.store(true, Ordering::Relaxed);
        }
        let mut final_board = HashSet::new();
        final_board.insert(board);
        return final_board;
    }

    // Remove the first piece and get all valid placements
    let mut piece = pieces.remove(0);
    let valid_boards = board.find_all_valid_boards_with_new_piece(&mut piece);

    // Use parallel iterator to process the valid boards
    let all_boards: HashSet<Board> = valid_boards
        .into_par_iter() // Convert to parallel iterator
        .flat_map(|valid_board| {
            if first && found.load(Ordering::Relaxed) {
                return HashSet::new(); // Terminate early if `--first` is set and a board is found
            }
            let mut remaining_pieces = pieces.clone();
            find_all_boards_placing_all_pieces(valid_board, &mut remaining_pieces, found, first)
                .into_iter() // Convert the returned Vec<Board> into an iterator
                .collect::<HashSet<_>>() // Collect into a HashSet to eliminate duplicates within each subresult
        })
        .collect(); // Collect into a HashSet to eliminate duplicates across all results

    // Restore the removed piece for the caller
    pieces.insert(0, piece);

    all_boards
}
