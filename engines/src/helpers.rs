pub(crate) mod turn {
    pub const W: u8 = 0;
    pub const U: u8 = 1;
}

pub(crate) mod squares {
    pub const A1: usize = 0;
    pub const A2: usize = 1;
    pub const A3: usize = 2;
    pub const A4: usize = 3;
    pub const A5: usize = 4;

    pub const B1: usize = 5;
    pub const B2: usize = 6;
    pub const B3: usize = 7;
    pub const B4: usize = 8;
    pub const B5: usize = 9;

    pub const C1: usize = 10;
    pub const C2: usize = 11;
    pub const C3: usize = 12;
    pub const C4: usize = 13;
    pub const C5: usize = 14;

    pub const D1: usize = 15;
    pub const D2: usize = 16;
    pub const D3: usize = 17;
    pub const D4: usize = 18;
    pub const D5: usize = 19;

    pub const E1: usize = 20;
    pub const E2: usize = 21;
    pub const E3: usize = 22;
    pub const E4: usize = 23;
    pub const E5: usize = 24;

}

pub(crate) mod workers {
    pub const W1: usize = 0;
    pub const W2: usize = 1;
    pub const U1: usize = 2;
    pub const U2: usize = 3;
}

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

pub(crate) mod sql_helpers{
    use chrono::prelude::*;

    use crate::models::Board;
    use rusqlite::{Connection, Result, params};
    use super::*;
    struct SearchResult {
        id_position: usize,
        vl_depth: usize,
        vl_evaluation: isize,
        id_searcher: usize,
        vl_search_duration: usize,
    }
    
    fn get_current_datetime_text() -> String {
        let local: DateTime<Local> = Local::now();
        local.format("%Y:%m:%d %H:%M:%S").to_string()
    }
    
    fn insert_search_result(search_result: SearchResult) -> Result<()> {
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
        Connection::open(r#"D:\santorini\rusty-santorini-engines\engines\src\sql\santorini_db.db"#)
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
}

#[cfg(test)]
mod tests {
    use crate::helpers::squares::*;
    use crate::helpers::sql_helpers::*;
    use crate::models::Board;
    use super::*;
    use super::turn::W;
    
    #[test]
    fn hashing(){
        let w1 = [A1, A2, A3, A4];
        let w2 = [C3, C4, B3, D3];
        let w3 = [A1, A5, E1, E5];
        for i in [w1, w2, w3]{
            assert_eq!(i, unhash_workers(hash_workers(i)));
        }
    }

    #[test]
    fn insert_pos(){
        let board = Board{
            blocks: [0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0,
                     0, 0, 0, 0, 0],
            workers: [C3, C4, B3, B4],
            turn: W,
        };
        let insertion = create_new_position(board, 'A');
        dbg!(&insertion);
        assert!(insertion.is_ok());
    }
}