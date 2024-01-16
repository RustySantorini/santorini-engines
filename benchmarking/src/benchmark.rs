pub(crate) mod benchmark{
    use engines::Flop;
    use engines::Engine;
    use engines::Request;
    use engines::flop;
    use engines::flop::search::SearchRequest;
    use engines::get_best_move;

    use crate::sql_helpers;

    fn get_engine(id_searcher:usize)-> fn(engines::flop::search::SearchRequest) -> engines::SearchResult{
        match id_searcher{
            1 => get_best_move,
            _ => unimplemented!(),
        }
    }

    pub fn run_test(id_searcher:usize, id_position:usize, depth:usize){
        let func = get_engine(id_searcher);
        let board = sql_helpers::read_position_from_id(id_position).unwrap();
        let flop_board = flop::BoardRepBoard{
            blocks: board.blocks,
            workers: board.workers,
            turn: board.turn,
            moves: vec![],
        };
        let request = SearchRequest{
            position: flop_board,
            max_depth: depth,
            time_left: None,
        };

        let result = func(request);
        let search_results = sql_helpers::SearchResult{
            id_position: id_position,
            vl_depth: depth,
            vl_evaluation: result.eval.unwrap(),
            id_searcher: id_searcher,
            vl_search_duration: result.time_spent.unwrap().as_nanos() as usize,
        };
        sql_helpers::insert_search_result(search_results);
    }

}