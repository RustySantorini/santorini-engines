use std::time::Duration;

pub struct EngineInfo {
    pub name: String,
    pub eval_range: (isize, isize),
}

#[derive(Debug)]
pub struct SearchResult{
    pub mv: Move,
    pub eval: Option<isize>,
    pub pv: Option<Vec<Move>>,
    pub depth_searched: Option<usize>,
    pub time_spent: Option<Duration>,
}

pub struct BenchmarkRequest{
    pub position: Board,
    pub max_depth: usize,
    pub debug: bool,
}

#[derive(Clone, Copy)]
pub struct Board {
    pub blocks: [u8; 25],
    pub workers: [usize; 4],
    pub turn: u8,
}


#[derive(Debug)]
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
    fn get_move(&self, request:Request) -> SearchResult;
}