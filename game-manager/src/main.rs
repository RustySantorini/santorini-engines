mod game;

use std::{io::{stdin, stdout, Write}, time::Duration};

use engines::*;

use crate::game::run_game;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();

    println!("{:?}", get_engine_names());

    let engine1 = loop {
        print!("Which engine will be player 1? > ");
        stdout().flush()?;

        buffer.clear();
        stdin().read_line(&mut buffer)?;

        if let Some(engine) = get_engine(buffer.trim_end()) {
            break engine;
        } else {
            println!("Invalid name!");
        }
    };
    let engine2 = loop {
        print!("Which engine will be player 2? > ");
        stdout().flush()?;

        buffer.clear();
        stdin().read_line(&mut buffer)?;

        if let Some(engine) = get_engine(buffer.trim_end()) {
            break engine;
        } else {
            println!("Invalid name!");
        }
    };

    run_game([engine1, engine2], Duration::from_secs(60));

    Ok(())
}
