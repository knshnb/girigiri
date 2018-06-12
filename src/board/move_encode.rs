use board::piece::*;
use board::state::*;
use board::position::*;
use csa::parser::*;

const PROMOTE_MASK: i8 = 0b10000000;
const TO_MASK: i8 = 0b01111111;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: i8,
    pub to: i8,
}

impl Move {
    pub fn normal_encode(from: Position, to: Position) -> Move {
        Move {
            from: from as i8,
            to: to as i8,
        }
    }
    pub fn drop_encode(kind: usize, to: Position) -> Move {
        Move {
            from: 81 + kind as i8,
            to: to as i8,
        }
    }
    pub fn promote_encode(from: Position, to: Position) -> Move {
        Move {
            from: from as i8,
            to: PROMOTE_MASK | (to as i8),
        }
    }

    pub fn from_usi(cmd: &str) -> Move {
        let bytes = cmd.as_bytes();
        let to = Position::row_and_column(
            bytes[3] as i8 - 'a' as i8,
            '9' as i8 - bytes[2] as i8
        );
        if bytes[1] == '*' as u8 {
            // drop
            let kind = Piece::usi_to_kind(bytes[0] as char);
            return Move::drop_encode(kind, to);
        } else {
            let from = Position::row_and_column(
                bytes[1] as i8 - 'a' as i8,
                '9' as i8 - bytes[0] as i8
            );
            if bytes.len() == 6 {
                // promote
                return Move::promote_encode(from, to);
            } else if bytes.len() == 5 {
                // normal
                return Move::normal_encode(from, to);
            } else {
                unreachable!();
            }
        }
    }
    pub fn to_usi(self) -> String {
        let to = format!("{}{}",
            9 - self.to_pos().column(),
            ('a' as u8 + self.to_pos().row() as u8) as char
        );
        if self.is_drop() {
            format!(
                "{}*{}\n",
                Piece::kind_to_usi(self.drop_kind()),
                to
            )
        } else {
            let promote_suffix = if self.is_promote() { "+" } else { "" };
            format!(
                "{}{}{}{}\n",
                9 - self.from_pos().column(),
                ('a' as u8 + self.from_pos().row() as u8) as char,
                to,
                promote_suffix
            )
        }
    }

    pub fn from_csa(cmd: &str, state: &State) -> Move {
        let bytes = cmd.as_bytes();
        let to = Position::row_and_column(
            bytes[4] as i8 - '1' as i8,
            '9' as i8 - bytes[3] as i8
        );
        match &cmd[1..3] {
            "00" => Move::drop_encode(csa_to_kind(&cmd[5..7]), to),
            _ => {
                let from = Position::row_and_column(
                    bytes[2] as i8 - '1' as i8,
                    '9' as i8 - bytes[1] as i8
                );
                if !state.board[from].is_promoted() && csa_is_promoted(&cmd[5..7]) {
                    Move::promote_encode(from, to)
                } else {
                    Move::normal_encode(from, to)
                }
            }
        }
    }

    // Use this function after applying move!!
    pub fn to_csa(self, state: &State) -> String {
        // 呼ばれるのがapply_moveをした後なので逆になっている
        let turn_symbol = if state.color { "-" } else { "+" };
        let to = format!("{}{}",
            9 - self.to_pos().column(),
            self.to_pos().row() + 1,
        );
        if self.is_drop() {
            format!(
                "{}00{}{}",
                turn_symbol,
                to,
                Piece::kind_to_csa(self.drop_kind())
            )
        } else {
            let piece = (*state).board[self.to_pos()];
            format!(
                "{}{}{}{}{}",
                turn_symbol,
                9 - self.from_pos().column(),
                self.from_pos().row() + 1,
                to,
                piece.to_csa()
            )
        }
    }

    pub fn is_drop(self) -> bool {
        self.from >= 81
    }

    pub fn is_promote(self) -> bool {
        self.to < 0
    }

    pub fn from_pos(self) -> Position {
        Position::unsafe_new(self.from)
    }
    pub fn to_pos(self) -> Position {
        Position::unsafe_new(self.to & TO_MASK)
    }

    pub fn drop_kind(&self) -> usize {
        (self.from as u8 - 81) as usize
    }

    pub fn is_null_move(self) -> bool {
        self == NULL_MOVE
    }
}

pub const NULL_MOVE: Move = Move { from: 0, to: 0 };

#[test]
fn usi_works() {
    let usi1 = "7g7f\n";
    let usi2 = "8h2b+\n";
    let usi3 = "P*2d\n";
    assert_eq!(usi1, Move::from_usi(usi1).to_usi());
    assert_eq!(usi2, Move::from_usi(usi2).to_usi());
    assert_eq!(usi3, Move::from_usi(usi3).to_usi());
}

#[test]
fn csa_works() {
    let state = State::new();
    let csa1 = "-3233FU";
    let csa2 = "-0033FU";
    println!("{:?}", Move::from_csa(csa1, &state));
    assert_eq!(csa1, Move::from_csa(csa1, &state).to_csa(&state));
    assert_eq!(csa2, Move::from_csa(csa2, &state).to_csa(&state));
}