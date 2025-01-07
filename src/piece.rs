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
    shape: Vec<(i32, i32)>, // Offsets for the piece's shape
    pub symbol: char,       // Symbol for display
    pub rotation: Rotation, // Current rotation
}

impl Piece {
    /// Creates a new Piece with a symbol and shape.
    pub fn new(symbol: char, shape: Vec<(i32, i32)>) -> Self {
        if shape.len() == 0 {
            panic!("A piece must consist of at least 1 block.");
        }
        Piece {
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
}
