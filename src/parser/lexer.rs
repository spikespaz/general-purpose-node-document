use super::iter::{SourceBytes, SourceChars};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cursor {
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
    #[must_use]
    pub fn index(&self) -> usize {
        match *self {
            Self::Index(index) | Self::Char(index) | Self::Slice(index, _) => index,
        }
    }

    /// Get the length of the cursor, zero if it is an index.
    #[must_use]
    pub fn len(&self) -> usize {
        match *self {
            Self::Index(_) => 0,
            Self::Char(_) => 1,
            Self::Slice(_, length) => length,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Index(_) | Self::Slice(_, 0))
    }

    /// Advance the cursor to the next position so that it is ready for another
    /// scan.
    pub fn advance(&mut self) -> &mut Self {
        *self = match *self {
            Self::Index(_) => *self,
            Self::Char(index) => Self::Index(index + 1),
            Self::Slice(index, length) => Self::Index(index + length),
        };
        self
    }

    /// Extend the cursor by `count` characters.
    pub fn extend(&mut self, count: usize) -> &mut Self {
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
pub enum Selection<'src> {
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

impl<'src> Selection<'src> {
    #[must_use]
    pub fn eq_char(&self, ch: char) -> bool {
        matches!(self, Self::Char(x) if *x == ch)
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn eq_slice(&self, slice: &'src str) -> bool {
        match self {
            Self::Char(buf) if slice.len() == 1 => buf == &slice.chars().nth(0).unwrap(),
            Self::Slice(buf) => buf.as_ref() == &slice,
            _ => false,
        }
    }

    /// Gets the character out of the buffer.
    ///
    /// # Panics
    /// Panics if the buffer does not contain a single character.
    /// Note that it could be a string slice with a single character, which will
    /// *not* panic.
    #[must_use]
    pub fn to_char_unchecked(self) -> char {
        match self {
            Self::Char(ch) => ch,
            Self::Slice(slice) if slice.len() == 1 => slice.chars().nth(0).unwrap(),
            _ => panic!("buffer was not a single character"),
        }
    }

    /// Gets the slice out of the buffer.
    ///
    /// # Panics
    /// Panics if the buffer does not contain a slice.
    #[must_use]
    pub fn to_slice_unchecked(&self) -> &str {
        match self {
            Self::Slice(slice) => slice.as_ref(),
            _ => panic!("buffer was not a string slice"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scanner<S>
where
    S: Iterator<Item = u8>,
{
    source: SourceBytes<S>,
    cursor: Cursor,
}

impl<S> Scanner<S>
where
    S: Iterator<Item = u8>,
{
    #[must_use]
    pub fn new<I>(source: I) -> Self
    where
        I: IntoIterator<Item = S::Item, IntoIter = S>,
    {
        Self {
            source: SourceBytes::new(source),
            cursor: Cursor::new(),
        }
    }

    #[must_use]
    pub fn cursor(&self) -> Cursor {
        self.cursor
    }

    fn source_chars(&mut self) -> impl Iterator + '_ {
        SourceChars::new(self.source.by_ref())
    }

    // #[must_use]
    // pub fn peek_byte(&mut self) -> Option<u8> {
    //     self.source.peek().copied()
    // }

    // #[must_use]
    // pub fn look_bytes(&mut self, count: usize) -> Option<&[u8]> {
    //     self.source.look(count)
    // }

    // pub fn selection(&mut self) -> Selection<'_> {
    //     let buf = match self.cursor {
    //         Cursor::Index(_) => Selection::Empty,
    //         Cursor::Char(index) => self
    //             .source
    //             .chars()
    //             .nth(index)
    //             .map_or(Selection::EndOfFile, Selection::Char),
    //         Cursor::Slice(index, length) => self
    //             .source
    //             .get(index..index + length)
    //             .map_or(Selection::EndOfFile, |slice| {
    //                 Selection::Slice(Box::new(slice))
    //             }),
    //     };
    //     self.cursor.advance();
    //     buf
    // }

    // pub fn take(&mut self, count: usize) -> &mut Self {
    //     self.cursor.extend(count);
    //     self
    // }

    // #[must_use]
    // pub fn peek(&self, count: usize) -> Selection<'_> {
    //     let mut scan = *self;
    //     scan.cursor.advance();
    //     scan.take(count).selection()
    // }
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
        assert_eq!(cursor.advance().index(), 5);
        assert_eq!(cursor.advance().index(), 5);
        assert_eq!(cursor.extend(1), &Cursor::Char(5));
        assert_eq!(cursor.len(), 1);
        assert_eq!(cursor.extend(1), &Cursor::Slice(5, 2));
        assert_eq!(cursor.extend(3), &Cursor::Slice(5, 5));
        assert_eq!(cursor.advance().index(), 10);
    }

    // #[test]
    // fn test_scanner_take() {
    //     let mut scan = Scanner::new("123456789");
    //     assert!(scan.take(1).selection().eq_char('1'));
    //     assert!(scan.take(1).selection().eq_char('2'));
    //     assert!(scan.take(3).selection().eq_slice("345"));
    //     assert_eq!(scan.selection(), Selection::Empty);
    //     assert!(scan.take(4).selection().eq_slice("6789"));
    //     assert_eq!(scan.take(1).selection(), Selection::EndOfFile);
    // }

    // #[test]
    // fn test_scanner_peek() {
    //     let mut scan = Scanner::new("123456789");
    //     assert_eq!(scan.selection(), Selection::Empty);
    //     assert!(scan.peek(0).eq_slice(""));
    //     assert!(scan.peek(1).eq_char('1'));
    //     assert!(scan.peek(1).eq_char('1'));
    //     assert!(scan.peek(2).eq_slice("12"));
    //     assert_eq!(scan.selection(), Selection::Empty);
    //     assert!(scan.take(3).selection().eq_slice("123"));
    //     assert!(scan.take(1).selection().eq_char('4'));
    //     assert_eq!(scan.take(9).selection(), Selection::EndOfFile);
    // }
}
