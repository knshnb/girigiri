use board::state::*;
use board::position::*;
use board::move_encode::*;
use board::movable::*;
use board::piece::*;

impl State {
    pub fn opponent_king_is_capturable(&mut self) -> bool {
        let opponent_king = if self.color { Piece::king } else { Piece::King };
        let moves = self.legal_move();
        for mv in moves {
            if self.board[mv.to_pos()] == opponent_king { return true; }
        }
        false
    }

    pub fn legal_move_no_drop(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for &from in Position::variants() {
            let piece = self.board[from];
            if !piece.is_mine(self.color) { continue; }
            // short
            for &dir in SHORT_MOVABLE[piece].iter() {
                let next = from.step(dir);
                if next.is_none() { continue; }
                let to = next.unwrap();
                if self.board[to].is_mine(self.color) { continue; }
                moves.push(Move::normal_encode(from, to));
                if piece.can_promote() && (from.enemy_line(self.color) || to.enemy_line(self.color)) {
                    moves.push(Move::promote_encode(from, to));
                }
            }
            // long
            for &dir in LONG_MOVABLE[piece].iter() {
                let mut to = from;
                loop {
                    let next = to.step(dir);
                    if next.is_none() { break; }
                    to = next.unwrap();
                    if self.board[to].is_mine(self.color) { break; }
                    moves.push(Move::normal_encode(from, to));
                    if piece.can_promote() && (from.enemy_line(self.color) || to.enemy_line(self.color)) {
                        moves.push(Move::promote_encode(from, to));
                    }
                    if self.board[to] != Piece::null { break; }
                }
            }
        }
        moves
    }

    pub fn legal_move(&self) -> Vec<Move> {
        let mut moves = self.legal_move_no_drop();
        // drop
        for kind in 0..7 {
            if !self.hand[self.color as usize].own(kind) { continue; }
            for &to in Position::variants() {
                if self.board[to] != Piece::null { continue; }
                moves.push(Move::drop_encode(kind, to));
            }
        }
        moves
    }
}
