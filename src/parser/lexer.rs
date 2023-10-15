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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ScanBuf<'src> {
    /// There is currently nothing in the buffer.
    #[default]
    Empty,
    /// The buffer contains a single character.
    Char(char),
    /// The buffer contains a slice.
    Slice(Box<&'src str>),
    /// No more text can be consumed.
    EndOfFile,
}

impl<'src> ScanBuf<'src> {
    #[must_use]
    pub fn eq_char(&self, ch: char) -> bool {
        matches!(self, Self::Char(x) if *x == ch)
    }

    #[must_use]
    pub fn eq_slice(&self, slice: &'src str) -> bool {
        matches!(self, Self::Slice(x) if **x == slice)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Scanner<'src> {
    source: &'src str,
    cursor: Cursor,
}

impl<'src> Scanner<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            cursor: Cursor::new(),
        }
    }

    pub fn buffer(&mut self) -> ScanBuf<'src> {
        let buf = match self.cursor {
            Cursor::Index(_) => ScanBuf::Empty,
            Cursor::Char(index) => self
                .source
                .chars()
                .nth(index)
                .map_or(ScanBuf::EndOfFile, ScanBuf::Char),
            Cursor::Slice(index, length) => self
                .source
                .get(index..index + length)
                .map_or(ScanBuf::EndOfFile, |slice| ScanBuf::Slice(Box::new(slice))),
        };
        self.cursor.reset();
        buf
    }

    pub fn take(&mut self, count: usize) -> &mut Self {
        self.cursor.extend(count);
        self
    }

    #[must_use]
    pub fn peek(&self, count: usize) -> ScanBuf<'src> {
        let mut scan = *self;
        scan.cursor.reset();
        scan.take(count).buffer()
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

    #[test]
    fn test_scanner_take() {
        let mut scan = Scanner::new("123456789");
        assert!(scan.take(1).buffer().eq_char('1'));
        assert!(scan.take(1).buffer().eq_char('2'));
        assert!(scan.take(3).buffer().eq_slice("345"));
        assert_eq!(scan.buffer(), ScanBuf::Empty);
        assert!(scan.take(4).buffer().eq_slice("6789"));
        assert_eq!(scan.take(1).buffer(), ScanBuf::EndOfFile);
    }

    #[test]
    fn test_scanner_peek() {
        let mut scan = Scanner::new("123456789");
        assert_eq!(scan.buffer(), ScanBuf::Empty);
        assert!(scan.peek(0).eq_slice(""));
        assert!(scan.peek(1).eq_char('1'));
        assert!(scan.peek(1).eq_char('1'));
        assert!(scan.peek(2).eq_slice("12"));
        assert_eq!(scan.buffer(), ScanBuf::Empty);
        assert!(scan.take(3).buffer().eq_slice("123"));
        assert!(scan.take(1).buffer().eq_char('4'));
        assert_eq!(scan.take(9).buffer(), ScanBuf::EndOfFile);
    }
}
