use std::i32;
use std::cmp;

use board::move_encode::*;
use board::state::*;
use board::eval::*;
use board::hash::*;
use std::collections::HashMap;

pub fn sub_search(ref mut state: &State, depth: u8, alpha: i32, beta: i32, mut hash_table: &mut HashMap<u64, HashValue>) -> (i32, Move) {
    if hash_table.contains_key(&state.hash_key) {
        let hash_val = &hash_table[&state.hash_key];
        if (state.color == hash_val.color && depth == hash_val.remain_depth) {
            return (hash_val.value, hash_val.best_move);
        }
    }
    let mut best_pair;
    if depth == 0 {
        best_pair = (eval(&state), NULL_MOVE)
    } else {
        let moves = state.legal_move();
        best_pair = (-(i32::max_value()), NULL_MOVE);
        for mv in moves {
            let mut new_state = state.clone();
            new_state.apply_move(&mv);
            new_state.change_color();
            let new_pair = sub_search(&new_state, depth - 1, -beta, -cmp::max(alpha, best_pair.0), &mut hash_table);
            let new_pair = (-new_pair.0, new_pair.1);
            if new_pair.0 > best_pair.0 {
                best_pair = (new_pair.0, mv);
            }
            if best_pair.0 >= beta {
                break;
            }
        }
    }
    let hash_val: HashValue = HashValue {
        color: state.color,
        value: best_pair.0,
        remain_depth: depth,
        best_move: best_pair.1,
    };
    hash_table.insert(state.hash_key, hash_val);
    best_pair
}
pub fn search(ref mut state: &State, depth: u8) -> (i32, Move) {
    let mut hash_table: HashMap<u64, HashValue> = HashMap::new();
    sub_search(&state, depth, -(i32::max_value()), i32::max_value(), &mut hash_table)
}
