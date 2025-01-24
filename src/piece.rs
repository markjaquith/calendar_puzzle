use strum::IntoEnumIterator;

use colored::Color;

use strum_macros::EnumIter;

use crate::board::Board;

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Rotation {
    Zero,       // 0°
    Ninety,     // 90°
    OneEighty,  // 180°
    TwoSeventy, // 270°
}

pub type Coordinates = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub rotation: Rotation,
    pub flipped: bool,
    pub x: i32,
    pub y: i32,
}

impl Placement {
    pub fn new(rotation: Rotation, flipped: bool, coordinates: Coordinates) -> Self {
        Placement {
            rotation,
            flipped,
            x: coordinates.0,
            y: coordinates.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub display_symbol: char,
    pub symbol: char,
    pub color: Color,
    pub bg: Color,
    pub flippable: bool,
    rotations: Vec<Vec<Coordinates>>, // Precomputed rotations
    rotations_flipped: Option<Vec<Vec<Coordinates>>>, // Precomputed flipped rotations if flippable
    allowed_placements: Vec<Placement>, // Allowed placements for this piece
}

impl Piece {
    /// Creates a new Piece with precomputed rotations.
    pub fn new(
        symbol: char,
        shape: Vec<Coordinates>,
        color: (u8, u8, u8),
        bg: (u8, u8, u8),
        flippable: bool,
    ) -> Self {
        if shape.is_empty() {
            panic!("A piece must consist of at least 1 block.");
        }

        // Precompute all rotations
        let rotations: Vec<_> = (0..4)
            .scan(shape.clone(), |current_shape, _| {
                let result = current_shape.clone();
                *current_shape = current_shape
                    .iter()
                    .map(|(x, y)| (*y, -*x)) // Rotate 90° clockwise
                    .collect();
                Some(result)
            })
            .collect();

        let rotations_flipped = match flippable {
            true => Some(
                rotations
                    .iter()
                    .map(|rotation| {
                        rotation
                            .iter()
                            .map(|(x, y)| (-*x, *y)) // Flip horizontally
                            .collect()
                    })
                    .collect(),
            ),
            false => None,
        };

        Piece {
            color: Color::TrueColor {
                r: color.0,
                g: color.1,
                b: color.2,
            },
            bg: Color::TrueColor {
                r: bg.0,
                g: bg.1,
                b: bg.2,
            },
            display_symbol: symbol,
            symbol,
            rotations,
            rotations_flipped,
            flippable,
            allowed_placements: Vec::new(),
        }
    }

    /// Get the dimensions of the default (unrotated) shape.
    pub fn get_dimensions_of_shape(&self, rotation: Rotation, flipped: bool) -> (i32, i32) {
        // Iterate over the shape and find the maximum x and y values of its coordinates
        // which is therefore the width and height of the shape.
        self.get_shape(rotation, flipped)
            .iter()
            .fold((0, 0), |(max_x, max_y), &(x, y)| {
                (max_x.max(x + 1), max_y.max(y + 1))
            })
    }

    /// Get the shape at a specific rotation.
    pub fn get_shape(&self, rotation: Rotation, flipped: bool) -> &Vec<Coordinates> {
        let index = match rotation {
            Rotation::Zero => 0,
            Rotation::Ninety => 1,
            Rotation::OneEighty => 2,
            Rotation::TwoSeventy => 3,
        };

        match flipped {
            true => &self.rotations_flipped.as_ref().unwrap()[index],
            false => &self.rotations[index],
        }
    }

    pub fn display_as(&mut self, symbol: char) {
        self.display_symbol = symbol;
    }

    /// Precomputes and stores allowed placements for this piece based on the given board.
    /// The board provides its dimensions and current occupied cells (treated as forbidden).
    pub fn precompute_allowed_placements(&mut self, board: &Board) {
        let mut allowed_placements = Vec::new();

        for rotation in Rotation::iter() {
            for flip in [false, true] {
                if flip && !self.flippable {
                    continue;
                }
                let shape = self.get_shape(rotation, flip);

                for y in 0..board.height as i32 {
                    for x in 0..board.width as i32 {
                        let mut is_valid = true;

                        for &(dx, dy) in shape {
                            let xx = x + dx;
                            let yy = y + dy;

                            if xx < 0
                                || xx >= board.width as i32
                                || yy < 0
                                || yy >= board.height as i32
                                || board.grid[yy as usize][xx as usize].is_some()
                            {
                                is_valid = false;
                                break;
                            }
                        }

                        if is_valid {
                            allowed_placements.push(Placement::new(rotation, flip, (x, y)));
                        }
                    }
                }
            }
        }

        self.allowed_placements = allowed_placements;
    }

    /// Gets the allowed positions for this piece.
    pub fn get_allowed_placements(&self) -> &Vec<Placement> {
        &self.allowed_placements
    }
}
