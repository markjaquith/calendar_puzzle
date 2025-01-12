use crate::piece::Piece;

pub fn get_default_pieces() -> Vec<Piece> {
    vec![
        // I I I I I
        Piece::new(
            'I',
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
            (255, 255, 255), // White foreground
            (64, 140, 86),   // Green background
        ),
        // L L L L
        // L
        Piece::new(
            'L',
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)],
            (255, 255, 255), // White foreground
            (0, 102, 204),   // Blue background
        ),
        // P
        // P P
        // P P
        Piece::new(
            'P',
            vec![(0, 0), (0, 1), (1, 1), (0, 2), (1, 2)],
            (255, 255, 255), // White foreground
            (204, 102, 0),   // Orange background
        ),
        // U U
        // U
        // U U
        Piece::new(
            'U',
            vec![(0, 0), (1, 0), (0, 1), (0, 2), (1, 2)],
            (0, 0, 0),       // Black foreground
            (239, 235, 231), // Light beige background
        ),
        // T
        // T T T
        // T
        Piece::new(
            'T',
            vec![(0, 0), (0, 1), (0, 2), (1, 1), (2, 1)],
            (0, 0, 0),      // Black foreground
            (120, 81, 169), // Purple background
        ),
        // Y Y Y Y
        //   Y
        Piece::new(
            'Y',
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (1, 1)],
            (255, 255, 255), // White foreground
            (204, 102, 255), // Light purple background
        ),
        // V
        // V
        // V V V
        Piece::new(
            'V',
            vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            (255, 255, 255), // White foreground
            (88, 28, 71),    // Deep magenta background
        ),
        // Z
        // Z Z Z
        //     Z
        Piece::new(
            'Z',
            vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)],
            (120, 60, 60),   // Dark reddish-brown foreground
            (255, 255, 255), // White background
        ),
        //   N N N
        // N N
        Piece::new(
            'N',
            vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1)],
            (255, 255, 255), // White foreground
            (102, 204, 153), // Soft teal background
        ),
        //   F
        // F F
        //   F F
        Piece::new(
            'F',
            vec![(1, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
            (0, 0, 0),       // Black foreground
            (255, 204, 102), // Light orange background
        ),
    ]
}

pub fn get_corner_piece() -> Piece {
    Piece::new(' ', vec![(0, 0)], (255, 255, 255), (0, 0, 0))
}
