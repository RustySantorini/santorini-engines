use crate::helpers::squares::*;
use crate::helpers::workers::*;
use crate::helpers::turn::*;
use crate::flop::board_rep::*;

fn get_best_move(board:Board, depth:usize) -> Move{
    Move{from:0, to:0, build:0}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn m1(){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 3, 0, 0, 0,
                         0, 2, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [C3, C2, C4, B3],
                turn: W,
            };
        let depth = 1;
        let best_move = Move {from: C2, to:B2, build: C2};
        assert_eq!(get_best_move(board, depth), best_move);
    }
    #[test]
    fn m1_5(){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 0, 0, 2, 2,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 2, 1,
                         0, 0, 0, 3, 0],
                workers: [C3, A4, A5, E5],
                turn: W,
            };
        let depth = 2;
        let best_move = Move {from: C3, to:C4, build: D5};
        assert_eq!(get_best_move(board, depth), best_move);
    }
    #[test]
    fn mi2_dw (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 0, 2, 0, 0,
                         0, 0, 2, 0, 0,
                         0, 0, 2, 3, 0,
                         0, 0, 0, 0, 0],
                workers: [B2, C4, A5, B5],
                turn: W,
            };
        let depth = 3;
        let best_move = Move {from: B1, to:C1, build: B2};
        assert_eq!(get_best_move(board, depth), best_move);
    }

    #[test]
    fn mi2_dw_blue (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 0, 2, 0, 0,
                         0, 0, 2, 0, 0,
                         0, 0, 2, 3, 0,
                         0, 0, 0, 0, 0],
                workers: [D5, E5, B3, C2],
                turn: U,
            };
        let depth = 3;
        let best_move = Move {from: B3, to:C3, build: B3};
        assert_eq!(get_best_move(board, depth), best_move);
    }

    #[test]
    fn mi2_fa (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 3,
                         0, 0, 0, 2, 1,
                         0, 0, 2, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [C3, A4, C4, B3],
                turn: W,
            };
        let depth = 3;
        let best_move = Move {from: C3, to:B4, build: B5};
        assert_eq!(get_best_move(board, depth), best_move);
    }

    #[test]
    fn prevent_mi2 (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 3,
                         0, 0, 0, 2, 1,
                         0, 0, 2, 0, 1,
                         0, 0, 0, 3, 0,
                         0, 0, 0, 0, 0],
                workers: [B3, D5, C3, A4],
                turn: W,
            };
        let depth = 4;
        let best_move = Move {from: D5, to:C5, build: D4};
        assert_eq!(get_best_move(board, depth), best_move);
    }
    #[test]
    fn m2_5 (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 4, 1,
                         4, 4, 0, 2, 0,
                         4, 0, 1, 3, 0],
                workers: [C5, D3, E2, D5],
                turn: W,
            };
        let depth = 4;
        let best_move = Move {from: C5, to:D4, build: E3};
        assert_eq!(get_best_move(board, depth), best_move);
    }
    #[test]
    fn stalling (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 2, 3, 0, 0,
                         0, 2, 0, 0, 0,
                         2, 3, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [B1, D5, A2, B2],
                turn: W,
            };
        let depth = 4;
        let best_move = Move {from: D5, to:C4, build: B3};
        assert_eq!(get_best_move(board, depth), best_move);
    }
    #[test]
    fn m3_zz (){
        let board = 
            Board {
                blocks: [1, 2, 2, 2, 2,
                         0, 0, 4, 0, 2,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [A1, A5, B2, C4],
                turn: W,
            };
        let depth = 5;
        let best_move = Move {from: A1, to:A2, build: A3};
        assert_eq!(get_best_move(board, depth), best_move);
    }
}