// Module definitions
pub mod core {
    pub mod js_interop;
    pub mod solution;
}
pub mod solutions;

#[cfg(test)]
mod tests {
    mod solutions;
}

// Re-exports
pub use crate::core::solution;
