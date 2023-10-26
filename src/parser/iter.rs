use std::collections::VecDeque;

pub trait Buffered: Iterator {
    type ItemSlice<'a>
    where
        Self: 'a;

    /// Consume up to `count` items from the internal iterator, moving them into
    /// the buffer. Return an optional reference to the buffer's items.
    ///
    /// If the iterator did not contain enough items to satisfy `count`, `None`
    /// will be returned. In this case, the only way to get the remaining items
    /// out is by consuming the iterator normally.
    fn buffer(&mut self, count: usize) -> Option<Self::ItemSlice<'_>>;
}

pub trait Peekable {
    type Item<'item>
    where
        Self: 'item;
    type ItemSlice<'items>
    where
        Self: 'items;

    /// Peek at a single item in this iterator, without consuming.
    fn peek(&mut self) -> Option<Self::Item<'_>>;

    /// Peek at multiple items in this iterator, without consuming.
    fn look(&mut self, count: usize) -> Option<Self::ItemSlice<'_>>;
}

#[derive(Clone, Debug)]
pub struct SourceBytes<S>
where
    S: Iterator<Item = u8>,
{
    iter: S,
    buffer: VecDeque<S::Item>,
}

impl<S> SourceBytes<S>
where
    S: Iterator<Item = u8>,
{
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = S::Item, IntoIter = S>,
    {
        Self {
            iter: iter.into_iter(),
            buffer: VecDeque::new(),
        }
    }
}

impl<S> Iterator for SourceBytes<S>
where
    S: Iterator<Item = u8>,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop_front().or_else(|| self.iter.next())
    }
}

impl<S> Buffered for SourceBytes<S>
where
    S: Iterator<Item = u8>,
{
    type ItemSlice<'a> = &'a [u8] where S: 'a;

    fn buffer(&mut self, count: usize) -> Option<Self::ItemSlice<'_>> {
        if self.buffer.len() < count {
            self.buffer
                .extend(self.iter.by_ref().take(count - self.buffer.len()));
        }
        self.buffer.make_contiguous().get(0..count)
    }
}

impl<S> Peekable for SourceBytes<S>
where
    S: Iterator<Item = u8>,
{
    type Item<'item> = u8 where Self: 'item;
    type ItemSlice<'items> = &'items [u8] where Self: 'items;

    fn peek(&mut self) -> Option<Self::Item<'_>> {
        self.buffer(1).and_then(|slice| slice.first().copied())
    }

    fn look(&mut self, count: usize) -> Option<Self::ItemSlice<'_>> {
        self.buffer(count)
    }
}

#[derive(Clone, Debug)]
pub struct SourceChars<S>(S)
where
    S: Iterator<Item = u8>;

impl<S> SourceChars<S>
where
    S: Iterator<Item = u8>,
{
    pub fn new<I>(bytes: I) -> Self
    where
        I: IntoIterator<Item = S::Item, IntoIter = S>,
    {
        Self(bytes.into_iter())
    }
}

impl<S> Iterator for SourceChars<S>
where
    S: Iterator<Item = u8>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 4];
        // A single character can be at most 4 bytes.
        for (i, byte) in self.0.by_ref().take(4).enumerate() {
            buf[i] = byte;
            if let Ok(slice) = std::str::from_utf8(&buf[..=i]) {
                return slice.chars().next();
            }
        }
        None
    }
}

impl<S> Buffered for SourceChars<&mut S>
where
    for<'a> S: Iterator<Item = u8> + Buffered<ItemSlice<'a> = &'a [u8]> + 'a,
{
    type ItemSlice<'a> = &'a str where Self: 'a;

    // Allowed specifically here because the borrow checker is incorrect.
    #[allow(unsafe_code)]
    fn buffer(&mut self, count: usize) -> Option<Self::ItemSlice<'_>> {
        for byte_count in 0.. {
            let buf = self.0.buffer(byte_count)?;
            // SAFETY:
            //
            // This unsafe pointer coercion is here because of a limitation
            // in the borrow checker. In the future, when Polonius is merged as
            // the de-facto borrow checker, this unsafe code can be removed.
            //
            // The lifetime of the byte slice is shortened to the lifetime of
            // the return value, which lives as long as `self` does.
            //
            // This is referred to as the "polonius problem",
            // or more accurately, the "lack-of-polonius problem".
            //
            // <https://github.com/rust-lang/rust/issues/54663>
            let buf: *const [u8] = buf;
            let buf: &[u8] = unsafe { &*buf };

            if let Ok(slice) = std::str::from_utf8(buf) {
                if slice.chars().count() >= count {
                    return Some(slice);
                }
            }
        }
        unreachable!()
    }
}

impl<S> Peekable for SourceChars<S>
where
    for<'a> S: Iterator<Item = u8> + 'a,
    for<'a> Self: Buffered<ItemSlice<'a> = &'a str> + 'a,
{
    type Item<'item> = char where Self: 'item;
    type ItemSlice<'items> = &'items str where Self: 'items;

    fn peek(&mut self) -> Option<Self::Item<'_>> {
        self.buffer(1).and_then(|slice| slice.chars().next())
    }

    fn look(&mut self, count: usize) -> Option<Self::ItemSlice<'_>> {
        self.buffer(count)
    }
}

#[cfg(test)]
mod tests {
    use super::{Buffered, SourceBytes, SourceChars};

    #[test]
    fn test_source_chars() {
        let source = "abcdefg";
        let chars = SourceChars::new(source.bytes());
        assert_eq!(source, chars.collect::<String>());
    }

    #[test]
    fn test_source_chars_buffer() {
        let source = "abcdefg";
        let mut bytes = SourceBytes::new(source.bytes());
        let mut chars = SourceChars::new(&mut bytes);
        // Ensure that the `buffer` function works.
        assert_eq!(&source[0..3], chars.buffer(3).unwrap());
        // Ensure that the characters are taken from the buffer,
        // and that `buffer` correctly preserves them.
        assert_eq!(source, chars.collect::<String>());
    }
}
