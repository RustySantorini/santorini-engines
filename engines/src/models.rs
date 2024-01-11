use std::time::Duration;

pub struct EngineInfo {
    pub name: String,
}

pub struct Board {
    pub blocks: [u8; 25],
    pub workers: [usize; 4],
}

pub struct Move {
    pub from: usize,
    pub to: usize,
    pub build: Option<usize>,
}

pub struct Request {
    pub board: Board,
    pub time_left: Duration,
}

pub trait Engine {
    fn new() -> Self where Self: Sized;

    fn get_info(&self) -> EngineInfo;
    fn get_move(&self, request:Request) -> Move;
}