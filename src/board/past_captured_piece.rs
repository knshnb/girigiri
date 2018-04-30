// assume longest battle 1000
use board::piece::Piece;
pub static mut PAST_CAPTURED_PIECES: [Piece; 1000] = [Piece::null; 1000];
