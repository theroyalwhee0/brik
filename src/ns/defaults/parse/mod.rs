/// Function to parse HTML preamble and locate the html tag.
mod parser;
/// Pest parser for HTML preamble.
mod preamble;
/// Information extracted from parsing the HTML tag.
mod tag_info;

pub use parser::parse_preamble;
pub use tag_info::{HtmlTagInfo, Span, XmlnsPositions};
