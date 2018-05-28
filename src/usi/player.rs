use engine::alpha_beta_engine::*;
use usi::client::*;
use board::move_encode::*;

pub struct UsiPlayer {
    pub client: UsiClient,
    engine: AlphaBetaEngine,
}

impl UsiPlayer {
    pub fn new() -> UsiPlayer {
        UsiPlayer {
            client: UsiClient::new(),
            engine: AlphaBetaEngine::new(10, 10, false), // (depth, time_limit, use_pp)
        }
    }

    pub fn play(&mut self) {
        loop {
            self.client.read();
            if self.client.cmd.find("gameover").is_some() {
                break;
            } else if self.client.cmd.find("position").is_some() {
                if !self.client.is_first() {
                    self.engine.state.apply_move(&self.client.opponent_move());
                }
            } else if self.client.cmd.find("go").is_some() {
                let mv = self.engine.proceed_move_without_print();
                self.client.write_move(mv);
            }
        }
    }
}
