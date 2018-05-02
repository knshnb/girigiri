#[macro_use]
extern crate lazy_static;

mod csa;
mod engine;
mod board;
use csa::player::*;
use std::{thread, time};

const USERNAME: &str = "girigiri";
const PASSWORD: &str = "girigiri";

fn main() {
    let mut player = CsaPlayer::new(("gserver.computer-shogi.org", 4081));
    player.login(USERNAME, PASSWORD);
    println!("waiting for a game...");
    player.find_game_with_confirmation();
    println!("\n{}\n", player.client.read());
    player.init_turn();

    player.play();
}
