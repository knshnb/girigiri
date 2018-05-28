#[macro_use]
extern crate lazy_static;
extern crate girigiri;

use girigiri::csa::player::*;

const USERNAME: &str = "girigiri";
const PASSWORD: &str = "girigiri";

fn main() {
    let mut player = CsaPlayer::new("192.168.20.1:4081");
    player.login(USERNAME, PASSWORD);
    println!("waiting for a game...");
    player.find_game_with_confirmation();
    println!("\n{}\n", player.client.read());
    player.init_turn();

    player.play();
}
