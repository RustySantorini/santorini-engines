mod models;

use crate::models::Engine;

pub fn get_engine<T: Engine>(name: &str) -> Option<T> {
    match name {
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