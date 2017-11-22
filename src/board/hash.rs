extern crate rand;
use std::collections::HashMap;
use std::sync::RwLock;

use std::u64;
use self::rand::Rng;
use board::move_encode::*;

pub struct HashValue {
    pub color: bool, // is_black
    pub value: i32,
    pub remain_depth: u8,
    pub best_move: Move
}

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
    pub static ref HASH_TABLE: RwLock<HashMap<u64, HashValue>> = {
        let mut m = HashMap::new();
        RwLock::new(m)
    };
}

// static mut HASH_TABLE: HashMap<u64, HashValue> = HashMap::new();
