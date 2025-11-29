// Modules
pub mod compile;
pub mod parse;

// Exports
pub use parse::{Atom, Stmt, parse};
pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
