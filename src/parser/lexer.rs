#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cursor {
    /// The cursor is in a position, but corresponds to no characters.
    Index(usize),
    /// An index, corresponds to a character.
    Char(usize),
    /// An index and a length, corresponds with a slice.
    Slice(usize, usize),
}

impl Cursor {
    fn new() -> Self {
        Self::Index(0)
    }

    /// Get the index of the cursor.
    fn index(&self) -> usize {
        match *self {
            Self::Index(index) | Self::Char(index) | Self::Slice(index, _) => index,
        }
    }

    /// Get the length of the cursor, zero if it is an index.
    fn len(&self) -> usize {
        match *self {
            Self::Index(_) => 0,
            Self::Char(_) => 1,
            Self::Slice(_, length) => length,
        }
    }

    /// Reset the cursor to the next position so that it is ready for another
    /// scan.
    fn reset(&mut self) -> &mut Self {
        *self = match *self {
            Self::Index(_) => *self,
            Self::Char(index) => Self::Index(index + 1),
            Self::Slice(index, length) => Self::Index(index + length),
        };
        self
    }

    /// Extend the cursor by `count` characters.
    fn extend(&mut self, count: usize) -> &mut Self {
        *self = match *self {
            Self::Index(index) if count == 1 => Self::Char(index),
            Self::Index(index) => Self::Slice(index, count),
            Self::Char(index) => Self::Slice(index, 1 + count),
            Self::Slice(index, length) => Self::Slice(index, length + count),
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor() {
        let mut cursor = Cursor::new();
        assert_eq!(cursor.len(), 0);
        assert_eq!(cursor.extend(2).len(), 2);
        assert_eq!(cursor.extend(3).len(), 5);
        assert_eq!(cursor.reset().index(), 5);
        assert_eq!(cursor.reset().index(), 5);
        assert_eq!(cursor.extend(1), &Cursor::Char(5));
        assert_eq!(cursor.len(), 1);
        assert_eq!(cursor.extend(1), &Cursor::Slice(5, 2));
        assert_eq!(cursor.extend(3), &Cursor::Slice(5, 5));
        assert_eq!(cursor.reset().index(), 10);
    }
}
