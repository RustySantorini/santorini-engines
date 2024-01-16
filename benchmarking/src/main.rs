use benchmark::run_test;
use engines::Board;

mod sql_helpers;
mod benchmark;

fn main() {
    let mut depth = 4;
    loop{
        for _ in 0..10{
            let _ = run_test(1, 1, depth);
        }
        depth += 1;
    }
}
