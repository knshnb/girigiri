extern crate rand;

use std;
use std::cmp::Ordering;
use std::u64;
use self::rand::Rng;
use shogi::move_encode::*;

// Moveと16ビットの評価値をまとめた合計32ビットの構造体
#[derive(Debug, Clone, Copy, Eq)]
pub struct MoveValue {
    pub mv: Move,
    pub value: i16,
}

impl Ord for MoveValue {
    fn cmp(&self, other: &MoveValue) -> Ordering {
        self.value.cmp(&other.value)
    }
}
impl PartialOrd for MoveValue {
    fn partial_cmp(&self, other: &MoveValue) ->  Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for MoveValue {
    fn eq(&self, other: &MoveValue) ->  bool {
        self.value == other.value
    }
}
impl std::ops::Neg for MoveValue {
    type Output = MoveValue;
    fn neg(self) -> MoveValue {
        MoveValue {
            mv: self.mv,
            value: -self.value,
        }
    }
}
pub const NULL_MOVE_VALUE: MoveValue = MoveValue {
        mv: NULL_MOVE,
        value: -(i16::max_value()),
};

#[derive(Copy, Clone)]
pub struct HashEntry {
    pub hash_key: u64,
    pub color: bool, // is_black
    pub move_value: MoveValue,
    pub remain_depth: u8,
}

pub const HASH_ENTRY_NONE: HashEntry = HashEntry {
    hash_key: 0,
    color: false,
    move_value: NULL_MOVE_VALUE,
    remain_depth: 0,
};

// PCのメモリに応じて設定
const HASH_SHIFT_SIZE: u32 = 16;
pub const HASH_TABLE_SIZE: usize = 1 << HASH_SHIFT_SIZE;
lazy_static! {
    pub static ref HASH_KEY_MASK: u64 = 2u64.pow(HASH_SHIFT_SIZE) - 1;
}

pub static mut HASH_TABLE: [HashEntry; HASH_TABLE_SIZE] = [HASH_ENTRY_NONE; HASH_TABLE_SIZE];

lazy_static! {
    pub static ref BOARD_HASH: [[u64; 81]; 32] = {
        let mut m: [[u64; 81]; 32] = [[0; 81]; 32];
        for piece in 0..32 {
            for pos in 0..81 {
                m[piece][pos] = rand::thread_rng().gen_range(0, u64::max_value());
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

#[test]
fn mask_is_fine() {
    for i in 0..HASH_SHIFT_SIZE {
        let bit: u64 = 1 << i;
        assert!(*HASH_KEY_MASK & bit == bit, "HASH_KEY_MASK is too short");
    }
    let bit: u64 = 1 << (HASH_SHIFT_SIZE + 1);
    assert!(*HASH_KEY_MASK & bit != bit, "HASH_KEY_MASK is too long");
}
