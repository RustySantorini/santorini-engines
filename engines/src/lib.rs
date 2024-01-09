mod flop;
mod models;

pub use crate::flop::Flop;

use crate::models::Engine;

pub fn get_engine<T: Engine>(name: &str) -> Option<Box<dyn Engine>> {
    match name {
        "flop" => Some(Box::new(Flop::new())),
        _ => None,
    }
}


// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }