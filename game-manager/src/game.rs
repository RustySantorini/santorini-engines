use engines::{squares::*, Board, Engine, Move, Request, SearchResult};
use std::time::{Duration, Instant};

pub fn run_game(engines: [Box<dyn Engine> ; 2], time: Duration) {
    let mut board = Board {
        blocks: Default::default(),
        workers: [B3, C2, C4, D3],
        turn: 0,
    };

    let start = Instant::now();
    'game: loop {
        let SearchResult {
            mv: Move {
                from,
                to,
                build,
            },
            eval,
            pv: _,
            depth_searched: depth,
            time_spent: _,
        } = engines[board.turn as usize].get_move(Request {
            board,
            time_left: time - (Instant::now() - start),
        });

        for i in 0..board.workers.len() {
            if board.workers[i] == from {
                board.workers[i] = to;
                break;
            }
        }
        if let Some(build) = build {
            board.blocks[build] += 1;
            println!("{from} -> {to} ({build}) Eval= {} depth = {} ", eval.unwrap(), depth.unwrap());
        } else {
            println!("{from} -> {to} ({})", eval.unwrap());
        }

        for worker in board.workers {
            if board.blocks[worker] == 3 {
                break 'game println!("Player {} wins!", board.turn);
            }
        }

        if board.turn == 0 {
            board.turn = 1;
        } else {
            board.turn = 0;
        }
    }
}