use std::isize::MIN;
use std::ops::Add;
use std::time::Duration;
use std::time::Instant;

use crate::BenchmarkRequest;
use crate::helpers::print_with_timestamp;
use crate::helpers::turn::*;
use crate::strange::board_rep::*;
use crate::strange::eval::*;
use crate::models::SearchResult;

use super::convert_move;
use super::time_management::get_time;

const BIG_ENOUGH_VALUE:isize = 10000;
const CHECK_CLOCK_EVERY:usize = 1000;

pub struct SearchRequest{
    pub position:Board,
    pub max_depth:usize,
    pub time_left:Option<Duration>,
    pub debug: bool,
}

fn prepare_to_benchmark() -> impl Fn(BenchmarkRequest) -> SearchResult {
    move |benchmark_request| {
        let internal_board = Board {
            blocks: benchmark_request.position.blocks,
            workers: benchmark_request.position.workers,
            turn: benchmark_request.position.turn,
            moves: vec![],
        };

        let request = SearchRequest {
            position: internal_board,
            max_depth: benchmark_request.max_depth,
            time_left: None,
            debug: true,
        };
        get_move(request)
    }
}

fn alphabeta_id (
    node:&mut Board,
    depth:usize,
    ply:usize,
    mut alpha:isize,
    beta:isize,
    last_pv: Vec<Move>,
    nodes_searched: usize, 
    stop_at: Instant,
    in_pv: bool,
)-> (isize, Vec<Move>){
    if nodes_searched % CHECK_CLOCK_EVERY == 0 && Instant::now() > stop_at {return (-BIG_ENOUGH_VALUE, vec![]);}
    let color =
        match node.turn {
            W => 1,
            U => -1,
            _ => unreachable!(),
        };
    match node.moves.last() {
        Some(last) => {
            if node.blocks[last.to] == 3 {
                return (-BIG_ENOUGH_VALUE - (depth - ply) as isize, vec![]);
            }
        }
        None => {}
    }   
    if ply == depth{
        return (color * eval(node), vec![]);  
    }
    let mut value = -BIG_ENOUGH_VALUE * 100;
    let mut pv:Vec<Move> = vec![];
    let previous_best_move =
        if depth == 1 || (ply+1 == depth) || in_pv == false{
            None
        }
        else{
            Some(last_pv[ply])
        };
    match previous_best_move{
        Some (mv) => {
            node.make_move(mv);
            let result = alphabeta_id(node, depth, ply+1, -beta, -alpha, last_pv.clone(), nodes_searched+1, stop_at, true);
            let new_value = -result.0;
            node.undo_move(mv);
            if new_value > value{
                value = new_value;
                pv = vec![mv];
                pv.extend(result.1);
            }
            if value > alpha{
                alpha = value;
            }
            if alpha >= beta{
                return (value, pv);
            }
        }
        None =>(),
    }
    let moves = node.generate_moves();
    if moves.len() == 0{
        return (-BIG_ENOUGH_VALUE - (depth - ply) as isize, vec![]);
    }
    for mv in moves{
        match previous_best_move{
            Some(m) => if mv == m{continue;}
            None => ()
        }
        node.make_move(mv);
        let result = alphabeta_id(node, depth, ply+1, -beta, -alpha, last_pv.clone(), nodes_searched+1, stop_at, false);
        let new_value = -(result.0);
        node.undo_move(mv);
        if new_value > value{
            value = new_value;
        }
        if value > alpha{
            alpha = value;
            pv = vec![mv];
            pv.extend(result.1);
        }
        if alpha >= beta{
            break;
        }
    }
    (value, pv)

}



fn get_move(request: SearchRequest) -> SearchResult{ 
    let thinking_time = match request.time_left {
        Some(duration) => get_time(duration),
        None => std::time::Duration::from_secs(10000), // No thinking time if None
    };

    let current_time = Instant::now();
    let limit_time = current_time.add(thinking_time);

    let mut running = true;
    let mut board = Board {
        blocks: request.position.blocks,
        workers: request.position.workers,
        turn: request.position.turn,
        moves: vec![],
    };
    let mut pv:Vec<Move> = vec![];
    let mut depth = 0;
    let mut best_score = -BIG_ENOUGH_VALUE;

    while running {
        depth += 1;
        if request.debug{
            print_with_timestamp(&format!("Starting depth: {}", depth));
        }
        let result = alphabeta_id(&mut board, depth, 0, -BIG_ENOUGH_VALUE, BIG_ENOUGH_VALUE, pv.clone(), 0, limit_time, true);
        pv = result.1;
        best_score = result.0;
        if depth == request.max_depth {
            running = false;
        } 
    }
    let end_time = Instant::now();
    let time_spent_thinking = end_time - current_time;

    let best= pv[0];

    let best_move = best;
    let best_score = best_score;

    // dbg!(pv_table);

    SearchResult {
        mv: convert_move(board, best_move),
        eval: Some(best_score),
        pv: None,
        time_spent: Some(time_spent_thinking),
        depth_searched: Some(depth),
    }
    
}

pub fn get_best_move(request: SearchRequest) -> SearchResult{
    get_move(request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::squares::*;

    fn get_best_move_test(board:Board, depth:usize) -> Move{
        let request = SearchRequest{
            position:board,
            max_depth: depth,
            time_left: None,
            debug: false,
        };
        let mv = get_best_move(request).mv;
        Move{from: mv.from, to:mv.to, build: mv.build.unwrap_or(mv.from)}
    }

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