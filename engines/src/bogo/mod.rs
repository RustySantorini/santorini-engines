use rand::{seq::SliceRandom, thread_rng};

use crate::*;

pub struct Bogo;
pub fn new() -> Bogo {
    Bogo {}
}
impl Engine for Bogo {
    fn get_info(&self) -> EngineInfo {
        EngineInfo {
            name: "bogo".to_string(),
            eval_range: (0, 0),
        }
    }

    fn get_search_result(&self, request: Request) -> SearchResult {
        let mut moves = Vec::new();

        for from in Square::squares() {
            if let Some(Worker { turn }) = request.workers[from] {
                if turn == *request.turn {
                    for to in from.get_neighbours() {
                        if request.blocks[from].is_reachable(&request.blocks[to]) && request.workers[to].is_none() {
                            for at in to.get_neighbours() {
                                if request.blocks[at] != T4 && request.workers[at].is_none() {
                                    moves.push(Move{ from, to, at: Some(at) });
                                }
                            }
                        }
                    }
                }
            }
        }

        SearchResult {
            mv: *moves.choose(&mut thread_rng()).unwrap(),
            eval: None,
            pv: None,
            depth_searched: None,
            time_spent: None,
        }
    }
}