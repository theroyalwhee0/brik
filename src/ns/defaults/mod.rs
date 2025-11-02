/// Builder for namespace defaults.
mod builder;
/// NsDefaults implementation.
mod nsdefaults;
/// HTML preamble parsing for namespace injection.
pub mod parse;

pub use builder::NsDefaultsBuilder;
pub use nsdefaults::NsDefaults;
