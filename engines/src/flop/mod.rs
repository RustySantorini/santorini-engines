mod board_rep;
mod eval;
mod time_management;
pub mod search;
use crate::{Engine, EngineInfo, Move, Request, SearchResult, Square, Turn, Worker};

use self::search::{SearchRequest, get_best_move};
use self::time_management::get_time;
pub use self::search::flop_v1_benchmark;
pub use self::search::flop_v2_benchmark;

fn convert_move(board: board_rep::Board, internal_move: board_rep::Move) -> Move {
    let at = if board.blocks[internal_move.to] == 3 {
        None
    } else {
        Some(internal_move.build.try_into().unwrap())
    };
    Move {
        from: internal_move.from.try_into().unwrap(),
        to: internal_move.to.try_into().unwrap(),
        at,
    }
}

fn convert_board(request: Request) -> board_rep::Board {
    let mut workers = [0 ; 4];
    let mut index1 = 0;
    let mut index2 = 0;
    for square in Square::squares() {
        if let Some(Worker { turn }) = request.workers[square] {
            match turn {
                Turn::P1 => {
                    workers[index1] = square.into();
                    index1 += 1;
                }
                Turn::P2 => {
                    workers[index2] = square.into();
                    index2 += 1;
                }
            }
        }
    }

    board_rep::Board {
        blocks: request.blocks.map(|x| Into::<usize>::into(x) as u8),
        workers,
        turn: Into::<usize>::into(*request.turn) as u8,
        moves: Vec::new(),
    }
}

pub struct Flop;
impl Flop {
    pub fn new() -> Self
    {
        Flop {}
    }

}

impl Engine for Flop {
    fn get_info(&self) -> EngineInfo {
        EngineInfo {
            name: String::from("flop"),
            eval_range: (-46, 46),
        }
    }

    fn get_search_result(&self, request: Request) -> SearchResult {
        let thinking_time = get_time(request.time_left);
        let request = SearchRequest {
            position: convert_board(request),
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
        let flop = Flop {};
        let mv = flop.get_move(Request {
            board,
            time_left: total_time,
        });
        dbg!(&mv);
    }
}
