#![warn(missing_docs)]

//! Provides bindings for the `dylib_provider` crate, which provides operations on the
//! `Rational` type.

/// A `Rational` type, defined as a numerator divided by a denominator.
pub use rational_impl_types::Rational;

#[link(name = "rational_impl")]
extern "C" {
    fn rational_impl_get_version(major: *mut i32, minor: *mut i32, patch: *mut i32);
    fn rational_impl_add_rationals(_: *const Rational, _: *const Rational) -> Rational;
    fn rational_impl_is_dynamically_linked() -> i32;
    fn rational_impl_normalize_rational(_: *mut Rational);
}

/// Returns the version of the `rational_impl` library linked.
#[inline]
pub fn version() -> (i32, i32, i32) {
    let (mut maj, mut min, mut patch) = Default::default();
    unsafe { rational_impl_get_version(&mut maj, &mut min, &mut patch) }

    (maj, min, patch)
}

/// Returns `true` if the rational library is dynamically linked, `false` otherwise.
///
/// Dynamic linking is controlled through the `link_dynamic` feature.
#[inline]
pub fn is_dynamically_linked() -> bool {
    unsafe { rational_impl_is_dynamically_linked() != 0 }
}

/// Adds the given `Rational`s together, normalizing the result.
#[inline]
pub fn add(x: &Rational, y: &Rational) -> Rational {
    unsafe { rational_impl_add_rationals(x, y) }
}

/// Normalize the given `Rational` in place.
#[inline]
pub fn normalize(x: &mut Rational) {
    unsafe {
        rational_impl_normalize_rational(x);
    }
}
