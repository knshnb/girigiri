use board::move_encode::*;
use board::piece::*;
use board::hand::*;
use board::hash::*;
use board::eval::*;
use board::past_captured_piece::*;
use board::position::*;
use std::fmt;

extern crate rand;

#[derive(Clone)]
pub struct State {
    pub nth: u16,
    pub color: bool, // true: black, false: white
    pub board: [Piece; 81],
    pub hand: [Hand; 2],              // hand[0]: white, hand[1]: black
    pub pawn_checker: [[bool; 9]; 2], // pawn_checker[0]: white, pawn_checker[1]: black
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
                [lance, knight, silver, gold, king, gold, silver, knight, lance,
                 null,  rook, null,  null,  null,  null,  null,  bishop, null ,
                 pawn, pawn, pawn, pawn, pawn, pawn, pawn, pawn, pawn,
                 null, null, null, null, null, null, null, null, null,
                 null, null, null, null, null, null, null, null, null,
                 null, null, null, null, null, null, null, null, null,
                 Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn,
                 null,  Bishop,  null,  null,  null,  null,  null,  Rook,  null ,
                 Lance, Knight, Silver, Gold, King, Gold, Silver, Knight, Lance],
            hand: [Hand::new(), Hand::new()],
            pawn_checker: [[true; 9]; 2],
            hash_key: 0,
            weight:0,
        }
    }

    pub fn set_sfen(&mut self, sfen: &str) {
        let sfen: Vec<&str> = sfen.split(' ').collect();
        // board
        let mut pos = Position::P91;
        for row in sfen[0].split('/').collect::<Vec<&str>>() {
            let bytes = row.as_bytes();
            let mut x = 0;
            while x < bytes.len() {
                if '1' as u8 <= bytes[x] && bytes[x] <= '9' as u8 {
                    for _ in 0..(bytes[x] - '0' as u8) {
                        self.board[pos] = Piece::null;
                        pos = pos + 1;
                    }
                    x += 1;
                } else {
                    if bytes[x] as char == '+' {
                        self.board[pos] = Piece::from_sfen(&row[x..x+2]);
                        x += 2;
                    } else {
                        self.board[pos] = Piece::from_sfen(&row[x..x+1]);
                        x += 1;
                    }
                    pos = pos + 1;
                }
            }
        }
        // turn
        match sfen[1] {
            "b" => { self.color = true; },
            "w" => { self.color = false; },
            _ => unimplemented!(),
        }
        // hand
        if sfen[2] == "-" { return; }
        let bytes = sfen[2].as_bytes();
        let mut x = 0;
        while x < bytes.len() {
            if '1' as u8 <= bytes[x] && bytes[x] <= '9' as u8 {
                let (is_black, kind) = Piece::sfen_to_kind(bytes[x + 1] as char);
                for _ in 0..(bytes[x] - '0' as u8) {
                    self.hand[is_black as usize].add(kind);
                }
                x += 2;
            } else {
                let (is_black, kind) = Piece::sfen_to_kind(bytes[x] as char);
                self.hand[is_black as usize].add(kind);
                x += 1;
            }
        }
    }


    // pub fn valid(ps: &[Piece; 81]) -> bool {
    //     /*
    //     (1. 二歩になっていない)
    //     2. 歩、香、桂が進める位置にある
    //     3. 王が自陣にいる
    //     */
    //     for i in 0..9 {
    //         if ps[i].kind() <= 2 && ps[i].whose() == Color::Black {
    //             return false;
    //         }
    //     }
    //     for i in 72..81 {
    //         if ps[i].kind() <= 2 && ps[i].whose() == Color::White {
    //             return false;
    //         }
    //     }
    //     let mut flag = false;
    //     for i in 63..81 {
    //         if ps[i].kind() == 7 && ps[i].whose() == Color::Black {
    //             flag = true;
    //         }
    //     }
    //     if !flag {
    //         return false;
    //     }
    //     flag = false;
    //     for i in 0..18 {
    //         if ps[i].kind() == 7 && ps[i].whose() == Color::White {
    //             flag = true;
    //         }
    //     }
    //     if !flag {
    //         return false;
    //     }
    //     true
    // }

    // pub fn randomize(&mut self) {
    //     let mut ps: [Piece; 81] = [Piece::null; 81];
    //     let mut rng = rand::thread_rng();
    //     for i in 0..9 {
    //         for j in 0..9 {
    //             if i == 2 || i == 7 {
    //                 // randomly add pawn and Pawn
    //                 if rng.gen() {
    //                     ps[9 * i + j] = self.board[i][j];
    //                 } else {
    //                     ps[9 * i + j] = Piece::null;
    //                 }
    //             } else {
    //                 ps[9 * i + j] = self.board[i][j];
    //             }
    //         }
    //     }
    //     rng.shuffle(&mut ps);
    //     while !State::valid(&ps) {
    //         rng.shuffle(&mut ps);
    //     }
    //     for i in 0..81 {
    //         self.board[i / 9][i % 9] = ps[i];
    //     }
    // }

    pub fn is_capture(&self, &mv: &Move) -> bool {
        self.board[mv.to_pos()] != Piece::null
    }

    pub fn is_pawn_promote(&self, &mv: &Move) -> bool {
        if self.color {
            mv.is_promote() && self.board[mv.from_pos()] == Piece::Pawn
        } else {
            mv.is_promote() && self.board[mv.from_pos()] == Piece::pawn
        }
    }

    pub fn print_expectation(&mut self, depth: u8) {
        let mut state = self.clone();
        for _ in 0..depth {
            let mv;
            unsafe {
                mv = HASH_TABLE[(state.hash_key & *HASH_KEY_MASK) as usize].best_move;
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
            self.board[mv.to_pos()] = drop_piece;
            if drop_kind == 0 {
                // pawn
                self.pawn_checker[self.color as usize][mv.to_pos().column()] = true;
            }
            // weight
            self.weight -= KIND_TO_WEIGHT[drop_kind as usize] / 10;
            // hash
            self.hash_key = self.hash_key
                .wrapping_add(BOARD_HASH[drop_piece as usize][mv.to_pos()]);
            self.hash_key = self.hash_key
                .wrapping_sub(HAND_HASH[self.color as usize][drop_kind]);
        } else {
            let from_piece = self.board[mv.from_pos()];
            self.board[mv.from_pos()] = Piece::null;

            let to_piece = self.board[mv.to_pos()];
            unsafe {
                PAST_CAPTURED_PIECES[self.nth as usize] = to_piece;
            }
            // capture
            if to_piece != Piece::null {
                let captured_kind = Piece::kind(to_piece);
                self.hand[self.color as usize].add(captured_kind);
                if to_piece == Piece::pawn || to_piece == Piece::Pawn {
                    self.pawn_checker[!self.color as usize][mv.to_pos().column()] = false;
                }
                // weight
                self.weight += PIECE_TO_WEIGHT[to_piece as usize];
                self.weight += KIND_TO_WEIGHT[captured_kind];
                // hash
                self.hash_key = self.hash_key
                    .wrapping_sub(BOARD_HASH[to_piece as usize][mv.to_pos()]);
                self.hash_key = self.hash_key
                    .wrapping_add(HAND_HASH[self.color as usize][captured_kind]);
            }

            if mv.is_promote() {
                let promoted_piece = from_piece.promote();
                self.board[mv.to_pos()] = promoted_piece;
                if from_piece == Piece::pawn || from_piece == Piece::Pawn {
                    self.pawn_checker[self.color as usize][mv.to_pos().column()] = false;
                }
                // weight
                self.weight += PIECE_TO_WEIGHT[promoted_piece as usize];
                self.weight -= PIECE_TO_WEIGHT[from_piece as usize];
                // hash
                self.hash_key = self.hash_key
                    .wrapping_add(BOARD_HASH[promoted_piece as usize][mv.to_pos()]);
            } else {
                // hash
                self.board[mv.to_pos()] = from_piece;
                self.hash_key = self.hash_key
                    .wrapping_add(BOARD_HASH[from_piece as usize][mv.to_pos()]);
            }
            // hash
            self.hash_key = self.hash_key
                .wrapping_sub(BOARD_HASH[from_piece as usize][mv.from_pos()]);
        }
        self.color = !self.color;
        self.weight = -self.weight;
        self.nth += 1;
    }

    pub fn undo_move(&mut self, mv: &Move) {
        self.nth -= 1;
        self.weight = -self.weight;
        self.color = !self.color;
        let to_piece = self.board[mv.to_pos()];
        if mv.is_drop() {
            let drop_kind = mv.drop_kind();
            self.board[mv.to_pos()] = Piece::null;
            self.hand[self.color as usize].add(drop_kind);
            if drop_kind == 0 {
                // pawn
                self.pawn_checker[self.color as usize][mv.to_pos().column()] = false;
            }
            // weight
            self.weight += KIND_TO_WEIGHT[drop_kind as usize] / 10;
            // hash
            self.hash_key = self.hash_key
                .wrapping_add(HAND_HASH[self.color as usize][drop_kind]);
        } else {
            let captured_piece;
            unsafe {
                captured_piece = PAST_CAPTURED_PIECES[self.nth as usize];
            }
            // capture
            if captured_piece != Piece::null {
                let captured_kind = Piece::kind(captured_piece);
                self.board[mv.to_pos()] = captured_piece;
                self.hand[self.color as usize].sub(captured_kind);
                if captured_piece == Piece::pawn || captured_piece == Piece::Pawn {
                    self.pawn_checker[!self.color as usize][mv.to_pos().column()] = true;
                }
                // weight
                self.weight -= PIECE_TO_WEIGHT[captured_piece as usize];
                self.weight -= KIND_TO_WEIGHT[captured_kind];
                // hash
                self.hash_key = self.hash_key
                    .wrapping_add(BOARD_HASH[captured_piece as usize][mv.to_pos()]);
                self.hash_key = self.hash_key
                    .wrapping_sub(HAND_HASH[self.color as usize][captured_kind]);
            } else {
                self.board[mv.to_pos()] = Piece::null;
            }

            if mv.is_promote() {
                let demoted_piece = to_piece.demote();
                self.board[mv.from_pos()] = demoted_piece;
                if demoted_piece == Piece::pawn || demoted_piece == Piece::Pawn {
                    self.pawn_checker[self.color as usize][mv.to_pos().column()] = true;
                }
                // weight
                self.weight -= PIECE_TO_WEIGHT[to_piece as usize];
                self.weight += PIECE_TO_WEIGHT[demoted_piece as usize];
                // hash
                self.hash_key = self.hash_key
                    .wrapping_add(BOARD_HASH[demoted_piece as usize][mv.from_pos()]);
            } else {
                // hash
                self.board[mv.from_pos()] = to_piece;
                self.hash_key = self.hash_key
                    .wrapping_add(BOARD_HASH[to_piece as usize][mv.from_pos()]);
            }
        }
        // hash
        self.hash_key = self.hash_key
            .wrapping_sub(BOARD_HASH[to_piece as usize][mv.to_pos()]);
    }

    pub fn print_move(&self, mv: &Move) {
        if self.color {
            print!("▲");
        } else {
            print!("△");
        }
        if mv.is_null_move() {
            println!("投了");
            return;
        }
        print!("{}{}", 9 - mv.to_pos().column(), 1 + mv.to_pos().row());
        if mv.is_drop() {
            print!("{}打", Piece::new(mv.drop_kind(), self.color));
        } else {
            print!("{}", self.board[mv.from_pos()]);
            if mv.is_promote() {
                print!("成");
            }
        }
        println!("");
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}th", self.nth)?;
        writeln!(f, "color: {}", if self.color { "black" } else { "white" })?;
        for col in 0..9 {
            write!(f, " {} ", 9 - col)?;
        }
        writeln!(f, "")?;
        for &pos in Position::variants() {
            write!(f, "{}", self.board[pos])?;
            if pos.column() == 8 {
                writeln!(f, " {}", ('a' as u8 + pos.row() as u8) as char)?;
            }
        }
        write!(f, "先手の持駒: ")?;
        for n in 0..8 {
            for _ in 0..self.hand[1].get_num(n) {
                write!(f, "{}", Piece::kind_to_str(n))?;
            }
        }
        writeln!(f, "")?;
        write!(f, "後手の持駒: ")?;
        for n in 0..8 {
            for _ in 0..self.hand[0].get_num(n) {
                write!(f, "{}", Piece::kind_to_str(n))?;
            }
        }
        writeln!(f, "")?;
        Ok(())
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)?;
        writeln!(f, "hash key: {}", self.hash_key)?;
        writeln!(f, "weight: {}", self.weight)?;
        writeln!(f, "black's pawn checker: {:?}", self.pawn_checker[1])?;
        writeln!(f, "white's pawn checker: {:?}", self.pawn_checker[0])?;
        let legal_moves = self.legal_move();
        writeln!(f, "legal move: {}", legal_moves.len())?;

        for mv in legal_moves {
            if self.color {
                write!(f, "▲")?;
            } else {
                write!(f, "△")?;
            }
            write!(f, "{}{}", 9 - mv.to_pos().column(), 1 + mv.to_pos().row())?;
            if mv.is_drop() {
                write!(f, "{}打", Piece::new(mv.drop_kind(), self.color))?;
            } else {
                write!(
                    f,
                    "{}",
                    self.board[mv.from_pos()]
                )?;
                if mv.is_promote() {
                    write!(f, "成")?;
                }
            }
        }
        Ok(())
    }
}

#[test]
fn can_read_sfen() {
    let mut state = State::new();
    state.set_sfen("l3kgsnl/2n1g4/p3p1+Ppb/2P6/9/2p6/PP1PPP1PP/1B2K2R+p/L1GS1GS2 w 2NLPrs3p 1");
}