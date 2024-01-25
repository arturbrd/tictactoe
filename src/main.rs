mod game;

use std::io;

fn main() {
    let mut play = true;
    while play {
        let mut game = game::Game::new();
        match game.start() {
            Ok(()) => {
                println!("Type \'r\' to play again:");
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).expect("Error occured while reading");
                play = buf.as_str().trim() == "r";
            },
            Err(e) => { println!("{}", e); }
        }
    }
}