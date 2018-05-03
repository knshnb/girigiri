use engine::alpha_beta_engine::*;
use csa::client::*;
use board::move_encode::*;
use std::net::{TcpStream, ToSocketAddrs};

pub struct CsaPlayer {
    pub client: CsaClient,
    engine: AlphaBetaEngine,
    is_black: bool,
}

impl CsaPlayer {
    pub fn new<A: ToSocketAddrs>(host: A) -> CsaPlayer {
        CsaPlayer {
            client: CsaClient::connect(host),
            engine: AlphaBetaEngine::new(10, 10, true), // (depth, time_limit, use_pp)
            is_black: true,
        }
    }

    pub fn login(&mut self, username: &str, password: &str) {
        self.client.login(username, password);
    }

    pub fn find_game_with_confirmation(&mut self) {
        self.client.find_game_with_confirmation();
    }
    pub fn find_game_auto(&mut self) {
        self.client.find_game_auto();
    }

    pub fn init_turn(&mut self) {
        self.is_black = self.client.is_my_turn();
    }

    fn my_turn(&mut self) -> bool {
        let mv = self.engine.proceed_move();
        let turn_symbol = if self.is_black { "+" } else { "-" };
        if mv.is_null_move() {
            self.client.resign();
        } else {
            let mv_csa = format!("{}{}\n", turn_symbol, mv.to_csa_suffix(&self.engine.state));
            self.client.write(&mv_csa);
        }
        println!("my move: \n{}\n", self.client.read());
        if self.client.check_finish() {
            return true;
        }
        self.client.read(); // ?? no content
        false
    }
    fn opponent_turn(&mut self) -> bool {
        println!("waiting ...");
        let cmd = self.client.read();
        println!("opponent's move: \n{}\n", cmd);
        if self.client.check_finish() {
            return true;
        }
        let mv = Move::from_csa(&cmd, &self.engine.state);
        self.engine.state.apply_move(&mv);
        false
    }
    pub fn play(&mut self) {
        if self.is_black {
            if self.my_turn() {
                return;
            }
        }
        loop {
            if self.opponent_turn() {
                return;
            }
            if self.my_turn() {
                return;
            }
        }
    }
}
