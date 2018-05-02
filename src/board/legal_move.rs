use board::color::*;
use board::state::*;
use board::move_encode::*;
use board::piece::*;
use board::rules::MOVABLES;

impl State {
    pub fn opponent_king_is_capturable(&mut self) -> bool {
        let opponent_king = if self.color { Piece::king } else { Piece::King };
        let moves = self.legal_move();
        for mv in moves {
            if self.board[mv.to_i() as usize][mv.to_j() as usize] == opponent_king { return true; }
        }
        false
    }

    pub fn legal_move_no_drop(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        if self.color {
            for from_i in 0..9 {
                for from_j in 0..9 {
                    match self.board[from_i][from_j] {
                        // normal move
                        Piece::Pawn =>
                            for &(diff_i, diff_j) in MOVABLES.pawn_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    if to_i <= 2 {
                                        // always promote when possible
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    } else {
                                        moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                }
                            }
                        Piece::Knight =>
                            for &(diff_i, diff_j) in MOVABLES.knight_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    if to_i <= 2 {
                                        // can promote
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                    if to_i >= 2 {
                                        // must promote in the other cases
                                        moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                }
                            }
                        Piece::Silver =>
                            for &(diff_i, diff_j) in MOVABLES.silver_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    if to_i <= 2 || from_i <= 2 {
                                        // can promote
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        Piece::Propawn | Piece::Prolance | Piece::Proknight | Piece::Prosilver | Piece::Gold =>
                            for &(diff_i, diff_j) in MOVABLES.gold_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        Piece::King =>
                            for &(diff_i, diff_j) in MOVABLES.king_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        // long move
                        Piece::Lance =>
                            for &(diff_i, diff_j) in MOVABLES.lance_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                else {
                                    match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                        Color::Black => break,
                                        Color::White => {
                                            if to_i <= 2 {
                                                // can promote
                                                moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            if to_i >= 2 {
                                                // must promote in the other cases
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            break;
                                        },
                                        Color::Null => {
                                            if to_i <= 2 {
                                                // can promote
                                                moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            if to_i >= 2 {
                                                // must promote in the other cases
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                        },
                                    }
                                }
                            }
                        Piece::Bishop => // bishop
                            for &bishop_line in MOVABLES.bishop_black.iter() {
                                for &(diff_i, diff_j) in bishop_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::Black => break,
                                            Color::White => {
                                                if to_i <= 2 || from_i <= 2 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                                break;
                                            },
                                            Color::Null => {
                                                if to_i <= 2 || from_i <= 2 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        Piece::Rook => // rook
                            for &rook_line in MOVABLES.rook_black.iter() {
                                for &(diff_i, diff_j) in rook_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::Black => break,
                                            Color::White => {
                                                if to_i <= 2 || from_i <= 2 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                                break;
                                            },
                                            Color::Null => {
                                                if to_i <= 2 || from_i <= 2 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        Piece::Horse => // horse
                            // long move
                        {
                            for &bishop_line in MOVABLES.bishop_black.iter() {
                                for &(diff_i, diff_j) in bishop_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::Black => break,
                                            Color::White => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                break;
                                            },
                                            Color::Null => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            },
                                        }
                                    }
                                }
                            }
                            // normal move
                            for &(diff_i, diff_j) in MOVABLES.normal_horse_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        }
                        Piece::Dragon => // dragon
                            // long move
                        {
                            for &rook_line in MOVABLES.rook_black.iter() {
                                for &(diff_i, diff_j) in rook_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::Black => break,
                                            Color::White => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                break;
                                            },
                                            Color::Null => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            },
                                        }
                                    }
                                }
                            }
                            // normal move
                            for &(diff_i, diff_j) in MOVABLES.normal_dragon_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        }
                        _   => ()
                    }
                }
            }
        } else {
            for from_i in 0..9 {
                for from_j in 0..9 {
                    match self.board[from_i][from_j] {
                        // normal move
                        Piece::pawn =>
                            for &(diff_i, diff_j) in MOVABLES.pawn_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    if to_i >= 6 {
                                        // always promote when possible
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    } else {
                                        moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                }
                            }
                        Piece::knight =>
                            for &(diff_i, diff_j) in MOVABLES.knight_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    if to_i >= 6 {
                                        // can promote
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                    if to_i <= 6 {
                                        // must promote in the other cases
                                        moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                }
                            }
                        Piece::silver =>
                            for &(diff_i, diff_j) in MOVABLES.silver_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    if to_i >= 6 || from_i >= 6 {
                                        // can promote
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        Piece::propawn | Piece::prolance | Piece::proknight | Piece::prosilver | Piece::gold =>
                            for &(diff_i, diff_j) in MOVABLES.gold_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        Piece::king =>
                            for &(diff_i, diff_j) in MOVABLES.king_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        // long move
                        Piece::lance =>
                            for &(diff_i, diff_j) in MOVABLES.lance_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                else {
                                    match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                        Color::White => break,
                                        Color::Black => {
                                            if to_i >= 6 {
                                                // can promote
                                                moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            if to_i <= 6 {
                                                // must promote in the other cases
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            break;
                                        },
                                        Color::Null => {
                                            if to_i >= 6 {
                                                // can promote
                                                moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            if to_i <= 6 {
                                                // must promote in the other cases
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                        },
                                    }
                                }
                            }
                        Piece::bishop => // bishop
                            for &bishop_line in MOVABLES.bishop_white.iter() {
                                for &(diff_i, diff_j) in bishop_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::White => break,
                                            Color::Black => {
                                                if to_i >= 6 || from_i >= 6 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                                break;
                                            },
                                            Color::Null => {
                                                if to_i >= 6 || from_i >= 6 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        Piece::rook => // rook
                            for &rook_line in MOVABLES.rook_white.iter() {
                                for &(diff_i, diff_j) in rook_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::White => break,
                                            Color::Black => {
                                                if to_i >= 6 || from_i >= 6 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                                break;
                                            },
                                            Color::Null => {
                                                if to_i >= 6 || from_i >= 6 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        Piece::horse => // horse
                            // long move
                        {
                            for &bishop_line in MOVABLES.bishop_white.iter() {
                                for &(diff_i, diff_j) in bishop_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::White => break,
                                            Color::Black => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                break;
                                            },
                                            Color::Null => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            },
                                        }
                                    }
                                }
                            }
                            // normal move
                            for &(diff_i, diff_j) in MOVABLES.normal_horse_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        }
                        Piece::dragon => // dragon
                            // long move
                        {
                            for &rook_line in MOVABLES.rook_white.iter() {
                                for &(diff_i, diff_j) in rook_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::White => break,
                                            Color::Black => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                break;
                                            },
                                            Color::Null => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            },
                                        }
                                    }
                                }
                            }
                            // normal move
                            for &(diff_i, diff_j) in MOVABLES.normal_dragon_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        }
                        _   => ()
                    }
                }
            }
        }
        moves
    }

    pub fn legal_move(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        // drop
        let mut captured_kinds = Vec::new();
        for n in 0..7 {
            if self.hand[self.color as usize].own(n) {
                captured_kinds.push(n);
            }
        }
        for to_i in 0..9 {
            for to_j in 0..9 {
                if self.board[to_i][to_j] == Piece::null {
                    for &n in &captured_kinds {
                        match n {
                            0 => if to_i >= 1 && !self.pawn_checker[self.color as usize][to_j] {
                                moves.push(Move::drop_encode(n as u8, to_i as i8, to_j as i8));
                            },
                            1 => if to_i >= 1 {
                                moves.push(Move::drop_encode(n as u8, to_i as i8, to_j as i8));
                            },
                            2 => if to_i >= 2 {
                                moves.push(Move::drop_encode(n as u8, to_i as i8, to_j as i8));
                            },
                            _ => moves.push(Move::drop_encode(n as u8, to_i as i8, to_j as i8)),
                        }
                    }
                }
            }
        }

        // not drop
        if self.color {
            for from_i in 0..9 {
                for from_j in 0..9 {
                    match self.board[from_i][from_j] {
                        // normal move
                        Piece::Pawn =>
                            for &(diff_i, diff_j) in MOVABLES.pawn_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    if to_i <= 2 {
                                        // always promote when possible
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    } else {
                                        moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                }
                            }
                        Piece::Knight =>
                            for &(diff_i, diff_j) in MOVABLES.knight_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    if to_i <= 2 {
                                        // can promote
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                    if to_i >= 2 {
                                        // must promote in the other cases
                                        moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                }
                            }
                        Piece::Silver =>
                            for &(diff_i, diff_j) in MOVABLES.silver_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    if to_i <= 2 || from_i <= 2 {
                                        // can promote
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        Piece::Propawn | Piece::Prolance | Piece::Proknight | Piece::Prosilver | Piece::Gold =>
                            for &(diff_i, diff_j) in MOVABLES.gold_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        Piece::King =>
                            for &(diff_i, diff_j) in MOVABLES.king_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        // long move
                        Piece::Lance =>
                            for &(diff_i, diff_j) in MOVABLES.lance_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                else {
                                    match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                        Color::Black => break,
                                        Color::White => {
                                            if to_i <= 2 {
                                                // can promote
                                                moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            if to_i >= 2 {
                                                // must promote in the other cases
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            break;
                                        },
                                        Color::Null => {
                                            if to_i <= 2 {
                                                // can promote
                                                moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            if to_i >= 2 {
                                                // must promote in the other cases
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                        },
                                    }
                                }
                            }
                        Piece::Bishop => // bishop
                            for &bishop_line in MOVABLES.bishop_black.iter() {
                                for &(diff_i, diff_j) in bishop_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::Black => break,
                                            Color::White => {
                                                if to_i <= 2 || from_i <= 2 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                                break;
                                            },
                                            Color::Null => {
                                                if to_i <= 2 || from_i <= 2 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        Piece::Rook => // rook
                            for &rook_line in MOVABLES.rook_black.iter() {
                                for &(diff_i, diff_j) in rook_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::Black => break,
                                            Color::White => {
                                                if to_i <= 2 || from_i <= 2 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                                break;
                                            },
                                            Color::Null => {
                                                if to_i <= 2 || from_i <= 2 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        Piece::Horse => // horse
                            // long move
                        {
                            for &bishop_line in MOVABLES.bishop_black.iter() {
                                for &(diff_i, diff_j) in bishop_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::Black => break,
                                            Color::White => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                break;
                                            },
                                            Color::Null => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            },
                                        }
                                    }
                                }
                            }
                            // normal move
                            for &(diff_i, diff_j) in MOVABLES.normal_horse_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        }
                        Piece::Dragon => // dragon
                            // long move
                        {
                            for &rook_line in MOVABLES.rook_black.iter() {
                                for &(diff_i, diff_j) in rook_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::Black => break,
                                            Color::White => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                break;
                                            },
                                            Color::Null => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            },
                                        }
                                    }
                                }
                            }
                            // normal move
                            for &(diff_i, diff_j) in MOVABLES.normal_dragon_black.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::Black {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        }
                        _   => ()
                    }
                }
            }
        } else {
            for from_i in 0..9 {
                for from_j in 0..9 {
                    match self.board[from_i][from_j] {
                        // normal move
                        Piece::pawn =>
                            for &(diff_i, diff_j) in MOVABLES.pawn_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    if to_i >= 6 {
                                        // always promote when possible
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    } else {
                                        moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                }
                            }
                        Piece::knight =>
                            for &(diff_i, diff_j) in MOVABLES.knight_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    if to_i >= 6 {
                                        // can promote
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                    if to_i <= 6 {
                                        // must promote in the other cases
                                        moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                }
                            }
                        Piece::silver =>
                            for &(diff_i, diff_j) in MOVABLES.silver_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    if to_i >= 6 || from_i >= 6 {
                                        // can promote
                                        moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                    }
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        Piece::propawn | Piece::prolance | Piece::proknight | Piece::prosilver | Piece::gold =>
                            for &(diff_i, diff_j) in MOVABLES.gold_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        Piece::king =>
                            for &(diff_i, diff_j) in MOVABLES.king_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        // long move
                        Piece::lance =>
                            for &(diff_i, diff_j) in MOVABLES.lance_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                else {
                                    match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                        Color::White => break,
                                        Color::Black => {
                                            if to_i >= 6 {
                                                // can promote
                                                moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            if to_i <= 6 {
                                                // must promote in the other cases
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            break;
                                        },
                                        Color::Null => {
                                            if to_i >= 6 {
                                                // can promote
                                                moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                            if to_i <= 6 {
                                                // must promote in the other cases
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            }
                                        },
                                    }
                                }
                            }
                        Piece::bishop => // bishop
                            for &bishop_line in MOVABLES.bishop_white.iter() {
                                for &(diff_i, diff_j) in bishop_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::White => break,
                                            Color::Black => {
                                                if to_i >= 6 || from_i >= 6 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                                break;
                                            },
                                            Color::Null => {
                                                if to_i >= 6 || from_i >= 6 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        Piece::rook => // rook
                            for &rook_line in MOVABLES.rook_white.iter() {
                                for &(diff_i, diff_j) in rook_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::White => break,
                                            Color::Black => {
                                                if to_i >= 6 || from_i >= 6 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                                break;
                                            },
                                            Color::Null => {
                                                if to_i >= 6 || from_i >= 6 {
                                                    // always promote when possible
                                                    moves.push(Move::promote_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                } else {
                                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        Piece::horse => // horse
                            // long move
                        {
                            for &bishop_line in MOVABLES.bishop_white.iter() {
                                for &(diff_i, diff_j) in bishop_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::White => break,
                                            Color::Black => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                break;
                                            },
                                            Color::Null => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            },
                                        }
                                    }
                                }
                            }
                            // normal move
                            for &(diff_i, diff_j) in MOVABLES.normal_horse_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        }
                        Piece::dragon => // dragon
                            // long move
                        {
                            for &rook_line in MOVABLES.rook_white.iter() {
                                for &(diff_i, diff_j) in rook_line.iter() {
                                    let to_i = (from_i as i8) + diff_i;
                                    let to_j = (from_j as i8) + diff_j;
                                    if to_i < 0 || 8 < to_i || to_j < 0 || 8 < to_j { break; }
                                    else {
                                        match Piece::whose(self.board[to_i as usize][to_j as usize]) {
                                            Color::White => break,
                                            Color::Black => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                                break;
                                            },
                                            Color::Null => {
                                                moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                            },
                                        }
                                    }
                                }
                            }
                            // normal move
                            for &(diff_i, diff_j) in MOVABLES.normal_dragon_white.iter() {
                                let to_i = (from_i as i8) + diff_i;
                                let to_j = (from_j as i8) + diff_j;
                                if 0 <= to_i && to_i <= 8 && 0 <= to_j && to_j <= 8 && Piece::whose(self.board[to_i as usize][to_j as usize]) != Color::White {
                                    moves.push(Move::normal_encode(from_i as i8, from_j as i8, to_i, to_j));
                                }
                            }
                        }
                        _   => ()
                    }
                }
            }
        }

        moves
    }
}
