mod board;

use std::fmt;
use board::{Board, Sign};
use std::io;

use self::board::TileSetError;

#[derive(Debug, Clone)]
pub struct GameError {
    msg: String
}
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GameError: {}", self.msg)
    }
}

#[derive(Debug, PartialEq)]
enum GameState {
    CirclesWon,
    CrossesWon,
    Draw,
    InProgress,
    NotStarted
}

#[derive(Debug)]
pub struct Game {
    game_state: GameState,
    whose_turn: Sign,
    counter: u8,
    board: Board
}
impl Game {
    pub fn new() -> Game {
        Game { 
            game_state: GameState::NotStarted,
            whose_turn: { if rand::random() { Sign::Circle } else { Sign::Cross } },
            counter: 0,
            board: Board::new()
        }
    }
    pub fn start(&mut self) -> Result<(), GameError> {
        if self.game_state != GameState::NotStarted {
            Err(GameError { msg: String::from("game has already started and isn't finished") })
        } else {
            self.game_state = GameState::InProgress;
            while self.game_state == GameState::InProgress {
                self.turn()
            };
            print!("{}", self);
            Ok(())
        }
    }
    fn turn(&mut self) {
        let mut valid = false;
        while !valid {
            println!("{}", self);
            let mut buf = String::new();
            match io::stdin().read_line(&mut buf) {
                Err(_) => {
                    println!("Invalid input, try again!");
                },
                Ok(_) => {
                    let chars: Vec<char> = buf.as_str().trim().chars().collect();
                    if chars.len() == 2 && (chars[0] == 'a' || chars[0] == 'b' || chars[0] == 'c') && (chars[1] == '1' || chars[1] == '2' || chars[1] == '3') {
                        match self.set(chars[0], chars[1]) {
                            Ok(_) => { valid = true; },
                            Err(e) => { println!("{}, try again!", e); }
                        }
                    } else { println!("Invalid input, try again!"); };
                }
            }
        };
        self.counter += 1;
        if self.counter >= 5 { self.game_state = self.board.check(self.counter); };
        if self.game_state != GameState::CirclesWon && self.game_state != GameState::CrossesWon {
            match self.whose_turn {
                Sign::Circle => { self.whose_turn = Sign::Cross},
                Sign::Cross => { self.whose_turn = Sign::Circle},
            }
        }
    }
    fn set(&mut self, row: char, col: char) -> Result<(), TileSetError> {
        let i = match row {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            _ => 3
        };
        let j = match col {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            _ => 3
        };
        self.board.set(i, j, &self.whose_turn)
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prompt = match self.game_state {
            GameState::NotStarted => String::from("Game doesn't started!\n"),
            GameState::InProgress => format!("{}' turn:", self.whose_turn),
            GameState::CirclesWon | GameState::CrossesWon => format!("{} won!!!\n", self.whose_turn),
            GameState::Draw => String::from("It's a draw\n")
        };
        write!(f, "_____________\n\n{}\n\n{}", self.board, prompt)
    }
}