use std::borrow::Cow;
use std::error::Error;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value<'borrow> {
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
    String(Cow<'borrow, str>),
    List(Vec<Value<'borrow>>),
    Null,
}

impl crate::Sealed for Value<'_> {}

macro_rules! ignore {
    ($ignore:tt, $instead:tt) => {
        $instead
    };
}

macro_rules! impl_kind {
    ($($variant:ident$($inner:ty)?),+) => {
        impl Value<'_> {
            pub const fn kind(&self) -> &'static str {
                match self {
                    $(Self::$variant$((ignore!($inner, _)))? => stringify!($variant),)*
                }
            }
        }
    };
}

impl_kind!(
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
    String(Cow<'borrow, str>),
    List(Vec<Value>),
    Null
);

macro_rules! impl_from {
    ($($(#[$meta:meta])* $from:ty => $variant:ident),+) => {
        $(impl_from!($(#[$meta])*, $from, $variant);)*
    };
    ($(#[$meta:meta])*, $from:ty, $variant:ident) => {
        impl<'borrow> From<$from> for Value<'borrow> {
            $(#[$meta])*
            fn from(other: $from) -> Self {
                Self::$variant(other)
            }
        }

        impl<'borrow> From<Option<$from>> for Value<'borrow> {
            $(#[$meta])*
            fn from(other: Option<$from>) -> Self {
                other.map_or(Self::Null, Into::into)
            }
        }
    };
}

impl_from!(
    /// Direct conversion from `u8` to the variant `Value::U8`.
    u8 => U8,
    /// Direct conversion from `u16` to the variant `Value::U16`.
    u16 => U16,
    /// Direct conversion from `u32` to the variant `Value::U32`.
    u32 => U32,
    /// Direct conversion from `u64` to the variant `Value::U64`.
    u64 => U64,
    /// Direct conversion from `usize` to the variant `Value::Uint`.
    usize => Uint,
    /// Direct conversion from `i8` to the variant `Value::I8`.
    i8 => I8,
    /// Direct conversion from `i16` to the variant `Value::I16`.
    i16 => I16,
    /// Direct conversion from `i32` to the variant `Value::I32`.
    i32 => I32,
    /// Direct conversion from `i64` to the variant `Value::I64`.
    i64 => I64,
    /// Direct conversion from `isize` to the variant `Value::Int`.
    isize => Int,
    /// Direct conversion from `f32` to the variant `Value::F32`.
    f32 => F32,
    /// Direct conversion from `f64` to the variant `Value::F64`.
    f64 => F64,
    /// Direct conversion from `bool` to the variant `Value::Bool`.
    bool => Bool,
    /// Direct conversion from `Vec<Value>` to the variant `Value::List`.
    ///
    /// This is not suitable for converting any other iterables.
    /// See `FromIterator<T> for Value` for recursive conversion from any `Iterator`.
    Vec<Value<'borrow>> => List
);

impl From<String> for Value<'_> {
    fn from(other: String) -> Self {
        Self::String(Cow::Owned(other))
    }
}

impl<'borrow> From<&'borrow String> for Value<'borrow> {
    fn from(other: &'borrow String) -> Self {
        Self::String(Cow::Borrowed(other))
    }
}

impl<'borrow> From<&'borrow str> for Value<'borrow> {
    fn from(other: &'borrow str) -> Self {
        Self::String(Cow::Borrowed(other))
    }
}

impl<T> FromIterator<T> for Value<'_>
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
    into_type: &'static str,
}

impl std::fmt::Display for IntoInnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cannot not convert variant `{}` to a `{}`",
            self.variant, self.into_type
        )
    }
}

impl Error for IntoInnerError {}

pub trait IntoInner<T>: crate::Sealed {
    fn into_inner(self) -> Result<T, IntoInnerError>;
}

macro_rules! impl_into_inner {
    ($($variant:ident => $inner:ty),+) => {
        $(impl_into_inner!($variant, $inner);)*
    };
    ($variant:ident, $inner:ty) => {
        impl<'borrow> IntoInner<$inner> for Value<'borrow> {
            fn into_inner(self) -> Result<$inner, IntoInnerError> {
                match self {
                    Self::$variant(value) => Ok(value),
                    _ => Err(IntoInnerError {
                        variant: self.kind(),
                        into_type: stringify!($inner),
                    }),
                }
            }
        }

        impl<'borrow> IntoInner<Option<$inner>> for Value<'borrow> {
            fn into_inner(self) -> Result<Option<$inner>, IntoInnerError> {
                match self {
                    Self::$variant(value) => Ok(Some(value)),
                    Self::Null => Ok(None),
                    _ => Err(IntoInnerError {
                        variant: self.kind(),
                        into_type: const_format::formatcp!("Option<{}>", stringify!($inner)),
                    }),
                }
            }
        }
    };
}

impl_into_inner!(
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
    F32 => f32,
    F64 => f64,
    Bool => bool,
    String => Cow<'borrow, str>,
    List => Vec<Value<'borrow>>
);

impl<'borrow> IntoInner<&'borrow str> for Value<'borrow> {
    fn into_inner(self) -> Result<&'borrow str, IntoInnerError> {
        match self {
            Self::String(Cow::Borrowed(inner)) => Ok(inner),
            _ => Err(IntoInnerError {
                variant: self.kind(),
                into_type: "&str",
            }),
        }
    }
}

impl IntoInner<String> for Value<'_> {
    fn into_inner(self) -> Result<String, IntoInnerError> {
        match self {
            Self::String(Cow::Owned(inner)) => Ok(inner),
            _ => Err(IntoInnerError {
                variant: self.kind(),
                into_type: "String",
            }),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{IntoInner, Value};

    #[test]
    fn test_roundtrip_borrowed_str() {
        let expect = "foo";
        let value = Value::from(expect);
        let inner: &str = value.into_inner().unwrap();

        assert_eq!(expect, inner);
    }

    #[test]
    fn test_roundtrip_owned_str() {
        let expect = "foo".to_owned();
        let value = Value::from(expect.clone());
        let inner: String = value.into_inner().unwrap();

        assert_eq!(expect, inner);
    }
}
