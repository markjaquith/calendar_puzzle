use crate::piece::{Piece, Rotation};
use colored::Colorize;
use std::hash::{Hash, Hasher};

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
    pub fn can_place_piece(&self, piece: &Piece, coordinates: (i32, i32)) -> Result<(), String> {
        let (base_x, base_y) = coordinates;
        for &(dx, dy) in piece.current_shape() {
            let x = base_x + dx;
            let y = base_y + dy;
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
    pub fn place_piece(&mut self, piece: &Piece, coordinates: (i32, i32)) -> bool {
        let (base_x, base_y) = coordinates;
        match self.can_place_piece(piece, coordinates) {
            Ok(_) => {
                for &(dx, dy) in piece.current_shape() {
                    let x = base_x + dx;
                    let y = base_y + dy;
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

    /// Returns the display representation of the board as a String, including colors.
    pub fn get_display(&self) -> String {
        let mut display = String::new();
        for row in &self.grid {
            for cell in row {
                match cell {
                    Some((symbol, color, bg)) => {
                        let colored_symbol = format!(
                            "{}{}{}",
                            " ".on_color(*bg),
                            symbol.to_string().color(*color).on_color(*bg),
                            " ".on_color(*bg),
                        );
                        display.push_str(&colored_symbol);
                    }
                    None => display.push_str(&format!(" {} ", self.blank)),
                }
            }
            display.push('\n'); // Newline after each row
        }
        display
    }

    /// Displays the board in a simple ASCII format.
    pub fn display(&self) {
        println!("{}", self.get_display());
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
    pub fn has_dead_end_blanks_smaller_than(&self, max_size: usize) -> bool {
        self.scan_blank_areas().iter().any(|&size| size < max_size)
    }

    /// Finds child boards that fit a given new piece.
    pub fn find_all_valid_boards_with_new_piece(&self, piece: &mut Piece) -> Vec<Board> {
        let mut valid_boards: Vec<Board> = Vec::new();

        for rotation in [
            Rotation::Zero,
            Rotation::Ninety,
            Rotation::OneEighty,
            Rotation::TwoSeventy,
        ] {
            // Rotate the piece to the current orientation
            while piece.get_rotation() != rotation {
                piece.rotate_clockwise();
            }

            // Try placing the piece in every position on the board
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.can_place_piece(piece, (x as i32, y as i32)).is_ok() {
                        let mut new_board = self.clone(); // Clone the current board
                        new_board.place_piece(piece, (x as i32, y as i32)); // Place the piece
                        if !new_board.has_dead_end_blanks_smaller_than(5) {
                            valid_boards.push(new_board); // Push the owned board
                        }
                    }
                }
            }

            // Reset the piece to its original rotation after testing
            piece.reset_rotation();
        }

        valid_boards
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.get_display() == other.get_display()
    }
}

impl Eq for Board {}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_display().hash(state);
    }
}
