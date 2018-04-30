#[derive(Copy, Clone)]
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
            to: (1 << 7) + to_i * 9 + to_j,
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
}

impl PartialEq for Move {
    fn eq(&self, other: &Move) -> bool {
        self.from == other.from && self.to == other.to
    }
}

pub const NULL_MOVE: Move = Move { from: 0, to: 0 };
