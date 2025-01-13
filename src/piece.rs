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

type Coordinates = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub rotation: Rotation,
    pub x: i32,
    pub y: i32,
}

impl Placement {
    pub fn new(rotation: Rotation, coordinates: Coordinates) -> Self {
        Placement {
            rotation,
            x: coordinates.0,
            y: coordinates.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub symbol: char,
    pub color: Color,
    pub bg: Color,
    shape: Vec<(i32, i32)>,             // Default (unrotated) shape
    rotations: Vec<Vec<(i32, i32)>>,    // Precomputed rotations
    allowed_placements: Vec<Placement>, // Allowed placements for this piece
}

impl Piece {
    /// Creates a new Piece with precomputed rotations.
    pub fn new(
        symbol: char,
        shape: Vec<(i32, i32)>,
        color: (u8, u8, u8),
        bg: (u8, u8, u8),
    ) -> Self {
        if shape.is_empty() {
            panic!("A piece must consist of at least 1 block.");
        }

        // Precompute all rotations
        let rotations = (0..4)
            .scan(shape.clone(), |current_shape, _| {
                let result = current_shape.clone();
                *current_shape = current_shape
                    .iter()
                    .map(|(x, y)| (*y, -*x)) // Rotate 90° clockwise
                    .collect();
                Some(result)
            })
            .collect();

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
            shape,
            symbol,
            rotations,
            allowed_placements: Vec::new(),
        }
    }

    /// Get the dimensions of the default (unrotated) shape.
    pub fn get_default_dimensions(&self) -> (i32, i32) {
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in &self.shape {
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        (max_x + 1, max_y + 1)
    }

    /// Get the shape at a specific rotation.
    pub fn rotated_to(&self, rotation: Rotation) -> &Vec<(i32, i32)> {
        &self.rotations[match rotation {
            Rotation::Zero => 0,
            Rotation::Ninety => 1,
            Rotation::OneEighty => 2,
            Rotation::TwoSeventy => 3,
        }]
    }

    /// Precomputes and stores allowed placements for this piece based on the given board.
    /// The board provides its dimensions and current occupied cells (treated as forbidden).
    pub fn precompute_allowed_placements(&mut self, board: &Board) {
        let mut allowed_placements = Vec::new();

        for rotation in Rotation::iter() {
            let shape = self.rotated_to(rotation);

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
                        allowed_placements.push(Placement::new(rotation, (x, y)));
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
