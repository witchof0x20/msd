#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::de) struct Position {
    pub(in crate::de) line: usize,
    pub(in crate::de) column: usize,
}

impl Position {
    pub(in crate::de) fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
        }
    }
}
