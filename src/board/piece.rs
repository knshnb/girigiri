use board::color::*;
use std::fmt;

// [is_black, is_promoted, kind(3 bits)]
const BLACK_MASK: isize = 0b10000;
const PROMOTED_MASK: isize = 0b01000;
const KIND_MASK: isize = 0b00111;

#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Piece {
    // black's pieces
    Pawn = 0 | BLACK_MASK,
    Propawn = 0 | PROMOTED_MASK | BLACK_MASK,
    Lance = 1 | BLACK_MASK,
    Prolance = 1 | PROMOTED_MASK | BLACK_MASK,
    Knight = 2 | BLACK_MASK,
    Proknight = 2 | PROMOTED_MASK | BLACK_MASK,
    Silver = 3 | BLACK_MASK,
    Prosilver = 3 | PROMOTED_MASK | BLACK_MASK,
    Bishop = 4 | BLACK_MASK,
    Horse = 4 | PROMOTED_MASK | BLACK_MASK,
    Rook = 5 | BLACK_MASK,
    Dragon = 5 | PROMOTED_MASK | BLACK_MASK,
    Gold = 6 | BLACK_MASK,
    King = 7 | BLACK_MASK,

    // white's pieces
    pawn = 0,
    propawn = 0 | PROMOTED_MASK,
    lance = 1,
    prolance = 1 | PROMOTED_MASK,
    knight = 2,
    proknight = 2 | PROMOTED_MASK,
    silver = 3,
    prosilver = 3 | PROMOTED_MASK,
    bishop = 4,
    horse = 4 | PROMOTED_MASK,
    rook = 5,
    dragon = 5 | PROMOTED_MASK,
    gold = 6,
    king = 7,

    null = 7 | PROMOTED_MASK, // in order to keep kind within 3 bits
}

impl Piece {
    fn to_piece(x: isize) -> Piece {
        unsafe { ::std::mem::transmute::<u8, Piece>(x as u8) }
    }

    pub fn new(kind: usize, is_black: bool) -> Piece {
        let x = if is_black {
            kind as isize | BLACK_MASK
        } else {
            kind as isize
        };
        Piece::to_piece(x)
    }

    pub fn whose(self) -> Color {
        if self == Piece::null {
            Color::Null
        } else if (self as isize) & BLACK_MASK == BLACK_MASK {
            Color::Black
        } else {
            Color::White
        }
    }

    pub fn to_white(self) -> Piece {
        Piece::to_piece((self as isize) & !BLACK_MASK)
    }

    pub fn promote(self) -> Piece {
        Piece::to_piece((self as isize) | PROMOTED_MASK)
    }

    pub fn demote(self) -> Piece {
        Piece::to_piece((self as isize) & !PROMOTED_MASK)
    }

    pub fn kind(self) -> usize {
        ((self as isize) & KIND_MASK) as usize
    }

    pub fn kind_to_str(kind: usize) -> &'static str {
        match kind {
            0 => " 歩",
            1 => " 香",
            2 => " 桂",
            3 => " 銀",
            4 => " 角",
            5 => " 飛",
            6 => " 金",
            7 => " 王",
            _ => " なし",
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self.to_white() {
            Piece::null => "口",
            Piece::pawn => "歩",
            Piece::lance => "香",
            Piece::knight => "桂",
            Piece::silver => "銀",
            Piece::bishop => "角",
            Piece::rook => "飛",
            Piece::propawn => "と",
            Piece::prolance => "杏",
            Piece::proknight => "圭",
            Piece::prosilver => "全",
            Piece::horse => "馬",
            Piece::dragon => "龍",
            Piece::gold => "金",
            Piece::king => "王",
            _ => "not a piece",
        };
        let prefix = match self.whose() {
            Color::Null | Color::Black => " ",
            Color::White => "^",
        };
        write!(f, "{}{}", prefix, name)
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}