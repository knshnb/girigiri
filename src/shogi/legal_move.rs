use shogi::state::*;
use shogi::position::*;
use shogi::move_encode::*;
use shogi::movable::*;
use shogi::piece::*;

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
                // オーダリングのために成りの手を先に生成
                // last3段目に入れば成れる
                if piece.can_promote() && (from.last_lines(self.color, 3) || to.last_lines(self.color, 3)) {
                    moves.push(Move::promote_encode(from, to));
                }
                // 成らない指し手（歩、香車、桂馬：行き所のない指し手は生成しない）
                if !((piece.is(Piece::pawn) || piece.is(Piece::lance)) && to.last_lines(self.color, 1)) &&
                    !(piece.is(Piece::knight) && to.last_lines(self.color, 2)) { 
                    moves.push(Move::normal_encode(from, to));
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
                    // オーダリングのために成りの手を先に生成
                    // last3段目に入れば成れる
                    if piece.can_promote() && (from.last_lines(self.color, 3) || to.last_lines(self.color, 3)) {
                        moves.push(Move::promote_encode(from, to));
                    }
                    // 成らない指し手（香車：行き所のない指し手は生成しない）
                    if !(piece.is(Piece::lance) && to.last_lines(self.color, 1)) {
                        moves.push(Move::normal_encode(from, to));
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
                // 二歩判定
                if kind == 0 && self.pawn_checker[self.color as usize][to.column()] { continue; }
                moves.push(Move::drop_encode(kind, to));
            }
        }
        moves
    }
}
