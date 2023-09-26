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

impl Value {
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::U8(_) => "U8",
            Self::U16(_) => "U16",
            Self::U32(_) => "U32",
            Self::U64(_) => "U64",
            Self::Uint(_) => "Uint",
            Self::I8(_) => "I8",
            Self::I16(_) => "I16",
            Self::I32(_) => "I32",
            Self::I64(_) => "I64",
            Self::Int(_) => "Int",
            Self::F32(_) => "F32",
            Self::F64(_) => "F64",
            Self::Bool(_) => "Bool",
            Self::String(_) => "String",
            Self::List(_) => "List",
            Self::Null => "Null",
        }
    }
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
    f32 => F32,
    f64 => F64,
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
                        variant: value.kind(),
                        into_type: stringify!($inner),
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
    F32 => f32,
    F64 => f64,
    Bool => bool,
    String => String,
    List => Vec<Value>
);
