pub enum Color {
    Black,
    White,
    Null,
}

impl PartialEq for Color {
    fn eq(&self, other:&Color) -> bool {
        match (self, other) {
            (&Color::Black, &Color::Black) | (&Color::White, &Color::White) | (&Color::Null, &Color::Null) => true,
            _ => false
        }
    }
}

pub fn whose(piece: u8) -> Color {
    if piece == 0 {
        Color::Null
    } else if piece <= 14 {
        Color::Black
    } else {
        Color::White
    }
}

pub fn promote(piece: u8) -> u8 {
    piece + 6
}

pub fn get_kind(piece: u8) -> usize {
    match piece {
        1 | 15 | 7 | 21 => 0, // pawn
        2 | 16 | 8 | 22 => 1, // lance
        3 | 17 | 9 | 23 => 2, // knight
        4 | 18 | 10 | 24 => 3, // silver
        5 | 19 | 11 | 25 => 4, // bishop
        6 | 20 | 12 | 26 => 5, // rook
        13 | 27 => 6, // gold
        14 | 28 => 7, // king
        0 | _ => {
            println!("Err null piece!");
            100
        }
    }
}
pub fn kind_to_japanese(kind: usize) -> &'static str {
    match kind {
        0   => " 歩",
        1   => " 香",
        2   => " 桂",
        3   => " 銀",
        4   => " 角",
        5   => " 飛",
        6   => " 金",
        7   => " 王",
        _   => " なし",
    }
}

pub fn kind_to_piece(kind: usize, is_black: bool) -> u8 {
    let black_piece = match kind {
        0 => 1,
        1 => 2,
        2 => 3,
        3 => 4,
        4 => 5,
        5 => 6,
        6 => 13,
        7 => 14,
        _ => {
            println!("Err not a kind");
            0
        }
    };
    if !is_black { black_piece + 14 }
    else { black_piece }
}

pub fn piece_to_english(piece: u8) -> &'static str {
    match piece {
        0   => "null",

        // black's pieces
        1   => "Pawn",
        2   => "Lance",
        3   => "Knight",
        4   => "Silver",
        5   => "Bishop",
        6   => "Rook",

        7   => "Propawn",
        8   => "Prolance",
        9   => "Pronight",
        10  => "Prosilver",
        11  => "Horse",
        12  => "Dragon",

        13  => "Gold",
        14  => "King",

        // white's pieces
        15  => "pawn",
        16  => "lance",
        17  => "knight",
        18  => "silver",
        19  => "bishop",
        20  => "rook",

        21  => "propawn",
        22  => "prolance",
        23  => "pronight",
        24  => "prosilver",
        25  => "horse",
        26  => "dragon",

        27  => "gold",
        28  => "king",

        _   => "not a piece",
    }
}

pub fn piece_to_japanese(piece : u8) -> &'static str {
    match piece {
        0   => " 口",

        // black's pieces
        1   => " 歩",
        2   => " 香",
        3   => " 桂",
        4   => " 銀",
        5   => " 角",
        6   => " 飛",

        7   => " と",
        8   => " 杏",
        9   => " 圭",
        10  => " 全",
        11  => " 馬",
        12  => " 龍",

        13  => " 金",
        14  => " 王",

        // white's pieces
        15  => "^歩",
        16  => "^香",
        17  => "^桂",
        18  => "^銀",
        19  => "^角",
        20  => "^飛",

        21  => "^と",
        22  => "^杏",
        23  => "^圭",
        24  => "^全",
        25  => "^馬",
        26  => "^龍",

        27  => "^金",
        28  => "^王",

        _   => "not a piece",
    }
}
