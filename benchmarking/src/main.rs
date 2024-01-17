use benchmark::run_test;
use sql_helpers::create_new_position;
use engines::{Board, squares::*, turn::*};

mod sql_helpers;
mod benchmark;

fn main() {
    let mut depth = 1;
    loop{
        for _ in 0..4{
            let _ = run_test(1, 14, depth);
        }
        depth += 1;
    }

    // let board = Board{
    //     blocks: [0, 3, 3, 0, 3,
    //              0, 3, 0, 4, 1,
    //              0, 0, 1, 2, 4,
    //              0, 0, 4, 1, 1,
    //              0, 0, 4, 2, 0],
    //     workers: [C3, D4, C2, C4],
    //     turn: U,
    // };

    // create_new_position(board, 'F');

}
