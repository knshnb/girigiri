use board::move_encode::*;
use board::piece::*;
use board::hand::*;
use board::hash::*;
use board::eval::*;
use board::past_captured_piece::*;
use std::fmt;

#[derive(Clone)]
pub struct State {
    pub nth: u16,
    pub color: bool,                    // true: black, false: white
    pub board: [[Piece; 9]; 9],
    pub hand: [Hand; 2],                // hand[0]: white, hand[1]: black
    pub pawn_checker: [[bool; 9]; 2],   // pawn_checker[0]: white, pawn_checker[1]: black
    pub hash_key: u64,
    pub weight: i32,
}

impl State {
    pub fn new() -> State {
        use self::Piece::*;
        State {
            nth: 0,
            color: true,
            board: // initial position
                [[lance, knight, silver, gold, king, gold, silver, knight, lance],
                 [null,  rook, null,  null,  null,  null,  null,  bishop, null ],
                 [pawn; 9],
                 [null; 9],
                 [null; 9],
                 [null; 9],
                 [Pawn; 9],
                 [null,  Bishop,  null,  null,  null,  null,  null,  Rook,  null ],
                 [Lance, Knight, Silver, Gold, King, Gold, Silver, Knight, Lance]],
            hand: [Hand::new(), Hand::new()],
            pawn_checker: [[true; 9]; 2],
            hash_key: 0,
            weight:0,
        }
    }

    pub fn is_lose(&self) -> bool {
        self.hand[!self.color as usize].own(7)
    }

    pub fn is_capture(&self, &mv: &Move) -> bool {
        self.board[mv.to_i() as usize][mv.to_j() as usize] != Piece::null
    }

    pub fn is_pawn_promote(&self, &mv: &Move) -> bool {
        if self.color {
            mv.is_promote() && self.board[mv.from_i() as usize][mv.from_j() as usize] == Piece::pawn        } else {
            mv.is_promote() && self.board[mv.from_i() as usize][mv.from_j() as usize] == Piece::Pawn
        }
    }

    pub fn print_expectation(&mut self, depth: u8) {
        let mut state = self.clone();
        for _ in 0..depth {
            let mv;
            unsafe {
                mv = HASH_TABLE[(state.hash_key & HASH_KEY_MASK) as usize].best_move;
            }
            state.print_move(&mv);
            state.apply_move(&mv);
        }
    }

    pub fn apply_move(&mut self, mv: &Move) {
        if mv.is_drop() {
            let drop_kind = mv.drop_kind();
            let drop_piece = Piece::new(drop_kind, self.color);
            self.hand[self.color as usize].sub(drop_kind);
            self.board[mv.to_i() as usize][mv.to_j() as usize] = drop_piece;
            if drop_kind == 0 { // pawn
                self.pawn_checker[self.color as usize][mv.to_j() as usize] = true;
            }
            // weight
            self.weight -= KIND_TO_WEIGHT[drop_kind as usize] / 10;
            // hash
            self.hash_key = self.hash_key.wrapping_add(BOARD_HASH[drop_piece as usize][mv.to_i() as usize][mv.to_j() as usize]);
            self.hash_key = self.hash_key.wrapping_sub(HAND_HASH[self.color as usize][drop_kind]);
        } else {
            let from_piece = self.board[mv.from_i() as usize][mv.from_j() as usize];
            self.board[mv.from_i() as usize][mv.from_j() as usize] = Piece::null;

            let to_piece = self.board[mv.to_i() as usize][mv.to_j() as usize];
            unsafe {
                PAST_CAPTURED_PIECES[self.nth as usize] = to_piece;
            }
            // capture
            if to_piece != Piece::null {
                let captured_kind = Piece::kind(to_piece);
                self.hand[self.color as usize].add(captured_kind);
                if to_piece == Piece::pawn || to_piece == Piece::Pawn {
                    self.pawn_checker[!self.color as usize][mv.to_j() as usize] = false;
                }
                // weight
                self.weight += PIECE_TO_WEIGHT[to_piece as usize];
                self.weight += KIND_TO_WEIGHT[captured_kind];
                // hash
                self.hash_key = self.hash_key.wrapping_sub(BOARD_HASH[to_piece as usize][mv.to_i() as usize][mv.to_j() as usize]);
                self.hash_key = self.hash_key.wrapping_add(HAND_HASH[self.color as usize][captured_kind]);
            }

            if mv.is_promote() {
                let promoted_piece = from_piece.promote();
                self.board[mv.to_i() as usize][mv.to_j() as usize] = promoted_piece;
                if from_piece == Piece::pawn || from_piece == Piece::Pawn {
                    self.pawn_checker[self.color as usize][mv.to_j() as usize] = false;
                }
                // weight
                self.weight += PIECE_TO_WEIGHT[promoted_piece as usize];
                self.weight -= PIECE_TO_WEIGHT[from_piece as usize];
                // hash
                self.hash_key = self.hash_key.wrapping_add(BOARD_HASH[promoted_piece as usize][mv.to_i() as usize][mv.to_j() as usize]);
            } else {
                // hash
                self.board[mv.to_i() as usize][mv.to_j() as usize] = from_piece;
                self.hash_key = self.hash_key.wrapping_add(BOARD_HASH[from_piece as usize][mv.to_i() as usize][mv.to_j() as usize]);
            }
            // hash
            self.hash_key = self.hash_key.wrapping_sub(BOARD_HASH[from_piece as usize][mv.from_i() as usize][mv.from_j() as usize]);
        }
        self.color = !self.color;
        self.weight = -self.weight;
        self.nth += 1;
    }

    pub fn undo_move(&mut self, mv: &Move) {
        self.nth -= 1;
        self.weight = -self.weight;
        self.color = !self.color;
        let to_piece = self.board[mv.to_i() as usize][mv.to_j() as usize];
        if mv.is_drop() {
            let drop_kind = mv.drop_kind();
            self.board[mv.to_i() as usize][mv.to_j() as usize] = Piece::null;
            self.hand[self.color as usize].add(drop_kind);
            if drop_kind == 0 { // pawn
                self.pawn_checker[self.color as usize][mv.to_j() as usize] = false;
            }
            // weight
            self.weight += KIND_TO_WEIGHT[drop_kind as usize] / 10;
            // hash
            self.hash_key = self.hash_key.wrapping_add(HAND_HASH[self.color as usize][drop_kind]);
        } else {
            let captured_piece;
            unsafe {
                captured_piece = PAST_CAPTURED_PIECES[self.nth as usize];
            }
            // capture
            if captured_piece != Piece::null {
                let captured_kind = Piece::kind(captured_piece);
                self.board[mv.to_i() as usize][mv.to_j() as usize] = captured_piece;
                self.hand[self.color as usize].sub(captured_kind);
                if captured_piece == Piece::pawn || captured_piece == Piece::Pawn {
                    self.pawn_checker[!self.color as usize][mv.to_j() as usize] = true;
                }
                // weight
                self.weight -= PIECE_TO_WEIGHT[captured_piece as usize];
                self.weight -= KIND_TO_WEIGHT[captured_kind];
                // hash
                self.hash_key = self.hash_key.wrapping_add(BOARD_HASH[captured_piece as usize][mv.to_i() as usize][mv.to_j() as usize]);
                self.hash_key = self.hash_key.wrapping_sub(HAND_HASH[self.color as usize][captured_kind]);
            } else {
                self.board[mv.to_i() as usize][mv.to_j() as usize] = Piece::null;
            }

            if mv.is_promote() {
                let demoted_piece = to_piece.demote();
                self.board[mv.from_i() as usize][mv.from_j() as usize] = demoted_piece;
                if demoted_piece == Piece::pawn || demoted_piece == Piece::Pawn {
                    self.pawn_checker[self.color as usize][mv.to_j() as usize] = true;
                }
                // weight
                self.weight -= PIECE_TO_WEIGHT[to_piece as usize];
                self.weight += PIECE_TO_WEIGHT[demoted_piece as usize];
                // hash
                self.hash_key = self.hash_key.wrapping_add(BOARD_HASH[demoted_piece as usize][mv.from_i() as usize][mv.from_j() as usize]);
            } else {
                // hash
                self.board[mv.from_i() as usize][mv.from_j() as usize] = to_piece;
                self.hash_key = self.hash_key.wrapping_add(BOARD_HASH[to_piece as usize][mv.from_i() as usize][mv.from_j() as usize]);
            }
        }
        // hash
        self.hash_key = self.hash_key.wrapping_sub(BOARD_HASH[to_piece as usize][mv.to_i() as usize][mv.to_j() as usize]);
    }

    pub fn print_move(&self, mv: &Move) {
        if self.color { print!("▲"); }
        else { print!("△"); }
        print!("{}{}", 9 - mv.to_j(), 1 + mv.to_i());
        if mv.is_drop() {
            print!("{}打", Piece::new(mv.drop_kind(), self.color));
        } else {
            print!("{}", self.board[mv.from_i() as usize][mv.from_j() as usize]);
            if mv.is_promote() {
                print!("成");
            }
        }
        println!("");
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let e = writeln!(f, "{}th", self.nth);
        writeln!(f, "color: {}", if self.color { "black" } else { "white"});
        for row in self.board.iter() {
            for p in row.iter() {
                write!(f, "{}", p);
            }
            writeln!(f, "");
        }
        write!(f, "先手の持駒: ");
        for n in 0..8 {
            for _ in 0..self.hand[1].get_num(n) {
                write!(f, "{}", Piece::kind_to_str(n));
            }
        }
        writeln!(f, "");
        write!(f, "後手の持駒: ");
        for n in 0..8 {
            for _ in 0..self.hand[0].get_num(n) {
                write!(f, "{}", Piece::kind_to_str(n));
            }
        }
        writeln!(f, "");
        e
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let e = write!(f, "{}", self);
        writeln!(f, "hash key: {}", self.hash_key);
        writeln!(f, "weight: {}", self.weight);
        writeln!(f, "black's pawn checker: {:?}", self.pawn_checker[1]);
        writeln!(f, "white's pawn checker: {:?}", self.pawn_checker[0]);
        let legal_moves = self.legal_move();
        writeln!(f, "legal move: {}", legal_moves.len());

        for mv in legal_moves {
            if self.color { write!(f, "▲"); }
            else { write!(f, "△"); }
            write!(f, "{}{}", 9 - mv.to_j(), 1 + mv.to_i());
            if mv.is_drop() {
                write!(f, "{}打", Piece::new(mv.drop_kind(), self.color));
            } else {
                write!(f, "{}", self.board[mv.from_i() as usize][mv.from_j() as usize]);
                if mv.is_promote() {
                    write!(f, "成");
                }
            }
            writeln!(f, "");
        }
        e
    }
}
