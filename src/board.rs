use crate::calendar::Day;
use crate::piece::{Piece, Placement, Rotation};
use crate::pieces::Pieces;
use colored::Colorize;
use lazy_static::lazy_static;
use std::hash::{Hash, Hasher};

use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};

/// Configuration
const BOARD_WIDTH: usize = 9;
const BOARD_HEIGHT: usize = 6;
const MISSING_CORNER_COORDINATES: (i32, i32) = (8, 5);

// Static pieces
lazy_static! {
    static ref MONTH_PIECE: Piece = Pieces::get_month();
    static ref DAY_PIECE: Piece = Pieces::get_day();
    static ref WEEKDAY_PIECE: Piece = Pieces::get_weekday();
    static ref corner_piece: Piece = Pieces::get_corner();
}

#[derive(Debug, Clone)]
pub struct Board<'a> {
    pub width: usize,                      // Width of the board
    pub height: usize,                     // Height of the board
    pub grid: Vec<Vec<Option<&'a Piece>>>, // Store Piece type for each cell
    blank: char,                           // Symbol for empty cells
}

impl<'a> Board<'a> {
    /// Creates a new Board with the given dimensions.
    pub fn new(width: usize, height: usize, blank: char) -> Self {
        let grid = vec![vec![None; width]; height];
        Board {
            width,
            height,
            grid,
            blank,
        }
    }

    pub fn make(day: &Day) -> Self {
        // Define the initial board.
        let mut board = Board::new(BOARD_WIDTH, BOARD_HEIGHT, '·');

        // Place the calendar pieces on the board.
        board.place_piece(
            &*MONTH_PIECE,
            Placement::new(Rotation::Zero, day.month.to_coordinates()),
        );
        board.place_piece(
            &*DAY_PIECE,
            Placement::new(Rotation::Zero, day.day.to_coordinates()),
        );
        board.place_piece(
            &*WEEKDAY_PIECE,
            Placement::new(Rotation::Zero, day.weekday.to_coordinates()),
        );

        // Place the corner piece on the board.
        board.place_piece(
            &corner_piece,
            Placement::new(Rotation::Zero, MISSING_CORNER_COORDINATES),
        );
        board
    }

    /// Checks if a piece can be placed at the given base position and rotation.
    pub fn can_place_piece(&self, piece: &'a Piece, placement: Placement) -> Result<(), String> {
        for &(dx, dy) in piece.rotated_to(placement.rotation) {
            let x = placement.x + dx;
            let y = placement.y + dy;
            if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
                return Err(format!("Position ({}, {}) is out of bounds.", x, y));
            }
            if self.grid[y as usize][x as usize].is_some() {
                return Err(format!("Position ({}, {}) is already occupied.", x, y));
            }
        }
        Ok(())
    }

    /// Places a piece on the board if it fits, at a specific rotation.
    pub fn place_piece(&mut self, piece: &'a Piece, placement: Placement) -> bool {
        match self.can_place_piece(piece, placement) {
            Ok(_) => {
                for &(dx, dy) in piece.rotated_to(placement.rotation) {
                    let x = placement.x + dx;
                    let y = placement.y + dy;
                    self.grid[y as usize][x as usize] = Some(&piece);
                }
                true
            }
            Err(err) => {
                eprintln!("Failed to place piece: {}", err);
                false
            }
        }
    }

    /// Output the board state in a simple string format, for hashing and unit testing
    /// Like get_display, but without colors or formatting or newlines
    pub fn serialize(&self) -> String {
        self.grid
            .iter()
            .flat_map(|row| {
                row.iter().map(|cell| match cell {
                    Some(piece) => piece.serialized_symbol,
                    None => self.blank,
                })
            })
            .collect()
    }

    /// Displays the board (colored).
    pub fn display(&self) {
        for row in &self.grid {
            for cell in row {
                match cell {
                    Some(piece) => {
                        print!(
                            "{}{}{}",
                            " ".on_color(piece.bg),
                            piece
                                .symbol
                                .to_string()
                                .color(piece.color)
                                .on_color(piece.bg),
                            " ".on_color(piece.bg),
                        );
                    }
                    None => print!(" {} ", self.blank),
                }
            }
            print!("{}", '\n'); // Newline after each row
        }
    }

    /// Scans the board for all contiguous blank areas.
    /// Returns a vector of sizes of each blank region.
    pub fn scan_blank_areas(&self) -> Vec<usize> {
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut blank_areas = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                // Skip visited cells and non-blank cells
                if visited[y][x] || self.grid[y][x].is_some() {
                    continue;
                }

                // Perform flood fill to calculate the size of this blank area
                let size = self.flood_fill_blank(x, y, &mut visited);
                blank_areas.push(size);
            }
        }

        blank_areas
    }

    /// Performs a flood fill to calculate the size of a blank area.
    fn flood_fill_blank(
        &self,
        start_x: usize,
        start_y: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> usize {
        let mut stack = vec![(start_x, start_y)];
        let mut size = 0;

        while let Some((x, y)) = stack.pop() {
            // Skip out-of-bounds or already visited cells
            if x >= self.width || y >= self.height || visited[y][x] || self.grid[y][x].is_some() {
                continue;
            }

            // Mark the cell as visited and increase the size
            visited[y][x] = true;
            size += 1;

            // Add neighboring cells to the stack
            stack.push((x + 1, y)); // Right
            stack.push((x.wrapping_sub(1), y)); // Left (with wrapping prevention)
            stack.push((x, y + 1)); // Down
            stack.push((x, y.wrapping_sub(1))); // Up (with wrapping prevention)
        }

        size
    }

    /// Checks if the board contains any dead-end blank areas.
    pub fn has_dead_end_blanks_not_divisible_by(&self, divisible_by: usize) -> bool {
        self.scan_blank_areas()
            .iter()
            .any(|&size| size % divisible_by != 0)
    }

    /// Finds all valid boards by placing a new piece in all possible positions and rotations.
    pub fn find_all_valid_boards_with_new_piece(&self, piece: &'a Piece) -> Vec<Board<'a>> {
        let mut valid_boards: Vec<Board<'a>> = Vec::new();

        for &placement in piece.get_allowed_placements() {
            if self.can_place_piece(piece, placement).is_ok() {
                let mut new_board = self.clone();
                new_board.place_piece(piece, placement);
                if !new_board.has_dead_end_blanks_not_divisible_by(5) {
                    valid_boards.push(new_board);
                }
            }
        }

        valid_boards
    }

    /// Recursively attempts to place all pieces on the board.
    /// Returns a vector of boards that successfully place all pieces.
    pub fn find_boards_placing_all_pieces(
        &self,
        pieces: &[&'a Piece],
        found: &AtomicBool,
        find_all: bool,
    ) -> HashSet<Board<'a>> {
        if pieces.is_empty() {
            if !find_all {
                found.store(true, Ordering::Relaxed);
            }
            return HashSet::from([self.clone()]);
        }

        if !find_all && found.load(Ordering::Relaxed) {
            return HashSet::new();
        }

        let piece = pieces[0];
        let valid_boards = self.find_all_valid_boards_with_new_piece(piece);

        valid_boards
            .into_par_iter()
            .flat_map(|valid_board| {
                valid_board.find_boards_placing_all_pieces(&pieces[1..], found, find_all)
            })
            .collect()
    }
}

impl PartialEq for Board<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.serialize() == other.serialize()
    }
}

impl Eq for Board<'_> {}

impl Hash for Board<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.serialize().hash(state);
    }
}
