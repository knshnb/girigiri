mod csa;
mod board;
use board::piece::*;
use board::state::*;
use csa_helper::parser::*;

#[test]
fn csa_test() {
    let initial_state = State::new();
    let cmd1 = "+7776FU";
    println!("{}", Move::from_csa(cmd1, initial_state));
}
