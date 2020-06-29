//! # Debug
//!
//! Module containing utilities used for debugging.

#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bae_debug/0.14.0")]

/// Equivalency check of two floating point values.
///
/// # Parameters
///
/// * `a`, `b` - Floating point values that will be compared
/// * `e` - The epsilon value with which the absolute difference between
///   parameters `a` and `b` must be to be deemed equivalent. For built-in
///   floating point types you are welcome to use [`f32::EPSILON`] or
///   [`f64::EPSILON`].
/// * `abs` - Function that determines the absolute value of the given floating
///   point value. For built-in floating point types you are welcome to use
///   [`f32::abs`] or [`f64::abs`].
///
/// [`f32::EPSILON`]: https://doc.rust-lang.org/std/primitive.f32.html#associatedconstant.EPSILON
/// [`f64::EPSILON`]: https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.EPSILON
/// [`f32::abs`]: https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.abs
/// [`f64::abs`]: https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.abs
pub fn float_equal<T, F>(a: T, b: T, e: T, abs: F) -> bool
where
    T: std::ops::Sub<T, Output = T> + std::cmp::PartialOrd,
    F: FnOnce(T) -> T,
{
    abs(a - b) < e
}
