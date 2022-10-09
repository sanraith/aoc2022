// Module definitions
pub mod api {
    pub mod cli;
    pub mod js_interop;
    pub mod solution;
}
pub mod solutions;

#[cfg(test)]
mod tests {
    mod solutions;
}

// Re-exports
pub use api::cli;
pub use api::solution;
