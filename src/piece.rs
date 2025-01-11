use colored::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Zero,       // 0°
    Ninety,     // 90°
    OneEighty,  // 180°
    TwoSeventy, // 270°
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub symbol: char,
    pub color: Color,
    pub bg: Color,
    shape: Vec<(i32, i32)>,          // Default (unrotated) shape
    rotations: Vec<Vec<(i32, i32)>>, // Precomputed rotations
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
}
