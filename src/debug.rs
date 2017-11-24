#[macro_use]
extern crate lazy_static;

use std::time::Instant;
use std::io;
use std::str::FromStr;

mod board;
use board::state::*;
use board::move_encode::*;

fn main() {
    let mut state = State::new();
    let mut mv = NULL_MOVE;
    loop {
        state.print_debug();
        println!("undo: 0, normal_move: 1, promote_move: 2, drop_move: 3");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("failed to read line");
        let input: u32 = input.trim().parse()
            .expect("not a number");
        match input {
            0 => {
                state.print_move(&mv);
                state.undo_move(&mv)
            },
            1 | 2 | 3 => {
                println!("move = ?");
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).ok();
                let mut iter = buf.split_whitespace().map(|n| i8::from_str(n).unwrap());

                if input == 3 {
                    // drop
                    let kind: u8 = iter.next().unwrap() as u8;
                    let to_j: i8 = 9 - iter.next().unwrap();
                    let to_i: i8 = iter.next().unwrap() - 1;
                    mv = board::move_encode::Move::drop_encode(kind, to_i, to_j);
                } else {
                    // not drop
                    let from_j: i8 = 9 - iter.next().unwrap();
                    let from_i: i8 = iter.next().unwrap() - 1;
                    let to_j: i8 = 9 - iter.next().unwrap();
                    let to_i: i8 = iter.next().unwrap() - 1;
                    if input == 1 { 
                        // normal
                        mv = board::move_encode::Move::normal_encode(from_i, from_j, to_i, to_j);
                    } else if input == 2 {
                        // promote
                        mv = board::move_encode::Move::promote_encode(from_i, from_j, to_i, to_j);
                    }
                }
                state.apply_move(&mv);
            },
            _ => println!("invalid input"),
        }
    }
}
