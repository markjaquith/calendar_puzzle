use crate::piece::Piece;
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,  // Width of the board
    pub height: usize, // Height of the board
    pub grid: Vec<Vec<Option<(char, colored::Color, colored::Color)>>>, // Store symbol and color and bg for each cell
    blank: char,                                                        // Symbol for empty cells
}

impl Board {
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

    /// Checks if a piece can be placed at the given base position.
    pub fn can_place_piece(&self, piece: &Piece, base_x: i32, base_y: i32) -> Result<(), String> {
        for &(x, y) in piece.positions(base_x, base_y).iter() {
            if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
                return Err(format!("Position ({}, {}) is out of bounds.", x, y));
            }
            if self.grid[y as usize][x as usize].is_some() {
                return Err(format!("Position ({}, {}) is already occupied.", x, y));
            }
        }
        Ok(())
    }

    /// Places a piece on the board if it fits.
    pub fn place_piece(&mut self, piece: &Piece, base_x: i32, base_y: i32) -> bool {
        match self.can_place_piece(piece, base_x, base_y) {
            Ok(_) => {
                for (x, y) in piece.positions(base_x, base_y) {
                    self.grid[y as usize][x as usize] = Some((piece.symbol, piece.color, piece.bg));
                }
                true
            }
            Err(err) => {
                eprintln!("Failed to place piece: {}", err);
                false
            }
        }
    }

    /// Displays the board in a simple ASCII format.
    pub fn display(&self) {
        for row in &self.grid {
            for cell in row {
                match cell {
                    Some((symbol, color, bg)) => {
                        print!(
                            "{}{}{}",
                            " ".on_color(*bg),
                            symbol.to_string().color(*color).on_color(*bg),
                            " ".on_color(*bg),
                        ); // Apply color to symbol
                    }
                    None => print!(" {} ", self.blank),
                }
            }
            println!(); // Newline after each row
        }
    }
}
