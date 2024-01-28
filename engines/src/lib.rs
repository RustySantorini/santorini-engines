mod helpers;
mod models;

use phf::{Map, phf_map};

// Engines

mod bogo;
mod flop;
// mod spectre;
// mod strange;

const ENGINE_REGISTRY: Map<&'static str, fn() -> Box<dyn Engine>> = phf_map! {
    "bogo" => || Box::new(bogo::new()),
    "flop" => || Box::new(flop::new()),
    // "spectre" => || Box::new(spectre::Spectre::new()),
    // "strange" => || Box::new(strange::Strange::new()),
};

// Public

pub use models::*;

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