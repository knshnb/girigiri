use board::state::*;
use board::helper::*;

pub const WEIGHT: [i32; 29] = [
    // black
    0,      // null
    86,     // pawn
    227,    // lance
    256,    // knight
    365,    // silver
    563,    // bishop
    629,    // rook
    540,    // propawn
    508,    // prolance
    517,    // proknight
    502,    // prosilver
    826,    // horse
    942,    // dragon
    439,    // gold
    15000,  // king
    // white
    -86,    // pawn
    -227,   // lance
    -256,   // knight
    -365,   // silver
    -563,   // bishop
    -629,   // rook
    -540,   // propawn
    -508,   // prolance
    -517,   // proknight
    -502,   // prosilver
    -826,   // horse
    -942,   // dragon
    -439,   // gold
    -15000, // king
];
    
pub fn eval(ref state: &State) -> i32 {
    let mut board_weight = 0;
    let mut hand_weight = 0;
    for i in 0..9 {
        for j in 0..9 {
            board_weight += WEIGHT[state.board[i][j] as usize];
        }
    }
    for n in 0..8 {
        hand_weight += (state.hand[1].get_num(n) as i32) * WEIGHT[kind_to_piece(n, true) as usize]; // black
        hand_weight += (state.hand[0].get_num(n) as i32) * WEIGHT[kind_to_piece(n, false) as usize]; // white
    }
    let mut weight = board_weight * 9 / 10 + hand_weight;
    if state.board[7][4] == 6 {
        weight += 10;
    }
    if state.board[8][4] == 6 {
        weight += 20;
    }
    if state.board[4][4] == 1 {
        weight += 5;
    }
    if state.color {
        weight
    } else {
        -weight
    }
}
