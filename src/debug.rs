#[macro_use]
extern crate lazy_static;

use std::time::Instant;
use std::io;
use std::io::Read;
use std::str::FromStr;

mod board;
use board::state::*;
use board::move_encode::*;

fn main() {
    let mut state = State::new();
    let mut mv = NULL_MOVE;
    loop {
        state.print_debug();
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("failed to read line");
        if input.len() == 2 {
            // undo
            state.print_move(&mv);
            state.undo_move(&mv);
        } else {
            let to_j: i8 = 9 - (input.chars().nth(2).unwrap().to_digit(10).unwrap() as i8);
            let to_i: i8 = input.chars().nth(3).unwrap() as i8 - 'a' as i8;
            if input.chars().nth(1) == Some('*') {
                // drop
                let kind = match input.chars().nth(0).unwrap() {
                    'K' => 7,
                    'G' => 6,
                    'R' => 5,
                    'B' => 4,
                    'S' => 3,
                    'N' => 2,
                    'L' => 1,
                    'P' => 0,
                    _ => 7,
                };
                mv = board::move_encode::Move::drop_encode(kind, to_i, to_j);
            } else {
                let from_j: i8 = 9 - (input.chars().nth(0).unwrap().to_digit(10).unwrap() as i8);
                let from_i: i8 = input.chars().nth(1).unwrap() as i8 - 'a' as i8;
                if input.len() == 6 {
                    // promote
                    mv = board::move_encode::Move::promote_encode(from_i, from_j, to_i, to_j);
                } else if input.len() == 5 {
                    // normal
                    mv = board::move_encode::Move::normal_encode(from_i, from_j, to_i, to_j);
                }
            }
            state.print_move(&mv);
            state.apply_move(&mv);
        }
    }
}
