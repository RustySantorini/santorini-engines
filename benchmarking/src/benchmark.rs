use engines::flop_v1_benchmark;
use engines::BenchmarkRequest;
use engines::flop_v2_benchmark;
use crate::sql_helpers;

fn get_engine(id_searcher:usize)-> fn(BenchmarkRequest) -> engines::SearchResult{
    match id_searcher{
        1 => flop_v1_benchmark,
        2 => flop_v2_benchmark,
        _ => unimplemented!(),
    }
}

pub fn run_test(id_searcher:usize, id_position:usize, depth:usize)-> Result<(), rusqlite::Error>{
    let func = get_engine(id_searcher);
    let board = sql_helpers::read_position_from_id(id_position).unwrap();
    let request = BenchmarkRequest{
        position: board,
        max_depth: depth,
        debug:true,
    };

    let result = func(request);
    let search_results = sql_helpers::SearchResult{
        id_position: id_position,
        vl_depth: depth,
        vl_evaluation: result.eval.unwrap(),
        id_searcher: id_searcher,
        vl_search_duration: result.time_spent.unwrap().as_nanos() as usize,
    };
    sql_helpers::insert_search_result(search_results)
}

