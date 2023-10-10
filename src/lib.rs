#![warn(clippy::pedantic)]
#![warn(unused_crate_dependencies)]
// FIXME change before release
#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]
// Stylistic problems
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![deny(elided_lifetimes_in_paths)]
#![deny(non_ascii_idents)]
// Lifetimes
#![warn(explicit_outlives_requirements)]
#![warn(single_use_lifetimes)]
#![warn(unused_lifetimes)]
// Macros
#![warn(meta_variable_misuse)]
#![warn(unused_macro_rules)]
// Missing implementations
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
// Unreliable or unintended behavior
#![warn(unreachable_pub)]
#![warn(unused_tuple_struct_fields)]
#![warn(variant_size_differences)]
#![deny(let_underscore_drop)]
#![deny(pointer_structural_match)]
#![deny(unsafe_code)]

pub mod traits;
pub mod value;

pub(crate) use private::Sealed;
pub(crate) mod private {
    pub trait Sealed {}
}

pub use traits::*;
pub use value::*;
