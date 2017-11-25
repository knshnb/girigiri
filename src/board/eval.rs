use board::state::*;

pub const PIECE_TO_WEIGHT: [i32; 29] = [
    // black
    0 * 9 / 10,      // null
    86 * 9 / 10,     // pawn
    227 * 9 / 10,    // lance
    256 * 9 / 10,    // knight
    365 * 9 / 10,    // silver
    563 * 9 / 10,    // bishop
    629 * 9 / 10,    // rook
    540 * 9 / 10,    // propawn
    508 * 9 / 10,    // prolance
    517 * 9 / 10,    // proknight
    502 * 9 / 10,    // prosilver
    826 * 9 / 10,    // horse
    942 * 9 / 10,    // dragon
    439 * 9 / 10,    // gold
    15000 * 9 / 10,  // king
    // white
    86 * 9 / 10,    // pawn
    227 * 9 / 10,   // lance
    256 * 9 / 10,   // knight
    365 * 9 / 10,   // silver
    563 * 9 / 10,   // bishop
    629 * 9 / 10,   // rook
    540 * 9 / 10,   // propawn
    508 * 9 / 10,   // prolance
    517 * 9 / 10,   // proknight
    502 * 9 / 10,   // prosilver
    826 * 9 / 10,   // horse
    942 * 9 / 10,   // dragon
    439 * 9 / 10,   // gold
    15000 * 9 / 10, // king
];

pub const KIND_TO_WEIGHT: [i32; 8] = [
    86,     // pawn
    227,    // lance
    256,    // knight
    365,    // silver
    563,    // bishop
    629,    // rook
    439,    // gold
    15000,  // king
];
    
pub fn eval(ref state: &State) -> i32 {
    state.weight
}
