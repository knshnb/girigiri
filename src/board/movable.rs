#[derive(Clone, Copy)]
pub enum Direction {
    UpLeft, Up, UpRight,
    Left, Right,
    DownLeft, Down, DownRight,
    UpUpLeft, UpUpRight,
    DownDownLeft, DownDownRight,
}

use self::Direction::*;
lazy_static! {
    pub static ref SHORT_MOVABLE: [Vec<Direction>; 32] = [
        // white
        vec![Down],                                                         // pawn
        vec![],                                                             // lance
        vec![DownDownLeft, DownDownRight],                                  // knight
        vec![UpLeft, UpRight, DownLeft, Down, DownRight],                   // silver
        vec![],                                                             // bishop
        vec![],                                                             // rook
        vec![],                                                             // padding
        vec![],                                                             // null
        vec![Up, Left, Right, DownLeft, Down, DownRight],                   // propawn
        vec![Up, Left, Right, DownLeft, Down, DownRight],                   // prolance
        vec![Up, Left, Right, DownLeft, Down, DownRight],                   // proknight
        vec![Up, Left, Right, DownLeft, Down, DownRight],                   // prosilver
        vec![Up, Left, Right, Down],                                        // horse
        vec![UpLeft, UpRight, DownLeft, DownRight],                         // dragon
        vec![Up, Left, Right, DownLeft, Down, DownRight],                   // gold
        vec![UpLeft, Up, UpRight, Left, Right, DownLeft, Down, DownRight],  // king

        // black
        vec![Up],                                                           // pawn
        vec![],                                                             // lance
        vec![UpUpLeft, UpUpRight],                                          // knight
        vec![UpLeft, Up, UpRight, DownLeft, DownRight],                     // silver
        vec![],                                                             // bishop
        vec![],                                                             // rook
        vec![],                                                             // padding
        vec![],                                                             // padding
        vec![UpLeft, Up, UpRight, Left, Right, Down],                       // propawn
        vec![UpLeft, Up, UpRight, Left, Right, Down],                       // prolance
        vec![UpLeft, Up, UpRight, Left, Right, Down],                       // proknight
        vec![UpLeft, Up, UpRight, Left, Right, Down],                       // prosilver
        vec![Up, Left, Right, Down],                                        // horse
        vec![UpLeft, UpRight, DownLeft, DownRight],                         // dragon
        vec![UpLeft, Up, UpRight, Left, Right, Down],                       // gold
        vec![UpLeft, Up, UpRight, Left, Right, DownLeft, Down, DownRight],  // king

    ];
    pub static ref LONG_MOVABLE: [Vec<Direction>; 32] = [
        // white
        vec![],                                                             // pawn
        vec![Down],                                                         // lance
        vec![],                                                             // knight
        vec![],                                                             // silver
        vec![UpLeft, UpRight, DownLeft, DownRight],                         // bishop
        vec![Up, Left, Right, Down],                                        // rook
        vec![],                                                             // padding
        vec![],                                                             // null
        vec![],                                                             // propawn
        vec![],                                                             // prolance
        vec![],                                                             // proknight
        vec![],                                                             // prosilver
        vec![UpLeft, UpRight, DownLeft, DownRight],                         // horse
        vec![Up, Left, Right, Down],                                        // dragon
        vec![],                                                             // gold
        vec![],                                                             // king

        // black
        vec![],                                                             // pawn
        vec![Up],                                                           // lance
        vec![],                                                             // knight
        vec![],                                                             // silver
        vec![UpLeft, UpRight, DownLeft, DownRight],                         // bishop
        vec![Up, Left, Right, Down],                                        // rook
        vec![],                                                             // padding
        vec![],                                                             // padding
        vec![],                                                             // propawn
        vec![],                                                             // prolance
        vec![],                                                             // proknight
        vec![],                                                             // prosilver
        vec![UpLeft, UpRight, DownLeft, DownRight],                         // horse
        vec![Up, Left, Right, Down],                                        // dragon
        vec![],                                                             // gold
        vec![],                                                             // king
    ];
}
