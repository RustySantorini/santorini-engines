pub mod flop;
pub mod strange;
pub mod spectre;
pub mod helpers;
mod models;

pub use crate::flop::Flop;
pub use crate::models::*;

pub use crate::flop::flop_v1_benchmark;
pub use crate::flop::flop_v2_benchmark;
pub use crate::helpers::*;

pub fn get_engine(name: &str) -> Option<Box<dyn Engine>> {
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