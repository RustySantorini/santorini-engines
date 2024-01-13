mod board_rep;
mod eval;
mod time_management;
mod search;
use crate::{Engine, models::{EngineInfo, Move, Request, SearchResult}};

pub struct Flop {
}
impl Engine for Flop {
    fn new() -> Self where Self: Sized{
        Flop {}
    }

    fn get_info(&self) -> EngineInfo {
        EngineInfo {
            name: String::from("flop"),
            eval_range: (-100, 100),
        }
    }
    fn get_move(&self, _request: Request) -> SearchResult {
        let m = 
            Move {
                from: 0,
                to: 0,
                build: Some(0),
            };
        SearchResult{
            mv: m,
            eval: Some(0),
            pv: None
        }
    }
}