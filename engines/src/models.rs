use num_enum::IntoPrimitive;
use std::time::Duration;

pub struct EngineInfo {
    pub name: String
}

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum Square {
    A1,
    A2,
    A3,
    A4,
    A5,
    B1,
    B2,
    B3,
    B4,
    B5,
    C1,
    C2,
    C3,
    C4,
    C5,
    D1,
    D2,
    D3,
    D4,
    D5,
    E1,
    E2,
    E3,
    E4,
    E5,
}

#[derive(IntoPrimitive)]
#[repr(usize)]
pub enum Worker {
    W1,
    W2,
    U1,
    U2,
}

pub struct Board {
    pub blocks: [u8; 25],
    pub workers: [Square; 4],
}

pub struct Move {
    pub from: usize,
    pub to: usize,
    pub build:usize,
}

pub struct Request {
    pub board: Board,
    pub time_left: Duration,
}

pub trait Engine {
    fn get_info(&self) -> EngineInfo;
    fn get_move(request:Request) -> Move;
}