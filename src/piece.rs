use colored::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Zero,       // 0°
    Ninety,     // 90°
    OneEighty,  // 180°
    TwoSeventy, // 270°
}

impl Rotation {
    /// Rotates the piece 90° clockwise.
    pub fn rotate_clockwise(self) -> Self {
        match self {
            Rotation::Zero => Rotation::Ninety,
            Rotation::Ninety => Rotation::OneEighty,
            Rotation::OneEighty => Rotation::TwoSeventy,
            Rotation::TwoSeventy => Rotation::Zero,
        }
    }

    /// Returns the corresponding angle in degrees (if needed).
    pub fn to_degrees(self) -> i32 {
        match self {
            Rotation::Zero => 0,
            Rotation::Ninety => 90,
            Rotation::OneEighty => 180,
            Rotation::TwoSeventy => 270,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub color: Color,
    pub bg: Color,
    shape: Vec<(i32, i32)>, // Default (unrotated) shape
    pub symbol: char,
    rotation: Rotation,                          // Current rotation
    precomputed_rotations: Vec<Vec<(i32, i32)>>, // Cached rotations
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
        let mut precomputed_rotations = Vec::new();
        let mut current_shape = shape.clone();

        for _ in 0..4 {
            precomputed_rotations.push(current_shape.clone());
            current_shape = current_shape
                .iter()
                .map(|(x, y)| (*y, -*x)) // Rotate 90° clockwise
                .collect();
        }

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
            rotation: Rotation::Zero,
            precomputed_rotations,
        }
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
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

    /// Get the current shape based on the current rotation.
    pub fn current_shape(&self) -> &Vec<(i32, i32)> {
        &self.precomputed_rotations[self.rotation.to_degrees() as usize / 90]
    }

    /// Rotate the piece 90° clockwise.
    pub fn rotate_clockwise(&mut self) {
        self.rotation = self.rotation.rotate_clockwise();
    }

    /// Reset the piece to its default (0°) orientation.
    pub fn reset_rotation(&mut self) {
        self.rotation = Rotation::Zero;
    }

    /// Returns the current rotation.
    pub fn get_rotation(&self) -> Rotation {
        self.rotation
    }
}
