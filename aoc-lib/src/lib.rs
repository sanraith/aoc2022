// Module definitions
pub mod core {
    pub mod file_util;
    pub mod helpers;
    pub mod solution;
    pub mod solution_runner;
    pub mod util;
}
pub mod inputs;
pub mod solutions;

#[cfg(test)]
mod tests;

// Re-exports
pub use crate::core::helpers;
pub use crate::core::solution;
pub use crate::core::util;
