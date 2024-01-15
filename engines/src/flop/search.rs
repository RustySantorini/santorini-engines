use std::isize::MIN;
use std::ops::Add;
use std::time::Duration;
use std::time::SystemTime;

use crate::helpers::squares::*;
use crate::helpers::workers::*;
use crate::helpers::turn::*;
use crate::flop::board_rep::*;
use crate::flop::eval::*;
use crate::models::SearchResult;

use super::convert_move;
use super::time_management::get_time;

const BIG_ENOUGH_VALUE:isize = 10000;

pub struct SearchRequest{
    position:Board,
    max_depth:usize,
    time_left:Option<Duration>,
}

pub fn negamax (node:&mut Board, depth:usize) -> isize{
    let color =
        match node.turn {
            W => 1,
            U => -1,
            _ => unreachable!(),
        };
    match node.moves.last() {
        Some(last) => {
            if node.blocks[last.to] == 3 {
                return -BIG_ENOUGH_VALUE - depth as isize;
            }
        }
        None => {}
    }   
    if depth == 0{
        return color * eval(node);      
    }
    let mut value = -BIG_ENOUGH_VALUE * 100;
    let moves = node.generate_moves();
    if moves.len() == 0{
        value = -BIG_ENOUGH_VALUE - depth as isize;
    }
    for mv in moves{
        node.make_move(mv);
        let new_value = -negamax(node, depth-1);
        if new_value > value{
            value = new_value;
        }
        node.undo_move(mv);
    }
    value

}


fn get_best_move(request: SearchRequest) -> SearchResult {
    let thinking_time = match request.time_left {
        Some(duration) => get_time(duration),
        None => std::time::Duration::from_secs(0), // No thinking time if None
    };

    let current_time = SystemTime::now();
    let limit_time = current_time.add(thinking_time);

    let mut running = true;
    let mut board = Board {
        blocks: request.position.blocks,
        workers: request.position.workers,
        turn: request.position.turn,
        moves: vec![],
    };
    let available_moves = board.generate_moves();
    let num_moves = available_moves.len();
    let best_move = available_moves[0];
    let mut scores: Vec<isize> = vec![MIN; num_moves];
    let mut depth = 1;

    while running {
        for i in 0..num_moves {
            board.make_move(available_moves[i]);
            scores[i] = -negamax(&mut board, depth - 1);
            board.undo_move(available_moves[i]);
        }

        if depth == request.max_depth {
            running = false;
        } else if let Some(duration) = request.time_left {
            if SystemTime::now() > limit_time {
                running = false;
            }
        }

        depth += 1;
    }

    if let Some((index, &max_value)) = scores.iter().enumerate().max_by_key(|&(_, value)| value) {
        // dbg!(&available_moves);
        // dbg!(&scores);
        let mv = available_moves[index];
        SearchResult {
            mv: convert_move(board, mv),
            eval: Some(max_value),
            pv: None,
        }
    } else {
        SearchResult {
            mv: convert_move(board, best_move),
            eval: None,
            pv: None,
        }
    }
}


fn get_best_move_test(board:Board, depth:usize) -> Move{
    let request = SearchRequest{
        position:board,
        max_depth: depth,
        time_left: None
    };
    let mv = get_best_move(request).mv;
    Move{from: mv.from, to:mv.to, build: mv.build.unwrap_or(mv.from)}
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
                moves: vec![],
            };
        let depth = 1;
        let best_move = Move {from: C2, to:B2, build: C2};
        assert_eq!(get_best_move_test(board, depth), best_move);
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
                moves: vec![],
            };
        let depth = 2;
        let best_move = Move {from: C3, to:C4, build: D5};
        assert_eq!(get_best_move_test(board, depth), best_move);
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
                moves: vec![],
            };
        let depth = 2;
        let best_move = Move {from: D1, to:C1, build: B2};
        assert_eq!(get_best_move_test(board, depth), best_move);
    }
    #[test]
    fn mi2_dw (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 0, 1, 0, 0,
                         0, 0, 2, 0, 0,
                         0, 0, 2, 3, 0,
                         0, 0, 0, 0, 0],
                workers: [B3, C2, D5, E5],
                turn: W,
                moves: vec![],
            };
        let depth = 3;
        let best_move = Move {from: B3, to:C3, build: D3};
        assert_eq!(get_best_move_test(board, depth), best_move);
    }

    #[test]
    fn mi2_dw_blue (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 0,
                         0, 0, 1, 0, 0,
                         0, 0, 2, 0, 0,
                         0, 0, 2, 3, 0,
                         0, 0, 0, 0, 0],
                workers: [D5, E5, B3, C2],
                turn: U,
                moves: vec![],
            };
        let depth = 3;
        let best_move = Move {from: B3, to:C3, build: D3};
        assert_eq!(get_best_move_test(board, depth), best_move);
    }

    #[test]
    fn mi2_fa (){
        let board = 
            Board {
                blocks: [0, 0, 0, 0, 2,
                         0, 0, 0, 2, 2,
                         0, 0, 2, 0, 0,
                         0, 0, 0, 0, 0,
                         0, 0, 0, 0, 0],
                workers: [C3, A4, C4, B3],
                turn: W,
                moves: vec![],
            };
        let depth = 3;
        let best_move = Move {from: C3, to:B4, build: A5};
        assert_eq!(get_best_move_test(board, depth), best_move);
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
                moves: vec![],
            };
        let depth = 4;
        let best_move = Move {from: C5, to:D4, build: E3};
        assert_eq!(get_best_move_test(board, depth), best_move);
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
                moves: vec![],
            };
        let depth = 4;
        let best_move = Move {from: D5, to:C4, build: B3};
        assert_eq!(get_best_move_test(board, depth), best_move);
    }
}