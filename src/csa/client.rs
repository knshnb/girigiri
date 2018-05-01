use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct CsaClient {
    stream: TcpStream,
    buf: [u8; 8192],
    game_summary: String,
}

impl CsaClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> CsaClient {
        CsaClient {
            stream: TcpStream::connect(addr).unwrap(),
            buf: [0; 8192],
            game_summary: String::new(),
        }
    }

    pub fn read(&mut self) -> String {
        self.buf = [0; 8192];
        self.stream.read(&mut self.buf).unwrap();
        self.buf.iter().map(|&c| c as char).collect()
    }

    pub fn write(&mut self, cmd: &str) {
        writeln!(self.stream, "{}", cmd).unwrap();
    }

    pub fn login(&mut self, username: &str, password: &str) {
        let cmd = format!("LOGIN {} {}", username, password);
        self.write(&cmd);
    }

    fn confirm(&mut self) -> bool {
        println!("Play this game? [y/n]");
        let mut res = String::new();
        loop {
            io::stdin().read_line(&mut res).unwrap();
            match res.trim().as_ref() {
                "y" | "yes" => {
                    self.write("AGREE");
                    return true;
                }
                "n" | "no" => {
                    self.write("REJECT");
                    return false;
                }
                _ => (),
            }
        }
    }

    pub fn find_game(&mut self) {
        loop {
            let s = self.read();
            if !s.is_empty() && s.find("BEGIN Game_Summary").is_some() {
                println!("{}", s);
                if self.confirm() {
                    self.game_summary = s;
                    return;
                }
            }
        }
    }

    pub fn is_my_turn(&self) -> bool {
        self.game_summary.find("Your_Turn:+").is_some()
    }
}
