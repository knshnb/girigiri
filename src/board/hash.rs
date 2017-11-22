extern crate rand;

use std::u64;
use self::rand::Rng;
use board::move_encode::*;

#[derive(Copy, Clone)]
pub struct HashEntry {
    pub hash_key: u64,
    pub color: bool, // is_black
    pub value: i32,
    pub remain_depth: u8,
    pub best_move: Move,
}

pub const HASH_ENTRY_NONE: HashEntry = HashEntry {
    hash_key: 0,
    color: false,
    value: 0,
    remain_depth: 0,
    best_move: NULL_MOVE,
};

pub const HASH_TABLE_SIZE: usize = 1 << 16;
pub const HASH_KEY_MASK: u64 = 0b1111111111111111;

pub static mut HASH_TABLE: [HashEntry; HASH_TABLE_SIZE] = [HASH_ENTRY_NONE; HASH_TABLE_SIZE];

lazy_static! {
    pub static ref BOARD_HASH: [[[u64; 9]; 9]; 29] = {
        let mut m: [[[u64; 9]; 9]; 29] = [[[0; 9]; 9]; 29];
        for piece in 0..29 {
            for i in 0..9 {
                for j in 0..9 {
                    m[piece][i][j] = rand::thread_rng().gen_range(0, u64::max_value());
                }
            }
        }
        m
    };
    pub static ref HAND_HASH: [[u64; 8]; 2] = {
        let mut m: [[u64; 8]; 2] = [[0; 8]; 2];
        for color in 0..2 {
            for kind in 0..8 {
                m[color][kind] = rand::thread_rng().gen_range(0, u64::max_value());
            }
        }
        m
    };
}
