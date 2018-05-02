use board::piece::*;
use board::state::*;
use csa::parser::*;

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: i8,
    pub to: i8,
}

impl Move {
    pub fn normal_encode(from_i: i8, from_j: i8, to_i: i8, to_j: i8) -> Move {
        Move {
            from: from_i * 9 + from_j,
            to: to_i * 9 + to_j,
        }
    }
    pub fn drop_encode(kind: u8, to_i: i8, to_j: i8) -> Move {
        Move {
            from: 81 + kind as i8,
            to: to_i * 9 + to_j,
        }
    }
    pub fn promote_encode(from_i: i8, from_j: i8, to_i: i8, to_j: i8) -> Move {
        Move {
            from: from_i * 9 + from_j,
            to: (1 << 7) | (to_i * 9 + to_j),
        }
    }

    pub fn from_csa(cmd: &str, state: &State) -> Move {
        let cmd_as_bytes = cmd.as_bytes();
        let to_j = b'9' - cmd_as_bytes[3];
        let to_i = cmd_as_bytes[4] - b'1';
        match &cmd[1..3] {
            "00" => Move::drop_encode(csa_to_kind(&cmd[5..7]) as u8, to_i as i8, to_j as i8),
            _ => {
                let from_j = b'9' - cmd_as_bytes[1];
                let from_i = cmd_as_bytes[2] - b'1';
                if !state.board[from_i as usize][from_j as usize].is_promoted() && csa_is_promoted(&cmd[5..7]) {
                    Move::promote_encode(from_i as i8, from_j as i8, to_i as i8, to_j as i8)
                } else {
                    Move::normal_encode(from_i as i8, from_j as i8, to_i as i8, to_j as i8)
                }
            }
        }
    }

    // without first "+" or "-" (referring to color)
    pub fn to_csa_suffix(&self, state: &State) -> String {
        if self.is_drop() {
            format!(
                "00{}{}{}",
                9 - self.to_j(),
                self.to_i() + 1,
                Piece::kind_to_csa(self.drop_kind())
            )
        } else {
            let piece = (*state).board[self.to_i() as usize][self.to_j() as usize];
            format!(
                "{}{}{}{}{}",
                9 - self.from_j(),
                self.from_i() + 1,
                9 - self.to_j(),
                self.to_i() + 1,
                piece.to_csa()
            )
        }
    }

    pub fn is_drop(&self) -> bool {
        self.from >= 81
    }
    pub fn is_promote(&self) -> bool {
        self.to < 0
    }

    pub fn from_i(&self) -> i8 {
        (self.from & 0b01111111) / 9
    }
    pub fn from_j(&self) -> i8 {
        (self.from & 0b01111111) % 9
    }
    pub fn to_i(&self) -> i8 {
        (self.to & 0b01111111) / 9
    }
    pub fn to_j(&self) -> i8 {
        (self.to & 0b01111111) % 9
    }
    pub fn drop_kind(&self) -> usize {
        (self.from as u8 - 81) as usize
    }

    pub fn is_null_move(self) -> bool {
        self == NULL_MOVE
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Move) -> bool {
        self.from == other.from && self.to == other.to
    }
}

pub const NULL_MOVE: Move = Move { from: 0, to: 0 };
