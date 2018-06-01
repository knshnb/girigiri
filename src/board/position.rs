use board::movable::*;
use board::piece::*;
use std::ops;
use std::slice::Iter;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    P91, P81, P71, P61, P51, P41, P31, P21, P11,
    P92, P82, P72, P62, P52, P42, P32, P22, P12,
    P93, P83, P73, P63, P53, P43, P33, P23, P13,
    P94, P84, P74, P64, P54, P44, P34, P24, P14,
    P95, P85, P75, P65, P55, P45, P35, P25, P15,
    P96, P86, P76, P66, P56, P46, P36, P26, P16,
    P97, P87, P77, P67, P57, P47, P37, P27, P17,
    P98, P88, P78, P68, P58, P48, P38, P28, P18,
    P99, P89, P79, P69, P59, P49, P39, P29, P19,
}

// for Position::ROW, COLUMN
impl ops::Index<Position> for [usize; 81] {
    type Output = usize;
    fn index(&self, pos: Position) -> &usize {
        &self[pos as usize]
    }
}

// for State.board
impl ops::Index<Position> for [Piece; 81] {
    type Output = Piece;
    fn index(&self, pos: Position) -> &Piece {
        &self[pos as usize]
    }
}
impl ops::IndexMut<Position> for [Piece; 81] {
    fn index_mut(&mut self, pos: Position) -> &mut Piece {
        &mut self[pos as usize]
    }
}

// for BOARD_HASH[piece]
impl ops::Index<Position> for [u64; 81] {
    type Output = u64;
    fn index(&self, pos: Position) -> &u64 {
        &self[pos as usize]
    }
}

impl ops::Add<i8> for Position {
    type Output = Position;
    fn add(self, other: i8) -> Position {
        Position::unsafe_new(self as i8 + other)
    }
}

impl ops::Sub<i8> for Position {
    type Output = Position;
    fn sub(self, other: i8) -> Position {
        Position::unsafe_new(self as i8 - other)
    }
}

impl Position {
    // x should be in range of 0-81!!!!
    pub fn unsafe_new(x: i8) -> Position {
        unsafe { ::std::mem::transmute::<i8, Position>(x as i8) }
    }
    pub fn row_and_column(row: i8, column: i8) -> Position {
        Position::unsafe_new(row * 9 + column)
    }

    pub fn variants() -> Iter<'static, Position> {
        use self::Position::*;
        static VARIANTS: [Position; 81] = [
            P91, P81, P71, P61, P51, P41, P31, P21, P11,
            P92, P82, P72, P62, P52, P42, P32, P22, P12,
            P93, P83, P73, P63, P53, P43, P33, P23, P13,
            P94, P84, P74, P64, P54, P44, P34, P24, P14,
            P95, P85, P75, P65, P55, P45, P35, P25, P15,
            P96, P86, P76, P66, P56, P46, P36, P26, P16,
            P97, P87, P77, P67, P57, P47, P37, P27, P17,
            P98, P88, P78, P68, P58, P48, P38, P28, P18,
            P99, P89, P79, P69, P59, P49, P39, P29, P19,
        ];
        VARIANTS.into_iter()
    }

    const ROW: [usize; 81] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 1, 1, 1, 1, 1, 1, 1, 1,
        2, 2, 2, 2, 2, 2, 2, 2, 2,
        3, 3, 3, 3, 3, 3, 3, 3, 3,
        4, 4, 4, 4, 4, 4, 4, 4, 4,
        5, 5, 5, 5, 5, 5, 5, 5, 5,
        6, 6, 6, 6, 6, 6, 6, 6, 6,
        7, 7, 7, 7, 7, 7, 7, 7, 7,
        8, 8, 8, 8, 8, 8, 8, 8, 8,
    ];
    const COLUMN: [usize; 81] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8,
        0, 1, 2, 3, 4, 5, 6, 7, 8,
        0, 1, 2, 3, 4, 5, 6, 7, 8,
        0, 1, 2, 3, 4, 5, 6, 7, 8,
        0, 1, 2, 3, 4, 5, 6, 7, 8,
        0, 1, 2, 3, 4, 5, 6, 7, 8,
        0, 1, 2, 3, 4, 5, 6, 7, 8,
        0, 1, 2, 3, 4, 5, 6, 7, 8,
        0, 1, 2, 3, 4, 5, 6, 7, 8,
    ];

    pub fn row(&self) -> usize {
        Position::ROW[*self]
    }
    pub fn column(&self) -> usize {
        Position::COLUMN[*self]
    }

    pub fn step(&self, dir: Direction) -> Option<Position> {
        match dir {
            Direction::UpLeft =>
                if self.column() == 0 || self.row() == 0 { None } else { Some(*self - 10) },
            Direction::Up =>
                if self.row() == 0 { None } else { Some(*self - 9) },
            Direction::UpRight =>
                if self.column() == 8 || self.row() == 0 { None } else { Some(*self - 8) },
            Direction::Left =>
                if self.column() == 0 { None } else { Some(*self - 1) },
            Direction::Right =>
                if self.column() == 8 { None } else { Some(*self + 1) },
            Direction::DownLeft =>
                if self.column() == 0 || self.row() == 8 { None } else { Some(*self + 8) },
            Direction::Down =>
                if self.row() == 8 { None } else { Some(*self + 9) },
            Direction::DownRight =>
                if self.column() == 0 || self.row() == 8 { None } else { Some(*self + 10) },
            Direction::UpUpLeft =>
                if self.row() <= 1 || self.column() == 0 { None } else { Some(*self - 19) },
            Direction::UpUpRight =>
                if self.row() <= 1 || self.column() == 8 { None } else { Some(*self - 17) },
            Direction::DownDownLeft =>
                if self.row() >= 7 || self.column() == 0 { None } else { Some(*self + 17) },
            Direction::DownDownRight =>
                if self.row() >= 7 || self.column() == 8 { None } else { Some(*self + 19) },
        }
    }
    pub fn enemy_line(&self, is_black: bool) -> bool {
        if is_black { self.row() <= 2 } else { self.row() >= 6 }
    }
}

#[test]
fn variables_work() {
    assert_eq!(Position::variants().count(), 81);
}
