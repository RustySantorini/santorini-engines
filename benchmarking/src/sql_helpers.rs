use chrono::prelude::*;

use crate::Board;
use rusqlite::{Connection, Result, params, types::Value};
pub fn hash_workers(workers:[usize;4]) -> usize{
    let mut sum:usize = 0;
    let base:usize = 25;
    for i in 0..4{
        sum += base.pow(i) * workers[i as usize];
    }
    sum
}

pub fn unhash_workers(mut hash: usize) -> [usize; 4] {
    let mut w: [usize; 4] = [0; 4]; // Initialize the array with zeros
    let base: usize = 25;
    for i in 0..4 {
        w[i] = hash % base;
        hash /= base;
    }
    w
}
pub struct SearchResult {
    pub id_position: usize,
    pub vl_depth: usize,
    pub vl_evaluation: isize,
    pub id_searcher: usize,
    pub vl_search_duration: usize,
}

fn get_current_datetime_text() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y:%m:%d %H:%M:%S").to_string()
}

pub fn insert_search_result(search_result: SearchResult) -> Result<()> {
    let conn = get_connection()?;
    let formatted_datetime = get_current_datetime_text();

    conn.execute(
        "INSERT INTO TB_SEARCH_RESULTS (id_position, vl_depth, vl_evaluation, dh_search, id_searcher, vl_search_duration)
            VALUES (?, ?, ?, ?, ?, ?)",
        params![
            search_result.id_position,
            search_result.vl_depth,
            search_result.vl_evaluation,
            formatted_datetime,
            search_result.id_searcher,
            search_result.vl_search_duration
        ],
    )?;

    Ok(())
}

fn get_connection() -> Result<Connection>{
    Connection::open(r#"D:\santorini\rusty-santorini-engines\benchmarking\src\sql\santorini_db.db"#)
}
pub fn create_new_position (board: Board, tipo:char) -> Result<()> {
    // Open a connection to the SQLite database file
    let conn = get_connection()?;

    // Convert the blocks array to a string of 25 chars
    let blocks_str: String = board.blocks.iter().map(|&b| char::from(b)).collect();

    // Convert workers to their respective hash strings
    let workers = hash_workers(board.workers);

    // Insert data into the table
    conn.execute(
        "INSERT INTO TB_POSITION (vl_blocks, vl_workers, cd_tipo, vl_turno) VALUES (?1, ?2, ?3, ?4)",
        [&blocks_str, &(workers.to_string()), &(tipo.to_string()), &(board.turn.to_string())],
    )?;
    Ok(())
}
pub fn read_position_from_id(position_id: usize) -> Result<Board> {
    // Open a connection to the SQLite database file
    let conn = get_connection()?;

    // Prepare the SQL statement to retrieve data based on position_id
    let mut stmt = conn.prepare(
        "SELECT vl_blocks, vl_workers, cd_tipo, vl_turno FROM TB_POSITION WHERE id_position = ?1",
    )?;

    // Execute the statement and get a cursor
    let mut rows = stmt.query([Value::Integer(position_id as i64)])?;

    // Fetch the first row
    let row = rows.next()?.ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)?;

    // Extract data from the row
    let blocks_str: String = row.get(0)?;
    let workers_hash:usize = row.get(1)?;
    let turn: usize = row.get(3)?;

    let blocks: [u8; 25] = {
        let mut array = [0; 25]; // Initialize an array of size 25 with zeros
        for (index, &byte) in blocks_str.as_bytes().iter().take(25).enumerate() {
            array[index] = byte;
        }
        array
    };    
    // Convert workers_str back to a vector of Worker
    let workers = unhash_workers(workers_hash);

    // Create a Board instance
    let board = Board {
        blocks,
        workers,
        turn: turn as u8, // You might want to handle parsing errors
    };

    Ok(board) 
}
