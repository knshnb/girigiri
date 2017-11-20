use std::io;

mod engine;
mod board;
// use engine::first_engine::*;
use engine::random_engine::*;

fn main() {
    let mut n = 1;
    let mut input = String::new();
    let mut engine = RandomEngine::new();
    loop {
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        println!("{}th move", n);
        engine.proceed_move();
        n += 1;

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        println!("{}th move", n);
        engine.proceed_move();
        n += 1;
    }
}
