use std::i16;
use std::cmp;

use shogi::move_encode::*;
use shogi::hash::*;
use engine::alphabeta::controller::*;
use engine::alphabeta::static_search::*;

// 時間切れの時はNone, それ以外はSome(MoveValue)を返す
pub fn sub_search(ref mut engine: &mut AlphaBetaEngine, depth: u8, alpha: i16, beta: i16) -> Option<MoveValue> {
    // 時間切れ
    if engine.instant.elapsed() >= engine.time_limit { return None; }
    let mut first_move = NULL_MOVE;

    // ハッシュテーブルを確認
    let entry;
    unsafe {
        entry = HASH_TABLE[(engine.state.hash_key & *HASH_KEY_MASK) as usize];
    }
    if engine.state.hash_key == entry.hash_key && engine.state.color == entry.color {
        if depth <= entry.remain_depth {
            // 残り深さが深い場合は問答無用にその値を返す
            return Some(entry.move_value);
        } else {
            // 浅い深さの結果はオーダリングに使用
            first_move = entry.move_value.mv;
        }
    }

    let mut best = NULL_MOVE_VALUE;
    if depth == 0 {
        // best.value = engine.state.weight;
        best.value = static_search(&mut engine.state);
    } else {
        let mut moves = engine.state.legal_move();
        // 浅い探索での最善手を先頭に並び変える
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
            let new = sub_search(engine, depth - 1, -beta, -cmp::max(alpha, best.value));
            engine.state.undo_move(&mv);
            // 時間切れ
            if new.is_none() { return None; }
            let new = -(new.unwrap());
            if new >= best {
                best = MoveValue {
                    mv: mv,
                    value: new.value,
                }
            }
            if best.value >= beta {
                break;
            }
        }
        let new_entry: HashEntry = HashEntry {
            hash_key: engine.state.hash_key,
            color: engine.state.color,
            move_value: best,
            remain_depth: depth,
        };
        unsafe {
            HASH_TABLE[(engine.state.hash_key & *HASH_KEY_MASK) as usize] = new_entry;
        }
    }
    Some(best)
}

pub fn search(ref mut engine: &mut AlphaBetaEngine, depth: u8) -> Option<MoveValue> {
    sub_search(engine, depth, -(i16::max_value()), i16::max_value())
}