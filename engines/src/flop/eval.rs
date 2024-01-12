use crate::flop::board_rep::*;
use crate::helpers::squares::*;
use crate::helpers::workers::*;
use crate::helpers::turn::*;

fn position_height (board: &Board, p:fn (&Board, usize) -> usize, a:usize, b:usize, c:usize, worker_pos:usize) -> usize {
    let h = board.blocks[worker_pos];
    a * b.pow(h.into()) + c * p(board, worker_pos)
}

fn num_neighbors(_:&Board, worker_pos:usize) -> usize{
    get_neighbors(worker_pos).len()
}

fn neighbor_height(board:&Board, a:usize, b:usize, c:usize, worker_pos:usize) -> usize {
    position_height(board, num_neighbors, a, b, c, worker_pos)
}

fn nh_s(board:&Board, worker_pos:usize) -> usize{
    neighbor_height(board, 6, 2, 1, worker_pos)
}

pub fn eval (board: &Board) -> isize {
    (nh_s(board, board.workers[W1]) + nh_s(board, board.workers[W2])) as isize -
    (nh_s(board, board.workers[U1]) + nh_s(board, board.workers[U2])) as isize
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn starting_position (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [C3, C4, B3, D3],
                turn: W,
            };

        assert_eq!(eval(&board), 0);
    }
    #[test]
    fn simple_height (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 1, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 2, 0,
                         0, 0, 0, 0, 0],
                workers: [B2, B4, D2, D4],
                turn: W,
            };

        assert!(eval(&board) < 0);
    }
    #[test]
    fn simple_pos (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [C2, C1, C4, A1],
                turn: W,
            };

        assert!(eval(&board) > 0);
    }
    #[test]
    fn h2_over_h1 (){
        let board = 
            Board {
                blocks: [2, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 1, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [A1, C4, C3, D4],
                turn: W,
            };

        assert!(eval(&board) > 0);
    }
    #[test]
    fn h_not_all (){
        let board = 
            Board {
                blocks: [2, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 1, 0,
                         0, 0, 0, 1, 0,
                         0, 0, 0, 0, 0],
                workers: [A1, A5, C4, D4],
                turn: W,
            };

        assert!(eval(&board) < 0);
    }
    #[test]
    fn border_over_corner (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [A1, C4, A2, D4],
                turn: W,
            };

        assert!(eval(&board) < 0);
    }
    #[test]
    fn center_over_border (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [A2, C4, B2, D4],
                turn: W,
            };

        assert!(eval(&board) < 0);
    }
    #[test]
    fn height_over_pos (){
        let board = 
            Board {
                blocks: [1, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [A1, C4, A3, D4],
                turn: W,
            };

        assert!(eval(&board) > 0);
    }
}
