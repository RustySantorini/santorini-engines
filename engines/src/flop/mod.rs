mod board_rep;
mod eval;
mod time_management;
mod search;
use std::isize::MIN;
use std::ops::Add;
use std::time::SystemTime;

use crate::{Engine, models::{EngineInfo, Move, Request, SearchResult}};

use self::search::negamax;
use self::{time_management::get_time, board_rep::Board};

fn convert_move(board:board_rep::Board, internal_move: board_rep::Move) -> Move{
    let build =
        if board.blocks[internal_move.to] == 3{
            None
        }

        else{
            Some(internal_move.build)
        };
    Move { from: internal_move.from, to: internal_move.to, build: build }
}

pub struct Flop {
}
impl Engine for Flop {
    fn new() -> Self where Self: Sized{
        Flop {}
    }

    fn get_info(&self) -> EngineInfo {
        EngineInfo {
            name: String::from("flop"),
            eval_range: (-46, 46),
        }
    }
    fn get_move(&self, request: Request) -> SearchResult {
        let thinking_time = get_time(request.time_left);
        let current_time = SystemTime::now();
        let limit_time = current_time.add(thinking_time);

        let mut running = true;
        let mut board = Board{
            blocks: request.board.blocks,
            workers: request.board.workers,
            turn: request.board.turn,
        };
        let available_moves = board.generate_moves();
        let num_moves = available_moves.len();
        let best_move = available_moves[0];
        let mut scores: Vec<isize> = vec![MIN; num_moves];
        let mut depth = 1;

        while running{
            for i in 0..num_moves{
                board.make_move(available_moves[i]);
                scores[i] = -negamax(&mut board, depth-1);
                board.undo_move(available_moves[i]);
                if SystemTime::now() > limit_time{
                    running = false;
                    break;
                }
            }
            depth += 1;
        }
        if let Some((index, &max_value)) = scores.iter().enumerate().max_by_key(|&(_, value)| value) {
            let best_move = available_moves[index];
            SearchResult{
                mv: convert_move(board, best_move),
                eval: Some(max_value),
                pv: None,
            }
        } else {
            SearchResult{
                mv: convert_move(board, best_move),
                eval: None,
                pv: None,
            }
        }    
    }
}