use crate::{Engine, models::{EngineInfo, Move, Request}};

pub struct Flop {
}
impl Engine for Flop {
    fn new() -> Self where Self: Sized{
        Flop {}
    }

    fn get_info(&self) -> EngineInfo {
        EngineInfo {
            name: String::from("flop"),
        }
    }
    fn get_move(&self, _request: Request) -> Move {
        Move {
            from: 0,
            to: 0,
            build: 0,
        }
    }
}