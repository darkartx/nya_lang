// Pos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Default for Pos {
    fn default() -> Self {
        Self { index: 0, line: 1, column: 0 }
    }
}

impl Pos {
    pub fn new(index: usize, line: usize, column: usize) -> Self {
        Self {
            index,
            line,
            column
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub position: Pos,
    pub length: usize,
}

impl Span {
    pub fn new(position: Pos, length: usize) -> Self {
        Self {
            position,
            length,
        }
    }
}
