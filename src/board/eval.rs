use board::color::*;
use board::state::*;
use std::fs::*;
use std::io::*;

type PPsType = [[[[f32; 17]; 9]; 15]; 15];
type PPoType = [[[[f32; 17]; 17]; 15]; 15];

pub fn read_pps() -> PPsType {
    let mut f = File::open("eval_bin/pps.bin").expect("Cannot open pps");
    let mut buf = [0; 17 * 9 * 15 * 15 * 4];
    f.read_exact(&mut buf).expect("Cannot read pps");
    unsafe { ::std::mem::transmute::<[u8; 17 * 9 * 15 * 15 * 4], PPsType>(buf) }
}

pub fn read_ppo() -> PPoType {
    let mut f = File::open("eval_bin/ppo.bin").expect("Cannot open ppo");
    let mut buf = [0; 17 * 17 * 15 * 15 * 4];
    f.read_exact(&mut buf).expect("Cannot read ppo");
    unsafe { ::std::mem::transmute::<[u8; 17 * 17 * 15 * 15 * 4], PPoType>(buf) }
}

pub fn write_pps(pps: &mut PPsType) {
    let mut f = File::create("eval_bin/pps.bin").expect("Cannot create pps");
    let pps = unsafe { ::std::mem::transmute::<PPsType, [u8; 17 * 9 * 15 * 15 * 4]>(*pps) };
    f.write_all(&pps[..]).expect("Cannot write in pps");
}

pub fn write_ppo(ppo: &mut PPoType) {
    let mut f = File::create("eval_bin/ppo.bin").expect("Cannot create ppo");
    let ppo = unsafe { ::std::mem::transmute::<PPoType, [u8; 17 * 17 * 15 * 15 * 4]>(*ppo) };
    f.write_all(&ppo[..]).expect("Cannot write in ppo");
}

pub const PIECE_TO_WEIGHT: [i32; 32] = [
    // white
    86 * 9 / 10,    // pawn
    227 * 9 / 10,   // lance
    256 * 9 / 10,   // knight
    365 * 9 / 10,   // silver
    563 * 9 / 10,   // bishop
    629 * 9 / 10,   // rook
    439 * 9 / 10,   // gold
    15000 * 9 / 10, // king
    540 * 9 / 10,   // propawn
    508 * 9 / 10,   // prolance
    517 * 9 / 10,   // proknight
    502 * 9 / 10,   // prosilver
    826 * 9 / 10,   // horse
    942 * 9 / 10,   // dragon
    0,              // padding
    0 * 9 / 10,     // null
    // black
    86 * 9 / 10,    // pawn
    227 * 9 / 10,   // lance
    256 * 9 / 10,   // knight
    365 * 9 / 10,   // silver
    563 * 9 / 10,   // bishop
    629 * 9 / 10,   // rook
    439 * 9 / 10,   // gold
    15000 * 9 / 10, // king
    540 * 9 / 10,   // propawn
    508 * 9 / 10,   // prolance
    517 * 9 / 10,   // proknight
    502 * 9 / 10,   // prosilver
    826 * 9 / 10,   // horse
    942 * 9 / 10,   // dragon
    0,              // padding
    0,              // padding
];

pub const KIND_TO_WEIGHT: [i32; 8] = [
    86,    // pawn
    227,   // lance
    256,   // knight
    365,   // silver
    563,   // bishop
    629,   // rook
    439,   // gold
    15000, // king
];

pub struct Evaluator {
    pps: PPsType,
    ppo: PPoType,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            pps: read_pps(),
            ppo: read_ppo(),
        }
    }

    pub fn save_pps(&mut self) {
        write_pps(&mut self.pps);
    }

    pub fn save_ppo(&mut self) {
        write_ppo(&mut self.ppo);
    }

    pub fn eval(&self, state: &State) -> f32 {
        let (mut mine, mut yours) = (Vec::new(), Vec::new());
        for i in 0..9 {
            for j in 0..9 {
                match (*state).board[i][j].whose() {
                    Color::Black => {
                        let dst = if (*state).color {
                            &mut yours
                        } else {
                            &mut mine
                        };
                        (*dst).push(((*state).board[i][j].to_white(), i, j));
                    }
                    Color::White => {
                        let dst = if (*state).color {
                            &mut mine
                        } else {
                            &mut yours
                        };
                        (*dst).push(((*state).board[i][j].to_white(), i, j));
                    }
                    _ => (),
                }
            }
        }
        let mut sum: f32 = 0.0;
        for (i, &a) in mine.iter().enumerate() {
            for &b in &mine[(i + 1)..] {
                let ((ap, ai, aj), (bp, bi, bj)) = if a.2 < b.2 { (a, b) } else { (b, a) };
                if state.color {
                    sum += self.pps[ap as usize][bp as usize][bj - aj][bi + 8 - ai];
                } else {
                    sum += self.pps[ap as usize][bp as usize][bj - aj][ai + 8 - bi];
                }
            }
        }
        for (i, &a) in yours.iter().enumerate() {
            for &b in &yours[(i + 1)..] {
                let ((ap, ai, aj), (bp, bi, bj)) = if a.2 < b.2 { (a, b) } else { (b, a) };
                if state.color {
                    sum -= self.pps[bp as usize][ap as usize][bj - aj][ai + 8 - bi];
                } else {
                    sum -= self.pps[bp as usize][ap as usize][bj - aj][bi + 8 - ai];
                }
            }
        }
        for (&(ap, ai, aj), &(bp, bi, bj)) in mine.iter().zip(yours.iter()) {
            if state.color {
                sum += self.ppo[ap as usize][bp as usize][bj + 8 - aj][bi + 8 - ai];
            } else {
                sum += self.ppo[ap as usize][bp as usize][aj + 8 - bj][ai + 8 - bi];
            }
        }
        sum
    }

    pub fn update(&mut self, state: &State, pi: usize, pj: usize) {
        // 学習時、pは先手のみ
        let p = ((*state).board[pi][pj].to_white(), pi, pj);
        let (mut mine, mut yours) = (Vec::new(), Vec::new());
        for i in 0..9 {
            for j in 0..9 {
                match (*state).board[i][j].whose() {
                    Color::Black => {
                        let dst = if (*state).color {
                            &mut yours
                        } else {
                            &mut mine
                        };
                        (*dst).push(((*state).board[i][j].to_white(), i, j));
                    }
                    Color::White => {
                        let dst = if (*state).color {
                            &mut mine
                        } else {
                            &mut yours
                        };
                        (*dst).push(((*state).board[i][j].to_white(), i, j));
                    }
                    _ => (),
                }
            }
        }

        for &a in mine.iter() {
            let ((ap, ai, aj), (pp, pi, pj)) = if a.2 < p.2 { (a, p) } else { (p, a) };
            if ai != pi || aj != pj {
                self.pps[ap as usize][pp as usize][pj - aj][pi + 8 - ai] += 1.0;
            }
        }

        for &(ap, ai, aj) in yours.iter() {
            self.ppo[p.0 as usize][ap as usize][aj + 8 - pj][ai + 8 - pi] += 1.0;
        }
    }

    pub fn normalize(&mut self, turns: i32) {
        for i in 0..9 {
            for j in 0..17 {
                for k in 0..15 {
                    for l in 0..15 {
                        self.pps[k][l][i][j] -= 1.0 / turns as f32;
                    }
                }
            }
        }

        for i in 0..17 {
            for j in 0..17 {
                for k in 0..15 {
                    for l in 0..15 {
                        self.ppo[k][l][i][j] -= 1.0 / turns as f32;
                    }
                }
            }
        }
    }
}

pub fn eval(state: &State) -> i32 {
    (*state).weight
}
