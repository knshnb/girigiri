use std::i32;
use std::cmp;
use board::state::*;
use board::eval::*;
use board::position::*;

fn sub_static_search(ref mut state: &mut State, to: Position) -> i32 {
    let mut best_value = -i32::max_value();
    let mut is_finish = true;
    let moves = state.legal_move_no_drop();
    for mv in moves {
        if mv.to_pos() == to {
            is_finish = false;
            state.apply_move(&mv);
            best_value = cmp::max(best_value, -sub_static_search(state, to));
            state.undo_move(&mv);
        }
    }
    if is_finish {
        eval(&state)
    } else {
        best_value
    }
}

pub fn static_search(ref mut state: &mut State) -> i32 {
    let mut best_value = -i32::max_value();
    let moves = state.legal_move_no_drop();
    for mv in moves {
        if state.is_capture(&mv) || state.is_pawn_promote(&mv) {
            state.apply_move(&mv);
            best_value = cmp::max(best_value, -sub_static_search(state, mv.to_pos()));
            state.undo_move(&mv);
        }
    }
    cmp::max(eval(&state), best_value)
}
