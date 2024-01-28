use engines::*;
use std::time::{Duration, Instant};

pub fn run_game(engines: [Box<dyn Engine> ; 2], time: Duration) {
    let mut board = Board::new([B3, C2], [C4, D3]);
    let mut times = [time, time];
    
}