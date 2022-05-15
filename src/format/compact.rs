use super::Formatter;

/// This structure compacts a Lua Table with no extra whitespace.
#[derive(Clone, Debug)]
pub struct CompactFormatter;

impl Formatter for CompactFormatter {}
