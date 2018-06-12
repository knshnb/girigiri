use std::i16;
use std::cmp;
use shogi::state::*;
use shogi::position::*;
use engine::alphabeta::eval::*;

fn sub_static_search(ref mut state: &mut State, to: Position) -> i16 {
    let mut best_value = -i16::max_value();
    let moves = state.legal_move_no_drop();
    for mv in moves {
        if mv.to_pos() == to {
            state.apply_move(&mv);
            if state.opponent_king_is_capturable() {
                // 王手無視
                state.undo_move(&mv);
                continue;
            }
            best_value = cmp::max(best_value, sub_static_search(state, to));
            state.undo_move(&mv);
        }
    }
    -cmp::max(best_value, eval(&state))
}

pub fn static_search(ref mut state: &mut State) -> i16 {
    let mut best_value = -i16::max_value();
    let moves = state.legal_move_no_drop();
    for mv in moves {
        if state.is_capture(&mv) || state.is_pawn_promote(&mv) {
            state.apply_move(&mv);
            if state.opponent_king_is_capturable() {
                // 王手無視
                state.undo_move(&mv);
                continue;
            }
            best_value = cmp::max(best_value, sub_static_search(state, mv.to_pos()));
            state.undo_move(&mv);
        }
    }
    cmp::max(eval(&state), best_value)
}