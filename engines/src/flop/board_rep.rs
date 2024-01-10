use crate::helpers::Squares::*;
use crate::helpers::Workers::*;
use crate::helpers::Turn::*;


pub struct Move {
    // From is the worker index 
    pub from: usize,
    pub to: usize,
    pub build:usize,
}

pub struct Board {
    pub blocks: [u8; 25],
    pub workers: [usize; 4],
    pub turn: u8,
}

#[derive(Debug, PartialEq)]
enum MoveError {
    InvalidFromSquare,
    InvalidToSquare,
    InvalidBuildSquare,
    OccupiedToSquare,
    OccupiedBuildSquare,
    HeightDifferenceHigh,
    ToSquareInaccessible,
    BuildSquareInaccessible,
    WorkerOfWrongColor,
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
    
        if mv.to > E5 {
            return Err(MoveError::InvalidToSquare);
        }

        if mv.build > E5 {
            return Err(MoveError::InvalidBuildSquare);
        }

        if !self.square_is_free(mv.to){
            return Err(MoveError::OccupiedToSquare)
        }

        if (!self.square_is_free(mv.build) && mv.build != self.workers[mv.from]) || (mv.build == mv.to){ 
            return Err(MoveError::OccupiedBuildSquare)
        }

        if (self.blocks[mv.to] - self.blocks[mv.from]) > 1{
            return Err(MoveError::HeightDifferenceHigh)
        }

        if (self.turn == W && (mv.from == U1 || mv.from == U2)) || (self.turn == U && (mv.from == W1 || mv.from == W2)){
            return Err(MoveError::WorkerOfWrongColor)
        }
    
        Ok(())
    }

    fn make_move(&mut self, mv: Move){
        let sq = self.workers[mv.from];
        self.workers[mv.from] = mv.to;
        self.blocks[mv.build] += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_board_1: Board = Board {
        blocks: [0, 0, 0, 0, 0,
                 0, 1, 0, 0, 0,
                 0, 4, 0, 2, 0,
                 0, 0, 0, 0, 0,
                 0, 0, 1, 0, 2],
        workers: [C4, D4, B3, C3],
        turn: W,
    };
    const test_board_2: Board = Board {
        blocks: [0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0,
                 0, 0, 0, 0, 0],
        workers: [A1, E5, A5, E1],
        turn: U,
    };

    #[test]
    fn invalid_from_square() {
        let mut board = test_board_1;

        let mv = Move { from: 5, to: D5, build: D1 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::InvalidFromSquare));
    }
    #[test]
    fn invalid_to_square() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: 25, build: D4 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::InvalidToSquare));
    }
    #[test]
    fn invalid_build_square() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: D5, build: 25 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::InvalidBuildSquare));
    }
    #[test]
    fn to_square_far_away() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: A1, build: A2 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::ToSquareInaccessible));
    }
    #[test]
    fn build_square_far_away() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: D5, build: A2 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::BuildSquareInaccessible));
    }
    #[test]
    fn to_square_occupied() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: C3, build: D3 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::OccupiedToSquare));
    }
    #[test]
    fn build_square_occupied() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: D3, build: C3 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::OccupiedBuildSquare));
    }
    #[test]
    fn move_to_self() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: D4, build: D3 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::OccupiedToSquare));
    }
    #[test]
    fn build_in_previous_square() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: D5, build: D4 };
        assert_eq!(board.move_is_legal(mv), Ok(()));
    }
    #[test]
    fn build_in_new_square() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: D5, build: D5 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::OccupiedBuildSquare));
    }
    #[test]
    fn height_difference() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: E5, build: E4 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::HeightDifferenceHigh));
    }
    #[test]
    fn wrong_color() {
        let mut board = test_board_1;

        let mv = Move { from: U1, to: A3, build: A2 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::WorkerOfWrongColor));
    }
    #[test]
    fn wrong_color_2() {
        let mut board = test_board_2;

        let mv = Move { from: W1, to: A2, build: A3 };
        assert_eq!(board.move_is_legal(mv), Err(MoveError::WorkerOfWrongColor));
    }
    #[test]
    fn normal_move() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: D5, build: E4 };
        assert_eq!(board.move_is_legal(mv), Ok(()));
    }
    #[test]
    fn diagonal_move() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: C5, build: B5 };
        assert_eq!(board.move_is_legal(mv), Ok(()));
    }
    #[test]
    fn climbing_move() {
        let mut board = test_board_1;

        let mv = Move { from: W2, to: E3, build: E2 };
        assert_eq!(board.move_is_legal(mv), Ok(()));
    }
    #[test]
    fn jumping_move() {
        let mut board = test_board_1;

        let mv = Move { from: W1, to: C5, build: B5 };
        assert_eq!(board.move_is_legal(mv), Ok(()));
    }
}