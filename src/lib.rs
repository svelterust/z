// Modules
pub mod compile;
pub mod parse;

// Exports
pub use compile::compile;
pub use parse::{Atom, Node, Statement, parse};
pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
