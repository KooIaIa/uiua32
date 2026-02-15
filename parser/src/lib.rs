pub mod ast;
mod complex;
mod defs;
mod error;
mod inputs;
mod lex;
pub mod parse;
mod primitive;
mod signature;
mod split;
mod subscript;

pub use {
    complex::*, defs::*, error::*, inputs::*, lex::*, parse::parse, primitive::*, signature::*,
    split::*, subscript::*,
};

/// A Uiua identifier
pub type Ident = ecow::EcoString;

/// The floating-point scalar used by [`Complex`].
#[cfg(feature = "f32_num")]
pub type ComplexNum = f32;
/// The floating-point scalar used by [`Complex`].
#[cfg(not(feature = "f32_num"))]
pub type ComplexNum = f64;

/// Numeric constants that match [`ComplexNum`].
#[cfg(feature = "f32_num")]
pub mod num_consts {
    pub use core::f32::consts::*;
}
/// Numeric constants that match [`ComplexNum`].
#[cfg(not(feature = "f32_num"))]
pub mod num_consts {
    pub use core::f64::consts::*;
}

/// A NaN value that always compares as equal
#[cfg(feature = "f32_num")]
pub const WILDCARD_NAN: ComplexNum = ComplexNum::from_bits(0x7fc0_0003);
/// A NaN value that always compares as equal
#[cfg(not(feature = "f32_num"))]
pub const WILDCARD_NAN: ComplexNum = ComplexNum::from_bits(0x7ff8_0000_0000_0000u64 | 0x3);
/// A character value used as a wildcard that will equal any character
pub const WILDCARD_CHAR: char = '\u{100000}';
