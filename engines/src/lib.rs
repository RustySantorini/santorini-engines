mod flop;
mod helpers;
mod models;

use phf::{Map, phf_map};

pub mod flop;
pub mod strange;
pub mod spectre;
pub mod helpers;

pub use crate::flop::Flop;

pub use crate::models::*;

pub use crate::flop::flop_v1_benchmark;
pub use crate::flop::flop_v2_benchmark;
pub use crate::helpers::*;

pub fn get_engine_names() -> Vec<&'static str> {
    ENGINE_REGISTRY.keys().map(|x| *x).collect()
}
pub fn get_engine(name: &str) -> Option<Box<dyn Engine>> {
    if let Some(constructor) = ENGINE_REGISTRY.get(name) {
        Some(constructor())
    } else {
        None
    } 
}

const ENGINE_REGISTRY: Map<&'static str, fn() -> Box<dyn Engine>> = phf_map! {
    "flop" => || Box::new(Flop::new()),
};