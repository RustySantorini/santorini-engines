use crate::helpers::Squares::*;
use crate::helpers::Workers::*;
use crate::helpers::Turn::*;

fn get_neighbors(square: usize) -> Vec<usize> {
    match square {
        A1 => vec![A2, B1, B2],
        A2 => vec![A1, A3, B1, B2, B3],
        A3 => vec![A2, A4, B2, B3, B4],
        A4 => vec![A3, A5, B3, B4, B5],
        A5 => vec![A4, B4, B5],

        B1 => vec![A1, A2, B2, C1, C2],
        B2 => vec![A1, A2, A3, B1, B3, C1, C2, C3],
        B3 => vec![A2, A3, A4, B2, B4, C2, C3, C4],
        B4 => vec![A3, A4, A5, B3, B5, C3, C4, C5],
        B5 => vec![A4, A5, B4, C4, C5],

        C1 => vec![B1, B2, C2, D1, D2],
        C2 => vec![B1, B2, B3, C1, C3, D1, D2, D3],
        C3 => vec![B2, B3, B4, C2, C4, D2, D3, D4],
        C4 => vec![B3, B4, B5, C3, C5, D3, D4, D5],
        C5 => vec![B4, B5, C4, D4, D5],

        D1 => vec![C1, C2, D2, E1, E2],
        D2 => vec![C1, C2, C3, D1, D3, E1, E2, E3],
        D3 => vec![C2, C3, C4, D2, D4, E2, E3, E4],
        D4 => vec![C3, C4, C5, D3, D5, E3, E4, E5],
        D5 => vec![C4, C5, D4, E4, E5],

        E1 => vec![D1, D2, E2],
        E2 => vec![D1, D2, D3, E1, E3],
        E3 => vec![D2, D3, D4, E2, E4],
        E4 => vec![D3, D4, D5, E3, E5],
        E5 => vec![D4, D5, E4],

        _ => vec![]
    }
}

pub struct HalfMove{
    pub from: usize,
    pub to: usize,
}

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

    fn half_move_is_legal(&self, hm: HalfMove) -> Result<(), MoveError> {
        if hm.from != W1 && hm.from != W2 && hm.from != U1 && hm.from != U2 {
            return Err(MoveError::InvalidFromSquare);
        }
    
        if hm.to > E5 {
            return Err(MoveError::InvalidToSquare);
        }

        if !self.square_is_free(hm.to){
            return Err(MoveError::OccupiedToSquare)
        }

        if (self.blocks[hm.to] - self.blocks[hm.from]) > 1{
            return Err(MoveError::HeightDifferenceHigh)
        }

        if (self.turn == W && (hm.from == U1 || hm.from == U2)) || (self.turn == U && (hm.from == W1 || hm.from == W2)){
            return Err(MoveError::WorkerOfWrongColor)
        }
        if !get_neighbors(hm.to).contains(&self.workers[hm.from]){
            return Err(MoveError::ToSquareInaccessible);
        }
        Ok(())
    }

    fn move_is_legal(&self, mv: Move) -> Result<(), MoveError> {
        let half_move = HalfMove{from: mv.from, to: mv.to};

        self.half_move_is_legal(half_move)?;

        if mv.build > E5 {
            return Err(MoveError::InvalidBuildSquare);
        }

        if (!self.square_is_free(mv.build) && mv.build != self.workers[mv.from]) || (mv.build == mv.to){ 
            return Err(MoveError::OccupiedBuildSquare)
        }

        if !get_neighbors(mv.build).contains(&mv.to){
            return Err(MoveError::BuildSquareInaccessible);
        }
        Ok(())
    }

    fn make_move(&mut self, mv: Move){
        let sq = self.workers[mv.from];
        self.workers[mv.from] = mv.to;
        self.blocks[mv.build] += 1;
    }

    // fn generate_half_moves(&self){
    //     let from_squares = match self.turn {
    //         W => vec![&self.workers[0], &self.workers[1]],
    //         U => vec![&self.workers[2], &self.workers[3]],
    //         _ => panic!("NR")
    //     };

    //     from_squares
    //     .iter()
    //     .map(|&from| {
    //         let square = self.workers[from]
    //     })
    //     .collect();
    // }

    fn generate_moves(&self)-> Vec<Move>{
        vec![]
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
    
    #[test]
    fn opening_position() {
        let board = Board {
            blocks: [0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0],
            workers: [B3, C3, C2, C4],
            turn: W,
        };
        let len_moves = board.generate_moves().len();
        assert_eq!(len_moves, 59);
    }
    #[test]
    fn trapped_worker() {
        let board = Board {
            blocks: [0, 0, 0, 2, 0,
                     0, 0, 0, 0, 3,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0],
            workers: [A5, C3, B3, B4],
            turn: W,
        };
        let len_moves = board.generate_moves().len();
        assert_eq!(len_moves, 44);
    }
    #[test]
    fn winning_move() {
        let board = Board {
            blocks: [0, 0, 0, 0, 0,
                     0, 3, 0, 0, 0,
                     0, 2, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0],
            workers: [C2, C3, B3, B4],
            turn: W,
        };
        let len_moves = board.generate_moves().len();
        assert_eq!(len_moves, 58 );
    }
    #[test]
    fn domed() {
        let board = Board {
            blocks: [0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     4, 4, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0],
            workers: [A1, E1, D2, E2],
            turn: W,
        };
        let len_moves = board.generate_moves().len();
        assert_eq!(len_moves, 15);
    }
    #[test]
    fn multiple_heights() {
        let board = Board {
            blocks: [0, 0, 0, 0, 0,
                     0, 0, 2, 0, 0,
                     0, 1, 2, 1, 0,
                     0, 0, 3, 0, 0,
                     0, 0, 0, 0, 0],
            workers: [C2, C4, B3, A3],
            turn: W,
        };
        let len_moves = board.generate_moves().len();
        assert_eq!(len_moves, 70);
    }
    #[test]
    fn zero_moves() {
        let board = Board {
            blocks: [1, 0, 0, 0, 0,
                     1, 3, 0, 0, 0,
                     4, 0, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0],
            workers: [A1, B1, A2, C2],
            turn: W,
        };
        let len_moves = board.generate_moves().len();
        assert_eq!(len_moves, 0);
    }
}