use engines::*;
use std::time::{Duration, Instant};

pub fn run_game(engines: [Box<dyn Engine> ; 2], time: Duration) {
    let mut board = Board::new([B3, C2], [C4, D3]);
    let mut times = [time, time];
    let winner = loop {
        let turn = *board.get_turn();
        let start = Instant::now();
        let mv = engines[turn].get_move(board.get_request(times[turn]));
        let delta = Instant::now() - start;

        if delta > times[turn] {
            println!("Timeout!");
            break turn.next();   
        }
        times[turn] -= delta;

        board.apply_move(mv);
        println!("{:#?} {}: {}", times[turn], turn, mv);

        if let Some(winner) = board.get_winner() {
            break *winner;
        }
    };
    println!("{} wins!", winner);
}