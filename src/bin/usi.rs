#[macro_use]
extern crate lazy_static;
extern crate girigiri;

use girigiri::usi::player::*;

fn main() {
    let mut player = UsiPlayer::new();
    player.client.wait_usi();
    player.client.wait_isready();

    player.play();
}
