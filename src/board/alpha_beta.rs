use std::i32;
use std::cmp;

use board::move_encode::*;
use board::state::*;
use board::eval::*;
use board::hash::*;

pub fn sub_search(ref mut state: &State, depth: u8, alpha: i32, beta: i32) -> (i32, Move) {
    let mut first_move = NULL_MOVE;
    unsafe {
        let entry = HASH_TABLE[(state.hash_key & HASH_KEY_MASK) as usize];
        if state.hash_key == entry.hash_key && state.color == entry.color {
            if depth == entry.remain_depth {
                return (entry.value, entry.best_move);
            } else {
                first_move = entry.best_move;
            }
        }
    }
    let mut best_pair;
    if depth == 0 {
        best_pair = (eval(&state), NULL_MOVE)
    } else {
        let mut moves = state.legal_move();
        if first_move != NULL_MOVE {
            for mv_index in 0..moves.len() {
                if moves[mv_index] == first_move {
                    // swap
                    moves[mv_index] = moves[0];
                    moves[0] = first_move;
                    break;
                }
            }
        }
        best_pair = (-(i32::max_value()), NULL_MOVE);
        for mv in moves {
            let mut new_state = state.clone();
            new_state.apply_move(&mv);
            new_state.change_color();
            let new_pair = sub_search(&new_state, depth - 1, -beta, -cmp::max(alpha, best_pair.0));
            let new_pair = (-new_pair.0, new_pair.1);
            if new_pair.0 > best_pair.0 {
                best_pair = (new_pair.0, mv);
            }
            if best_pair.0 >= beta {
                break;
            }
        }
    }
    let new_entry: HashEntry = HashEntry {
        hash_key: state.hash_key,
        color: state.color,
        value: best_pair.0,
        remain_depth: depth,
        best_move: best_pair.1,
    };
    unsafe {
        HASH_TABLE[(state.hash_key & HASH_KEY_MASK) as usize] = new_entry;
    }
    best_pair
}
pub fn search(ref mut state: &State, depth: u8) -> (i32, Move) {
    sub_search(&state, depth, -(i32::max_value()), i32::max_value())
}
