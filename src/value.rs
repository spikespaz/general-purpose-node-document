use std::error::Error;

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
    ($($from:ty => $variant:ident),+) => {
        $(impl_from!($from, $variant);)*
    };
    ($from:ty, $variant:ident) => {
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

impl_from!(
    u8 => U8,
    u16 => U16,
    u32 => U32,
    u64 => U64,
    usize => Uint,
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
    isize => Int,
    bool => Bool,
    String => String,
    Vec<Value> => List
);

impl<T> FromIterator<T> for Value
where
    T: Into<Self>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::List(iter.into_iter().map(Into::into).collect())
    }
}

#[derive(Clone, Debug)]
pub struct IntoInnerError {
    variant: &'static str,
    into: &'static str,
}

impl std::fmt::Display for IntoInnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "could not convert `Value` to `{}` because it was not of variant `{}`",
            self.into, self.variant
        )
    }
}

impl Error for IntoInnerError {}

macro_rules! impl_try_into_inner {
    ($($variant:ident => $inner:ty),+) => {
        $(impl_try_into_inner!($variant, $inner);)*
    };
    ($variant:ident, $inner:ty) => {
        impl TryFrom<Value> for $inner {
            type Error = IntoInnerError;

            fn try_from(value: Value) -> Result<$inner, Self::Error> {
                match value {
                    Value::$variant(value) => Ok(value),
                    _ => Err(IntoInnerError {
                        variant: stringify!($variant),
                        into: stringify!($inner),
                    }),
                }
            }
        }
    };
}

impl_try_into_inner!(
    U8 => u8,
    U16 => u16,
    U32 => u32,
    U64 => u64,
    Uint => usize,
    I8 => i8,
    I16 => i16,
    I32 => i32,
    I64 => i64,
    Int => isize,
    Bool => bool,
    String => String,
    List => Vec<Value>
);
