#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Uint(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Int(isize),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Null,
}

macro_rules! impl_from {
    ($from:ty => $variant:ident) => {
        impl From<$from> for Value {
            fn from(other: $from) -> Self {
                Self::$variant(other)
            }
        }

        impl From<Option<$from>> for Value {
            fn from(other: Option<$from>) -> Self {
                other.map_or(Self::Null, Into::into)
            }
        }
    };
}

impl_from!(u8 => U8);
impl_from!(u16 => U16);
impl_from!(u32 => U32);
impl_from!(u64 => U64);
impl_from!(usize => Uint);
impl_from!(i8 => I8);
impl_from!(i16 => I16);
impl_from!(i32 => I32);
impl_from!(i64 => I64);
impl_from!(isize => Int);
impl_from!(bool => Bool);
impl_from!(String => String);
impl_from!(Vec<Value> => List);

impl<T> FromIterator<T> for Value
where
    T: Into<Self>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::List(iter.into_iter().map(Into::into).collect())
    }
}
