mod board_rep;

use crate::models::Square;
use crate::models::Worker;
use crate::helpers::Turn;
use std::collections::HashSet;


pub struct Move {
    // From is the worker index 
    pub from: usize,
    pub to: usize,
    pub build:usize,
}

pub struct Board {
    pub blocks: [u8; 25],
    pub workers: [usize; 4],
    pub turn: Turn,
}

#[derive(Debug)]
enum MoveError {
    InvalidFromSquare,
    InvalidToSquare,
    InvalidBuildSquare,
    OccupiedToSquare,
    OccupiedBuildSquare,
    HeightDifferenceHigh,
}

impl Board {
    fn square_is_free(&self, square:usize) -> bool {
        self.workers[W1] != square && self.workers[W2] != square && self.workers[U1] != square && self.workers[U2] != square &&
        self.blocks[square] < 4
    }

    fn move_is_legal(&self, mv: Move) -> Result<(), MoveError> {
        if mv.from != W1 && mv.from != W2 && mv.from != U1 && mv.from != U2 {
            return Err(MoveError::InvalidFromSquare);
        }
    
        if mv.to < A1 || mv.to > E5 {
            return Err(MoveError::InvalidToSquare);
        }

        if mv.build < A1 || mv.to > E5 {
            return Err(MoveError::InvalidBuildSquare);
        }

        if !self.square_is_free(mv.to){
            return Err(MoveError::OccupiedToSquare)
        }

        if !self.square_is_free(mv.build){
            return Err(MoveError::OccupiedBuildSquare)
        }

        if (self.squares[to] - self.squares[from]) > 1{
            return Err(MoveError::HeightDifferenceHigh)
        }
    
        Ok(())
    }

    fn make_move(&mut self, mv: Move){
        let sq = self.workers[mv.from];
        self.worker[mv.from] = mv.to;
        self.blocks[mv.build] += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_from_square() {
        let mut board = Board {
            blocks: [0, 0, 0, 0, 0,
                     0, 1, 0, 0, 0,
                     0, 4, 0, 2, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0],
            workers: [C4, D4, B3, C3],
            turn: Turn::W,
        };

        let invalid_move = Move { from: -1, to: D5, build: C5 };
        assert_eq!(board.move_is_legal(invalid_move), Err(MoveError::InvalidFromSquare));
    }
}