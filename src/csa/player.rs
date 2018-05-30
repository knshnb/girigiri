use engine::alpha_beta_engine::*;
use csa::client::*;
use board::move_encode::*;
use std::net::ToSocketAddrs;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

pub struct CsaPlayer {
    pub client: CsaClient,
    engine: AlphaBetaEngine,
    is_black: bool,
    kifu: Option<File>,
}

impl CsaPlayer {
    pub fn new<A: ToSocketAddrs>(host: A) -> CsaPlayer {
        CsaPlayer {
            client: CsaClient::connect(host),
            engine: AlphaBetaEngine::new(10, 10, true), // (depth, time_limit, use_pp)
            is_black: true,
            kifu: None,
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

    pub fn set_save_kifu(&mut self, file_path: &str) {
        self.kifu = match File::create(&Path::new(file_path)) {
            Err(why) => panic!("couldn't create {}: {}", file_path, Error::description(&why)),
            Ok(file) => Some(file),
        }
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
        let cmd = self.client.read();
        match self.kifu {
            Some(ref mut file) => file.write_all(cmd.as_bytes()),
            None => Ok(()),
        };
        println!("my move: \n{}\n", cmd);
        if self.client.check_finish() {
            return true;
        }
        self.client.read(); // ?? no content
        false
    }
    fn opponent_turn(&mut self) -> bool {
        println!("waiting ...");
        let cmd = self.client.read();
        match self.kifu {
            Some(ref mut file) => file.write_all(cmd.as_bytes()),
            None => Ok(()),
        };
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
