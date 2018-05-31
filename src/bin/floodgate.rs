extern crate lazy_static;
extern crate girigiri;

use girigiri::csa::player::*;

const USERNAME: &str = "girigiri";
const PASSWORD: &str = "floodgate-300-10F,hoge";

fn main() {
    let mut player = CsaPlayer::new(("wdoor.c.u-tokyo.ac.jp", 4081));
    player.set_save_kifu("kifu/sample.csa");
    player.login(USERNAME, PASSWORD);
    println!("waiting for a game...");
    player.find_game_auto();
    println!("\n{}\n", player.client.read());
    player.init_turn();

    player.play();
}
