use std::i32;
use std::cmp;

use board::move_encode::*;
use board::state::*;
use board::static_search::*;
use board::hash::*;

pub fn sub_search(ref mut state: &mut State, depth: u8, alpha: i32, beta: i32) -> i32 {
    let mut first_move = NULL_MOVE;
    let mut entry;
    unsafe {
        entry = HASH_TABLE[(state.hash_key & HASH_KEY_MASK) as usize];
    }
    if state.hash_key == entry.hash_key && state.color == entry.color {
        if depth <= entry.remain_depth {
            return entry.value;
        } else {
            first_move = entry.best_move;
        }
    }

    let mut best_val = -(i32::max_value());
    let mut best_move = NULL_MOVE;
    if depth == 0 {
        best_val = static_search(state);
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
        for mv in moves {
            state.apply_move(&mv);
            let new_val = -sub_search(state, depth - 1, -beta, -cmp::max(alpha, best_val));
            state.undo_move(&mv);
            if new_val > best_val{
                best_val = new_val;
                best_move = mv;
            }
            if best_val >= beta {
                break;
            }
        }
    }
    let new_entry: HashEntry = HashEntry {
        hash_key: state.hash_key,
        color: state.color,
        value: best_val,
        remain_depth: depth,
        best_move: best_move,
    };
    unsafe {
        HASH_TABLE[(state.hash_key & HASH_KEY_MASK) as usize] = new_entry;
    }
    best_val
}
pub fn search(ref mut state: &mut State, depth: u8) -> i32 {
    sub_search(state, depth, -(i32::max_value()), i32::max_value())
}
