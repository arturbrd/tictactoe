#[cfg(test)]
mod tests;

use std::fmt;
use super::GameState;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sign {
    Cross,
    Circle
}
impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Cross => "Crosses",
            Self::Circle => "Circles"
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TileSetError {
    msg: String
}
impl fmt::Display for TileSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    } 
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    state: Option<Sign>
}
impl Tile {
    pub fn new() -> Self {
        Tile { state: None }
    }
    pub fn set(&mut self, sign: &Sign) -> Result<(), TileSetError> {
        match self.state {
            Some(_) => Err(TileSetError { msg: String::from("Tile is not empty") }),
            None => {
                self.state = Some(sign.to_owned());
                Ok(())
            }
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self.state {
            None => "   ",
            Some(Sign::Circle) => " O ",
            Some(Sign::Cross) => " X "
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub struct Board {
    tiles: [[Tile; 3]; 3]
}
impl Board {
    pub fn new() -> Self {
        Board { tiles: [[Tile::new(); 3]; 3] }
    }
    pub fn set(&mut self, i: usize, j: usize, sign: &Sign) -> Result<(), TileSetError> {
        if i < 3 && j < 3 { self.tiles[i][j].set(sign)? } else { return Err(TileSetError { msg: String::from("No such tile (bad index)") } ) };
        Ok(())
    }
    pub fn check(&self, counter: u8) -> GameState {
        'outer: for i in 0..3 {
            let sign = self.tiles[i][0].state;
            if sign.is_some() {
                for j in 1..3 {
                    if self.tiles[i][j].state != sign {
                        continue 'outer;
                    }
                }
                match sign.unwrap() {
                    Sign::Circle => { return GameState::CirclesWon },
                    Sign::Cross => { return GameState::CrossesWon }
                }
            }
        }
        'outer: for i in 0..3 {
            let sign = self.tiles[0][i].state;
            if sign.is_some() {
                for j in 1..3 {
                    if self.tiles[j][i].state != sign {
                        continue 'outer;
                    }
                }
                match sign.unwrap() {
                    Sign::Circle => { return GameState::CirclesWon },
                    Sign::Cross => { return GameState::CrossesWon }
                }
            }
        }
        let sign = self.tiles[0][0].state;
        if sign.is_some() {
            let mut broke = false;
            for i in 1..3 {
                if self.tiles[i][i].state != sign {
                    broke = true;
                    break;
                }
            }
            if !broke {
                match sign.unwrap() {
                    Sign::Circle => { return GameState::CirclesWon },
                    Sign::Cross => { return GameState::CrossesWon }
                }
            }
        }
        let sign = self.tiles[0][2].state;
        if sign.is_some() {
            let mut broke = false;
            for i in 1..3 {
                if self.tiles[i][2-i].state != sign {
                    broke = true;
                    break;
                }
            }
            if !broke {
                match sign.unwrap() {
                    Sign::Circle => { return GameState::CirclesWon },
                    Sign::Cross => { return GameState::CrossesWon }
                }
            }
        }
        if counter < 9 { GameState::InProgress }
        else { GameState::Draw }
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::from("   1   2   3\n");
        let map = ["a ", "b ", "c "];
        for (i, row) in self.tiles.iter().enumerate() {
            str.push_str(map[i]);
            for (j, tile) in row.iter().enumerate() {
                str.push_str(&tile.to_string());
                if j < 2 { str.push('|'); };
            }
            if i < 2 { str.push_str("\n  -----------\n"); };
        }
        write!(f, "{}", str)
    }
}