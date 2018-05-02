use std::i32;
use std::cmp;

use board::move_encode::*;
use board::state::*;
use board::static_search::*;
use board::hash::*;
use engine::alpha_beta_engine::*;

pub fn sub_search(ref mut engine: &mut AlphaBetaEngine, depth: u8, alpha: i32, beta: i32) -> Option<i32> {
    if engine.instant.elapsed() >= engine.time_limit {
        return None;
    }
    let mut first_move = NULL_MOVE;
    let entry;
    unsafe {
        entry = HASH_TABLE[(engine.state.hash_key & HASH_KEY_MASK) as usize];
    }
    if engine.state.hash_key == entry.hash_key && engine.state.color == entry.color {
        if depth <= entry.remain_depth {
            return Some(entry.value);
        } else {
            first_move = entry.best_move;
        }
    }

    let mut best_val = -(i32::max_value());
    let mut best_move = NULL_MOVE;
    if depth == 0 {
        best_val = static_search(&mut engine.state);
        if (engine.use_pp) {
            best_val += (engine.evaluator.eval(&engine.state) * 0.1) as i32;
        }
    } else {
        let mut moves = engine.state.legal_move();
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
            engine.state.apply_move(&mv);
            if engine.state.opponent_king_is_capturable() {
                // 王手無視
                engine.state.undo_move(&mv);
                continue;
            }
            let new_val = sub_search(engine, depth - 1, -beta, -cmp::max(alpha, best_val));
            engine.state.undo_move(&mv);
            if new_val.is_none() {
                return None;
            }
            let new_val = -(new_val.unwrap());
            if new_val >= best_val {
                best_val = new_val;
                best_move = mv;
            }
            if best_val >= beta {
                break;
            }
        }
    }
    let new_entry: HashEntry = HashEntry {
        hash_key: engine.state.hash_key,
        color: engine.state.color,
        value: best_val,
        remain_depth: depth,
        best_move: best_move,
    };
    unsafe {
        HASH_TABLE[(engine.state.hash_key & HASH_KEY_MASK) as usize] = new_entry;
    }
    Some(best_val)
}

pub fn search(ref mut engine: &mut AlphaBetaEngine, depth: u8) -> Option<i32> {
    sub_search(engine, depth, -(i32::max_value()), i32::max_value())
}
