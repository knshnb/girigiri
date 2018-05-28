use std::io;
use board::move_encode::*;

pub struct UsiClient {
    pub cmd: String,
}

impl UsiClient {
    pub fn new() -> UsiClient {
        UsiClient {
            cmd: String::new(),
        }
    }

    pub fn read(&mut self) {
        self.cmd = String::new();
        io::stdin().read_line(&mut self.cmd).expect("Failed to read line");
    }

    pub fn wait_usi(&mut self) {
        loop {
            self.read();
            if self.cmd == "usi\n" {
                println!("id name Girigiri");
                println!("id auther Kenshin");
                println!("usiok");
                break;
            }
        }
    }

    pub fn wait_isready(&mut self) {
        loop {
            self.read();
            if self.cmd == "isready\n" {
                println!("readyok");
                break;
            }
        }
    }

    pub fn opponent_move(&self) -> Move {
        let bytes = self.cmd.as_bytes();
        if bytes[bytes.len() - 2] as char == '+' {
            return Move::from_usi(&self.cmd[(bytes.len() - 6)..]);
        } else {
            return Move::from_usi(&self.cmd[(bytes.len() - 5)..]);
        }
    }

    pub fn is_first(&self) -> bool {
        let bytes = self.cmd.as_bytes();
        bytes[bytes.len() - 2] as char == 's'
    }
    
    pub fn write_move(&self, mv: Move) {
        if mv.is_null_move() {
            println!("bestmove resign");
        } else {
            println!("bestmove {}", mv.to_usi());
        }
    }
}
