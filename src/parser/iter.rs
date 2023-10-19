#[derive(Clone, Debug)]
pub struct Buffered<S, T>
where
    S: Iterator<Item = T>,
{
    iter: S,
    buffer: Vec<T>,
}

impl<S, T> Buffered<S, T>
where
    S: Iterator<Item = T>,
{
    pub fn new<I>(from: I) -> Self
    where
        I: IntoIterator<Item = u8, IntoIter = S>,
    {
        Self {
            iter: from.into_iter(),
            buffer: Vec::new(),
        }
    }

    /// Consume up to `count` items from the internal iterator, moving them into
    /// the buffer. Return an optional reference to the buffer's items.
    ///
    /// If the iterator did not contain enough items to satisfy `count`, `None`
    /// will be returned. In this case, the only way to get the remaining items
    /// out is by consuming the iterator normally.
    pub fn buffer(&mut self, count: usize) -> Option<&[S::Item]> {
        if self.buffer.len() < count {
            self.buffer
                .extend(self.iter.by_ref().take(count - self.buffer.len()));
        }
        self.buffer.get(0..count)
    }
}

impl<S, T> Iterator for Buffered<S, T>
where
    S: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop().or_else(|| self.iter.next())
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
    pub fn new<I>(from: I) -> Self
    where
        I: IntoIterator<Item = u8, IntoIter = S>,
    {
        Self(from.into_iter())
    }
}

impl<S> Iterator for SourceChars<S>
where
    S: Iterator<Item = u8>,
{
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::with_capacity(4);
        for byte in self.0.by_ref() {
            buf.push(byte);
            if let Ok(slice) = std::str::from_utf8(&buf) {
                return slice.chars().next();
            }
        }
        None
    }
}

pub trait Peekable {
    type Item<'item>
    where
        Self: 'item;
    type ItemSlice<'items>
    where
        Self: 'items;

    fn peek(&mut self) -> Option<Self::Item<'_>>;
    fn look(&mut self, count: usize) -> Option<Self::ItemSlice<'_>>;
}

impl<I, T> Peekable for Buffered<I, T>
where
    I: Iterator<Item = T>,
{
    type Item<'item> = &'item T where I: 'item, T: 'item;
    type ItemSlice<'items> = &'items [T] where I: 'items, T: 'items;

    fn peek(&mut self) -> Option<Self::Item<'_>> {
        self.buffer(1).and_then(|slice| slice.get(0))
    }

    fn look(&mut self, count: usize) -> Option<Self::ItemSlice<'_>> {
        self.buffer(count)
    }
}
