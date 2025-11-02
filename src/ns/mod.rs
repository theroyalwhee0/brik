/// Default namespace configuration and injection.
pub mod defaults;
/// Error types for namespace operations.
mod error;

pub use defaults::{NsDefaults, NsDefaultsBuilder};
pub use error::{NsError, NsResult};
