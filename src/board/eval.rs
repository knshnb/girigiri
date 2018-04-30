use board::state::*;
use std::fs::*;
use std::io::*;

type PPsType = [[[[i32; 9]; 9]; 14]; 14];
type PPoType = [[[[i32; 17]; 17]; 14]; 14];

pub fn read_pps() -> PPsType {
    let mut f = File::open("pps.bin").unwrap();
    let mut buf = [0; 9 * 9 * 14 * 14 * 4];
    f.read_exact(&mut buf).unwrap();
    unsafe {
        ::std::mem::transmute::<[u8; 9 * 9 * 14 * 14 * 4], PPsType>(buf)
    }
}

pub fn read_ppo() -> PPoType {
    let mut f = File::open("ppo.bin").unwrap();
    let mut buf = [0; 17 * 17 * 14 * 14 * 4];
    f.read_exact(&mut buf).unwrap();
    unsafe {
        ::std::mem::transmute::<[u8; 17 * 17 * 14 * 14 * 4], PPoType>(buf)
    }
}

pub fn write_pps(pps: &mut PPsType) {
    let mut f = File::create("pps.bin").unwrap();
    let pps = unsafe {
        ::std::mem::transmute::<PPsType, [u8; 9 * 9 * 14 * 14 * 4]>(*pps)
    };
    f.write_all(&pps[..]).unwrap();
}

pub fn write_ppo(ppo: &mut PPoType) {
    let mut f = File::create("ppo.bin").unwrap();
    let ppo = unsafe {
        ::std::mem::transmute::<PPoType, [u8; 17 * 17 * 14 * 14 * 4]>(*ppo)
    };
    f.write_all(&ppo[..]).unwrap();
}

pub const PIECE_TO_WEIGHT: [i32; 32] = [
    // white
    86 * 9 / 10,     // pawn
    227 * 9 / 10,    // lance
    256 * 9 / 10,    // knight
    365 * 9 / 10,    // silver
    563 * 9 / 10,    // bishop
    629 * 9 / 10,    // rook
    439 * 9 / 10,    // gold
    15000 * 9 / 10,  // king

    540 * 9 / 10,    // propawn
    508 * 9 / 10,    // prolance
    517 * 9 / 10,    // proknight
    502 * 9 / 10,    // prosilver
    826 * 9 / 10,    // horse
    942 * 9 / 10,    // dragon
    0,               // padding
    0   * 9 / 10,    // null
    // black
    86 * 9 / 10,     // pawn
    227 * 9 / 10,    // lance
    256 * 9 / 10,    // knight
    365 * 9 / 10,    // silver
    563 * 9 / 10,    // bishop
    629 * 9 / 10,    // rook
    439 * 9 / 10,    // gold
    15000 * 9 / 10,  // king

    540 * 9 / 10,    // propawn
    508 * 9 / 10,    // prolance
    517 * 9 / 10,    // proknight
    502 * 9 / 10,    // prosilver
    826 * 9 / 10,    // horse
    942 * 9 / 10,    // dragon
    0,               // padding
    0,               // padding
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
