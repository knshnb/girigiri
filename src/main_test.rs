#[macro_use]
extern crate lazy_static;

mod csa;
mod engine;
mod board;
use engine::alpha_beta_engine::*;
use csa::*;
use board::move_encode::*;
use std::{thread, time};

const USER_NAME: &str = "girigiri_test";
const PASSWORD: &str = "girigiri_test";

fn main() {
    let mut engine = AlphaBetaEngine::new();
    let host = ("gserver.computer-shogi.org", 4081);
    let mut client = client::CsaClient::connect(host);

    client.login(USER_NAME, PASSWORD);
    client.find_game();
    println!("00000\n{}\n00000", client.read());
    let is_black = client.is_my_turn();

    if is_black {
        loop {
            // my turn
            let mv = engine.proceed_move();
            let mv_csa = format!("+{}\n", mv.to_csa_suffix(&engine.state));
            client.write(&mv_csa);
            println!("11111\n{}\n11111", client.read());
            client.read();

            // opponent's turn
            println!("waiting ...");
            let cmd = client.read();
            println!("22222\n{}\n22222", cmd);
            let mv = Move::from_csa(&cmd, &engine.state);
            engine.state.apply_move(&mv);
        }
    } else {
        loop {
            // opponent's turn
            println!("waiting ...");
            let cmd = client.read();
            println!("22222\n{}\n22222", cmd);
            let mv = Move::from_csa(&cmd, &engine.state);
            println!("mv{:?}mv", mv);
            engine.state.apply_move(&mv);

            // my turn
            let mv = engine.proceed_move();
            let mv_csa = format!("-{}\n", mv.to_csa_suffix(&engine.state));
            client.write(&mv_csa);
            println!("11111\n{}\n11111", client.read());
            client.read();
        }

        client.write(&format!("{}", "-3334FU"));
    }
}
