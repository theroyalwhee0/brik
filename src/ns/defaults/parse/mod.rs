/// Function to parse HTML preamble and locate the html tag.
mod parse_preamble;
/// Pest parser for HTML preamble.
mod preamble;
/// Information extracted from parsing the HTML tag.
mod tag_info;

pub use parse_preamble::parse_preamble;
pub use tag_info::HtmlTagInfo;
