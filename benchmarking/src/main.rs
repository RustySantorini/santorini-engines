use benchmark::run_test;
use sql_helpers::create_new_position;
use engines::{Board, squares::*, turn::*};

mod sql_helpers;
mod benchmark;

fn main() {
    // run_test(4, 15, 6);
    let mut depth = 1;
    loop{
        for _ in 0..4{
            for s in 1..3{
                let _ = run_test(s, 15, depth);
            }
        }
        depth += 1;
    }

    // let board = Board{
    //     blocks: [1, 4, 0, 3, 2,
    //              3, 0, 0, 2, 3,
    //              4, 0, 0, 0, 2,
    //              0, 0, 4, 0, 0,
    //              2, 0, 1, 1, 0],
    //     workers: [C3, C2, B2, D2],
    //     turn: W,
    // };

    // create_new_position(board, 'F');

}
