use std::collections::HashMap;
use std::ops::Add;
use std::time::Duration;
use std::time::Instant;

use crate::BenchmarkRequest;
use crate::helpers::print_with_timestamp;
use crate::helpers::turn::*;
use crate::spectre::board_rep::*;
use crate::spectre::eval::*;
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
#[derive(Clone, Debug)]
struct TTEntry{
    pub depth: u8,
    pub flag: char,
    pub value: isize,
    pub mv: Move,
}

fn prepare_to_benchmark(func: fn(SearchRequest) -> SearchResult) -> impl Fn(BenchmarkRequest) -> SearchResult {
    move |benchmark_request| {
        let internal_board = Board {
            blocks: benchmark_request.position.blocks,
            workers: benchmark_request.position.workers,
            turn: benchmark_request.position.turn,
        };

        let request = SearchRequest {
            position: internal_board,
            max_depth: benchmark_request.max_depth,
            time_left: None,
            debug: true,
        };
        func(request)
    }
}
pub fn spectre_v1_benchmark(br:BenchmarkRequest)-> SearchResult{
    prepare_to_benchmark(get_move)(br)
}

pub fn spectre_v2_benchmark(br:BenchmarkRequest)-> SearchResult{
    prepare_to_benchmark(get_move_full_tt)(br)
}
    
fn get_color(node:&Board) -> isize{
    match node.turn {
        W => 1,
        U => -1,
        _ => unreachable!(),
    }
} 

fn alphabeta_tt (
    node: &mut Board,
    depth: usize,
    ply: usize,
    mut alpha: isize,
    beta: isize,
    nodes_searched: usize,
    stop_at: Instant,
    tt: &mut HashMap<Board, Move>,
) -> isize {
    if nodes_searched % CHECK_CLOCK_EVERY == 0 && Instant::now() > stop_at {
        return -BIG_ENOUGH_VALUE;
    }

    let color = get_color(node);

    if node.game_is_over() {
        return -BIG_ENOUGH_VALUE - (depth - ply) as isize;
    }

    if ply == depth {
        return color * eval(node);
    }

    let mut value = -BIG_ENOUGH_VALUE * 100;

    if let Some(mv) = tt.get(node).cloned() {
        node.make_move(mv);
        let new_value = -alphabeta_tt(node, depth, ply + 1, -beta, -alpha, nodes_searched + 1, stop_at, tt);
        node.undo_move(mv);

        if new_value > value {
            value = new_value;
            tt.insert(*node, mv);
        }

        if value > alpha {
            alpha = value;
        }

        if alpha >= beta {
            return value;
        }
    }

    let moves = node.generate_moves();

    if moves.is_empty() {
        return -BIG_ENOUGH_VALUE - (depth - ply) as isize;
    }

    for mv in moves {
        if let Some(m) = tt.get_mut(node) {
            if *m == mv {
                continue;
            }
        }

        node.make_move(mv);
        let new_value = -alphabeta_tt(node, depth, ply + 1, -beta, -alpha, nodes_searched + 1, stop_at, tt);
        node.undo_move(mv);

        if new_value > value {
            value = new_value;
        }

        if value > alpha {
            alpha = value;
            tt.insert(*node, mv);
        }

        if alpha >= beta {
            break;
        }
    }

    value
}

fn get_move(request: SearchRequest) -> SearchResult{ 
    let thinking_time = request.time_left.unwrap_or(Duration::from_secs(10000));


    let current_time = Instant::now();
    let limit_time = current_time.add(thinking_time);

    let mut running = true;
    let mut board = Board {
        blocks: request.position.blocks,
        workers: request.position.workers,
        turn: request.position.turn,
    };
    let mut tt: HashMap<Board, Move> = HashMap::new();
    let mut depth = 0;
    let mut best_score = -BIG_ENOUGH_VALUE;

    while running {
        depth += 1;
        if request.debug{
            print_with_timestamp(&format!("Starting depth: {}", depth));
        }
        let result = alphabeta_tt(&mut board, depth, 0, -BIG_ENOUGH_VALUE, BIG_ENOUGH_VALUE, 0, limit_time, &mut tt);
        
        if result > best_score{
            best_score = result;
        }

        if depth == request.max_depth {
            running = false;
        } 
    }
    let end_time = Instant::now();
    let time_spent_thinking = end_time - current_time;

    let best= tt.get(&board);

    let best_move = best;
    let best_score = best_score;

    if request.debug{
        print_with_timestamp(&format!("Best move: {:?} Score: {} Depth: {}", best_move, best_score, depth));
    }

    SearchResult {
        mv: convert_move(board, *(best_move.unwrap_or(&Move{from:0, to:0, build:0}))),
        eval: Some(best_score),
        pv: None,
        time_spent: Some(time_spent_thinking),
        depth_searched: Some(depth),
    }
    
}


fn alphabeta_full_tt (
    node: &mut Board,
    depth: usize,
    ply: usize,
    mut alpha: isize,
    mut beta: isize,
    nodes_searched: usize,
    stop_at: Instant,
    tt: &mut HashMap<Board, TTEntry>,
) -> isize {
    if nodes_searched % CHECK_CLOCK_EVERY == 0 && Instant::now() > stop_at {
        return -BIG_ENOUGH_VALUE;
    }

    let color = get_color(node);

    if node.game_is_over() {
        return -BIG_ENOUGH_VALUE - (depth - ply) as isize;
    }

    if ply == depth {
        return color * eval(node);
    }

    let alpha_orig = alpha;
    let entry_opt = tt.get(node).cloned();

    let mut value = -BIG_ENOUGH_VALUE * 100;
    let mut best_move = Move{from: 0, build: 0, to:0};

    match entry_opt{
        Some(entry) => {
            if entry.depth == (depth - ply) as u8{
                if entry.flag == 'E'{
                    return entry.value as isize;
                }
                else if entry.flag == 'U' && (entry.value as isize) > alpha{
                    alpha = entry.value as isize;
                }
                else if entry.flag == 'L' && (entry.value as isize) < beta{
                    beta = entry.value as isize;
                }
            }

            node.make_move(entry.mv);
            let new_value = -alphabeta_full_tt(node, depth, ply + 1, -beta, -alpha, nodes_searched + 1, stop_at, tt);
            node.undo_move(entry.mv);

            if new_value > value {
                value = new_value;
                best_move = entry.mv;
            }

            if value > alpha {
                alpha = value;
            }

            if alpha >= beta {
                let flag =
                    if value <= alpha_orig {'U'}
                    else if value >= beta {'L'}
                    else{'E'};

                let new_entry =
                TTEntry{
                    depth: (depth - ply) as u8,
                    flag: flag,
                    value: value,
                    mv: entry.mv,
                };
                tt.insert(*node, new_entry);
                return value;
            }
        }
        None => (),
    }

    let moves = node.generate_moves();

    if moves.is_empty() {
        return -BIG_ENOUGH_VALUE - (depth - ply) as isize;
    }
    for mv in moves {
        node.make_move(mv);
        let new_value = -alphabeta_full_tt(node, depth, ply + 1, -beta, -alpha, nodes_searched + 1, stop_at, tt);
        node.undo_move(mv);

        if new_value > value {
            value = new_value;
            best_move = mv;
        }

        if value > alpha {
            alpha = value;
        }

        if alpha >= beta {
            break;
        }
    }
    let flag =
        if value <= alpha_orig {'U'}
        else if value >= beta {'L'}
        else{'E'};

    let new_entry =
    TTEntry{
        depth: (depth - ply) as u8,
        flag: flag,
        value: value ,
        mv: best_move,
    };
    tt.insert(*node, new_entry);
    value
}

fn get_move_full_tt(request: SearchRequest) -> SearchResult{ 
    let thinking_time = request.time_left.unwrap_or(Duration::from_secs(10000));


    let current_time = Instant::now();
    let limit_time = current_time.add(thinking_time);

    let mut running = true;
    let mut board = Board {
        blocks: request.position.blocks,
        workers: request.position.workers,
        turn: request.position.turn,
    };
    let mut tt: HashMap<Board, TTEntry> = HashMap::new();
    let mut depth = 0;
    let mut best_score = -BIG_ENOUGH_VALUE;

    while running {
        depth += 1;
        if request.debug{
            print_with_timestamp(&format!("Starting depth: {}", depth));
        }
        let result = alphabeta_full_tt(&mut board, depth, 0, -BIG_ENOUGH_VALUE, BIG_ENOUGH_VALUE, 0, limit_time, &mut tt);
        
        if result > best_score{
            best_score = result;
        }

        if depth == request.max_depth {
            running = false;
        } 
    }
    let end_time = Instant::now();
    let time_spent_thinking = end_time - current_time;

    let entry= tt.get(&board).unwrap();

    let best_score = best_score;

    if request.debug{
        print_with_timestamp(&format!("Best move: {:?} Score: {} Depth: {}", entry.mv, best_score, depth));
    }

    SearchResult {
        mv: convert_move(board, entry.mv),
        eval: Some(best_score),
        pv: None,
        time_spent: Some(time_spent_thinking),
        depth_searched: Some(depth),
    }
    
}



pub fn get_best_move(request: SearchRequest) -> SearchResult{
    get_move_full_tt(request)
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
            };
        let depth = 4;
        let best_move = Move {from: D5, to:C4, build: B3};
        assert_eq!(get_best_move_test(board, depth), best_move);
    }
    #[test]
    fn evaluation(){
        let board = Board{
            blocks: [1, 4, 0, 3, 2,
                    3, 0, 0, 2, 3,
                    4, 0, 0, 0, 2,
                    0, 0, 4, 0, 0,
                    2, 0, 1, 1, 0],
            workers: [C3, C2, B2, D2],
            turn: W,
        };
        let depth = 6;
        let best_move = Move{from: 12, to:18, build:19};
        assert_eq!(get_best_move_test(board, depth), best_move);

    }
}