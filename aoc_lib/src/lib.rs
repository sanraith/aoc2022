// Module definitions
pub mod core {
    pub mod collect_solutions;
    pub mod helpers;
    pub mod js_interop;
    pub mod solution;
    pub mod util;
}
pub mod solutions;

#[cfg(test)]
mod tests {
    mod solutions;
    mod util;
}

// Re-exports
pub use crate::core::helpers;
pub use crate::core::solution;
pub use crate::core::util;
