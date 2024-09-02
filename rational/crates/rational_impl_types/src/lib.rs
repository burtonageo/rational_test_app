#![no_std]

use core::fmt;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rational {
    pub numerator: u64,
    pub denominator: u64,
}

impl Rational {
    #[inline(always)]
    pub const fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl Default for Rational {
    #[inline(always)]
    fn default() -> Self {
        Self::new(0, 1)
    }
}

impl fmt::Debug for Rational {
    #[inline]
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmtr, "{}/{}", self.numerator, self.denominator)
    }
}
