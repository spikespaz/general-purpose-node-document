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
        if self.buffer.is_empty() {
            self.buffer.push(self.iter.next()?);
        }
        self.buffer.get(0)
    }

    fn look(&mut self, count: usize) -> Option<Self::ItemSlice<'_>> {
        if self.buffer.len() < count {
            self.buffer
                .extend(self.iter.by_ref().take(count - self.buffer.len()));
        }
        self.buffer.get(0..count)
    }
}
