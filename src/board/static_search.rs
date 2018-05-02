use std::i32;
use std::cmp::max;
use board::state::*;
use board::eval::*;

fn sub_static_search(ref mut state: &mut State, to_i: i8, to_j: i8) -> i32 {
    let mut best_value = -i32::max_value();
    let mut is_finish = true;
    let moves = state.legal_move_no_drop();
    for mv in moves {
        if mv.to_i() == to_i && mv.to_j() == to_j {
            is_finish = false;
            state.apply_move(&mv);
            best_value = max(best_value, -sub_static_search(state, to_i, to_j));
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
    let mut best_value = eval(&state);
    let moves = state.legal_move_no_drop();
    for mv in moves {
        if state.is_capture(&mv) || state.is_pawn_promote(&mv) {
            state.apply_move(&mv);
            best_value = max(best_value, -sub_static_search(state, mv.to_i(), mv.to_j()));
            state.undo_move(&mv);
        }
    }
    best_value
}
