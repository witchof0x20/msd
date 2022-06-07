#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::de) struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub(in crate::de) fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub(in crate::de) fn increment_line(self) -> Self {
        Self {
            line: self.line + 1,
            column: 0,
        }
    }

    pub(in crate::de) fn increment_column(self) -> Self {
        Self {
            line: self.line,
            column: self.column + 1,
        }
    }

    pub(in crate::de) fn decrement_column(self) -> Self {
        Self {
            line: self.line,
            column: self.column - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn increment_line() {
        let position = Position::new(5, 7);

        assert_eq!(position.increment_line(), Position::new(6, 0));
    }

    #[test]
    fn increment_column() {
        let position = Position::new(5, 7);

        assert_eq!(position.increment_column(), Position::new(5, 8));
    }

    #[test]
    fn decrement_column() {
        let position = Position::new(5, 7);

        assert_eq!(position.decrement_column(), Position::new(5, 6));
    }
}
