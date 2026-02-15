/// Uiua's floating-point number type.
#[cfg(feature = "f32_num")]
pub type Num = f32;
/// Uiua's floating-point number type.
#[cfg(not(feature = "f32_num"))]
pub type Num = f64;

/// Numeric constants that match [`Num`].
#[cfg(feature = "f32_num")]
pub mod consts {
    pub use core::f32::consts::*;
}

/// Numeric constants that match [`Num`].
#[cfg(not(feature = "f32_num"))]
pub mod consts {
    pub use core::f64::consts::*;
}

/// Cast a high-precision literal or intermediate to [`Num`].
#[inline(always)]
pub fn as_num(value: f64) -> Num {
    value as Num
}

/// Build a [`Num`] from raw bits.
#[cfg(feature = "f32_num")]
#[inline(always)]
pub const fn num_from_bits(bits: u32) -> Num {
    Num::from_bits(bits)
}

/// Build a [`Num`] from raw bits.
#[cfg(not(feature = "f32_num"))]
#[inline(always)]
pub const fn num_from_bits(bits: u64) -> Num {
    Num::from_bits(bits)
}
