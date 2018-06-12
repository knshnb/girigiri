// state.undo_move()するのに必要
// assume longest battle 1000
use shogi::piece::Piece;
pub static mut PAST_CAPTURED_PIECES: [Piece; 1000] = [Piece::null; 1000];
