use crate::piece::Piece;

pub fn get_default_pieces() -> Vec<Piece> {
    vec![
        // A A A A A
        Piece::new(
            'A',
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)],
            (255, 255, 255), // White foreground
            (64, 140, 86),   // Green background
        ),
        // B B B B
        // B
        Piece::new(
            'B',
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (0, 1)],
            (255, 255, 255), // White foreground
            (0, 102, 204),   // Blue background
        ),
        // C
        // C C
        // C C
        Piece::new(
            'C',
            vec![(0, 0), (0, 1), (1, 1), (0, 2), (1, 2)],
            (255, 255, 255), // White foreground
            (204, 102, 0),   // Orange background
        ),
        // D D
        // D
        // D D
        Piece::new(
            'D',
            vec![(0, 0), (1, 0), (0, 1), (0, 2), (1, 2)],
            (0, 0, 0),       // Black foreground
            (239, 235, 231), // Light beige background
        ),
        // E
        // E E E
        // E
        Piece::new(
            'E',
            vec![(0, 0), (0, 1), (0, 2), (1, 1), (2, 1)],
            (0, 0, 0),      // Black foreground
            (120, 81, 169), // Purple background
        ),
        // F F F F
        //   F
        Piece::new(
            'F',
            vec![(0, 0), (1, 0), (2, 0), (3, 0), (1, 1)],
            (255, 255, 255), // White foreground
            (204, 102, 255), // Light purple background
        ),
        // G
        // G
        // G G G
        Piece::new(
            'G',
            vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            (255, 255, 255), // White foreground
            (88, 28, 71),    // Deep magenta background
        ),
        // H
        // H H H
        //     H
        Piece::new(
            'H',
            vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)],
            (120, 60, 60),   // Dark reddish-brown foreground
            (255, 255, 255), // White background
        ),
        //   I I I
        // I I
        Piece::new(
            'I',
            vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1)],
            (255, 255, 255), // White foreground
            (102, 204, 153), // Soft teal background
        ),
        //   J
        // J J
        //   J J
        Piece::new(
            'J',
            vec![(1, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
            (0, 0, 0),       // Black foreground
            (255, 204, 102), // Light orange background
        ),
    ]
}

pub fn get_corner_piece() -> Piece {
    Piece::new(' ', vec![(0, 0)], (255, 255, 255), (0, 0, 0))
}
