use crate::helpers::squares::*;
use crate::helpers::workers::*;
use crate::helpers::turn::*;
use crate::flop::board_rep::*;
use crate::flop::eval::*;

const BIG_ENOUGH_VALUE:isize = 10000;

fn negamax (node:&mut Board, depth:usize) -> isize{
    let color =
        match node.turn {
            W => 1,
            U => -1,
            _ => unreachable!(),
        };
    if depth == 0{
        return color * eval(node);      
    }
    let mut value = -BIG_ENOUGH_VALUE;
    let moves = node.generate_moves();
    if moves.len() == 0{
        value = (-BIG_ENOUGH_VALUE - depth as isize);
    }
    for mv in moves{
        if node.blocks[mv.to] == 3{
            value = (BIG_ENOUGH_VALUE + depth as isize);
            break;
        }
        node.make_move(mv);
        let new_value = -negamax(node, depth-1);
        if new_value > value{
            value = new_value;
        }
        node.undo_move(mv);
    }
    value

}


fn get_best_move(board:Board, depth:usize) -> Move{
    let mut initial_node = board.clone();
    let moves = board.generate_moves();
    let mut best_move = moves[0];
    let mut best_score = -BIG_ENOUGH_VALUE;
    for mv in moves{
        if initial_node.blocks[mv.to] == 3{
            return mv;
        }
        initial_node.make_move(mv);
        let score = -negamax(&mut initial_node, depth-1);
        if score > best_score{
            best_score = score;
            best_move = mv;
        }
        initial_node.undo_move(mv);
    }
    best_move
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
    fn prevent_m1(){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 3, 0, 0, 0,
                         0, 2, 0, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [D1, E5, C2, D2],
                turn: W,
            };
        let depth = 2;
        let best_move = Move {from: D1, to:C1, build: B2};
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
                workers: [B3, C2, D5, E5],
                turn: W,
            };
        let depth = 3;
        let best_move = Move {from: B3, to:C3, build: B3};
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
                         0, 0, 1, 0, 1,
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