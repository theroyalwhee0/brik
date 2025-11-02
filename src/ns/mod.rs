/// Default namespace configuration and injection.
pub mod defaults;
/// Error types for namespace operations.
mod error;

pub use defaults::NamespaceDefaults;
pub use error::{NsError, NsResult};
