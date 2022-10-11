// Module definitions
pub mod core {
    pub mod args;
    pub mod config;
    pub mod scaffold;
    pub mod timing;
}

// Re-exports
pub use crate::core::args;
pub use crate::core::config;
pub use crate::core::scaffold;
pub use crate::core::timing;
