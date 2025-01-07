use colored::{Color, Colorize};

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

    /// Rotates the piece 90° counterclockwise.
    pub fn rotate_counterclockwise(self) -> Self {
        match self {
            Rotation::Zero => Rotation::TwoSeventy,
            Rotation::Ninety => Rotation::Zero,
            Rotation::OneEighty => Rotation::Ninety,
            Rotation::TwoSeventy => Rotation::OneEighty,
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
    shape: Vec<(i32, i32)>, // Offsets for the piece's shape
    pub symbol: char,       // Symbol for display
    pub rotation: Rotation, // Current rotation
}

impl Piece {
    /// Creates a new Piece with a symbol and shape.
    pub fn new(
        symbol: char,
        shape: Vec<(i32, i32)>,
        color: (u8, u8, u8),
        bg: (u8, u8, u8),
    ) -> Self {
        if shape.len() == 0 {
            panic!("A piece must consist of at least 1 block.");
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
        }
    }

    /// Returns the positions the piece would occupy given a base position.
    pub fn positions(&self, base_x: i32, base_y: i32) -> Vec<(i32, i32)> {
        self.shape
            .iter()
            .map(|(dx, dy)| (base_x + dx, base_y + dy))
            .collect()
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

    /// Rotates the piece by 90° clockwise.
    pub fn rotate_clockwise(&mut self) {
        self.rotation = self.rotation.rotate_clockwise();
        self.shape = self
            .shape
            .iter()
            .map(|(x, y)| (*y, -*x)) // Apply 90° rotation transformation
            .collect();
    }

    /// Rotates the piece by 90° counterclockwise.
    pub fn rotate_counterclockwise(&mut self) {
        self.rotation = self.rotation.rotate_counterclockwise();
        self.shape = self
            .shape
            .iter()
            .map(|(x, y)| (-*y, *x)) // Apply 90° counterclockwise rotation
            .collect();
    }

    /// Resets the piece to its default (0°) orientation.
    pub fn reset_rotation(&mut self) {
        while self.rotation != Rotation::Zero {
            self.rotate_clockwise();
        }
    }

    /// Returns the current rotation.
    pub fn get_rotation(&self) -> Rotation {
        self.rotation
    }

    /// Displays the symbol in its color.
    pub fn colored_symbol(&self) -> String {
        format!("{}", self.symbol.to_string().color(self.color))
    }
}
