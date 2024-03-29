mod board_rep;
mod eval;
mod time_management;
pub mod search;
use crate::{Move, Board, Engine, EngineInfo, Request, SearchResult};

use self::search::{SearchRequest, get_best_move};
use self::time_management::get_time;
pub use self::search::strange_v1_benchmark;

fn convert_move(board: board_rep::Board, internal_move: board_rep::Move) -> Move {
    let build = if board.blocks[internal_move.to] == 3 {
        None
    } else {
        Some(internal_move.build)
    };
    Move {
        from: internal_move.from,
        to: internal_move.to,
        build,
    }
}

fn convert_board(board: Board) -> board_rep::Board {
    board_rep::Board {
        blocks: board.blocks,
        workers: board.workers,
        turn: board.turn,
        moves: vec![],
    }
}

pub struct Strange;

impl Engine for Strange {
    fn new() -> Self
    where
        Self: Sized,
    {
        Strange {}
    }

    fn get_info(&self) -> EngineInfo {
        EngineInfo {
            name: String::from("spectre"),
            eval_range: (-46, 46),
        }
    }

    fn get_move(&self, request: Request) -> SearchResult {
        let thinking_time = get_time(request.time_left);
        let request = SearchRequest {
            position: convert_board(request.board),
            // We use a fixed depth to avoid growing to unnecessary depths when a game-ending move is found
            max_depth: 20,
            time_left: Some(thinking_time),
            debug: false,
        };
        get_best_move(request)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::helpers::squares::*;
    use crate::helpers::turn::*;

    use super::*;
    #[test]
    fn t1() {
        let board = crate::models::Board {
            blocks: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            workers: [C2, C3, C4, C5],
            turn: U,
        };
        let total_time = Duration::from_secs(60);
        let flop = Strange {};
        let mv = flop.get_move(Request {
            board,
            time_left: total_time,
        });
        dbg!(&mv);
    }
}
