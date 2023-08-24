#[derive(Debug, Copy, Clone)]
pub enum LexerOffset {
    Value,
    Type,
    Line,
}

impl Into<u8> for LexerOffset {
    fn into(self) -> u8 {
        match self {
            LexerOffset::Value => 0,
            LexerOffset::Type => 1,
            LexerOffset::Line => 2,
        }
    }
}
