#[derive(Clone, Debug)]
pub struct Hand(u32);
// 0-5: pawn, 7-9: lance, 11-13: knight, 15-17: silver, 19-20: bishop, 22-23: rook, 25-27: gold,
// 29: king

pub const HAND_BIT_SHIFTS : [u32; 8] = [
    0,
    6,
    10,
    14,
    18,
    21,
    24,
    28,
];

pub const HAND_MASKS : [u32; 8] = [
    0b11111,        // pawn
    0b111 << 6,     // lance
    0b111 << 10,    // knight
    0b111 << 14,    // silver
    0b11 << 18,     // bishop
    0b11 << 21,     // rook
    0b111 << 24,    // gold
    0b1 << 28,      // king
];

pub const HAND_UNITS : [u32; 8] = [
    1,          // pawn
    1 << 6,     // lance
    1 << 10,    // knight
    1 << 14,    // silver
    1 << 18,    // bishop
    1 << 21,    // rook
    1 << 24,    // gold
    1 << 28,    // king
];

impl Hand {
    pub fn new() -> Hand {
        Hand(0)
    }
    pub fn add(&mut self, kind: usize) {
        self.0 += HAND_UNITS[kind];
    }
    pub fn sub(&mut self, kind: usize) {
        self.0 -= HAND_UNITS[kind];
    }
    pub fn own(&self, kind: usize) -> bool {
        self.0 & HAND_MASKS[kind] != 0
    }
    pub fn get_num(&self, kind: usize) -> u32 {
        (self.0 & HAND_MASKS[kind]) >> HAND_BIT_SHIFTS[kind]
    }
}
