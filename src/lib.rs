//! Implementation of Nice Numbers for Graph Labels by Paul S. Heckbert from Graphics Gems, Academic
//! Press, 1990.
//!
//! ```
//! # extern crate alloc;
//! # use alloc::vec::Vec;
//! # use nicenum::LooseLabel;
//! assert_eq!(
//!     &[100., 200., 300., 400., 500., 600.],
//!     LooseLabel::new(105., 543., 5).unwrap().into_iter().collect::<Vec<_>>().as_slice()
//! );
//! ```

#![no_std]
#![deny(
    missing_docs,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::missing_errors_doc
)]

#[cfg(feature = "std")]
extern crate std;

use private::Sealed;

mod math;
mod private;

/// Rounding mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum RoundingMode {
    /// Nearest.
    Nearest,
    /// Ceiling.
    Ceil,
}

/// A floating point number.
pub trait Float: Sealed {}

impl Float for f32 {}
impl Float for f64 {}

/// Label the data range from min to max loosely.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct LooseLabel<T> {
    /// Graph range min.
    graphmin: T,
    /// Graph range max.
    graphmax: T,
    /// Tick mark spacing
    d: T,
    /// Desired number of tick marks.
    ntick: usize,
}

/// On failure to construct [`LooseLabel`].
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LooseLabelError(());

impl core::fmt::Display for LooseLabelError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        "failed to construct LooseLabel".fmt(f)
    }
}

impl core::error::Error for LooseLabelError {}

impl<T> LooseLabel<T>
where
    T: Float,
{
    /// Label the data range from min to max loosely.
    pub fn new(min: T, max: T, ntick: usize) -> Result<Self, LooseLabelError> {
        if min >= max || !min.is_finite() || !max.is_finite() || ntick < 2 {
            return Err(LooseLabelError(()));
        }

        let range = nicenum(max - min, RoundingMode::Ceil);
        let d = nicenum(range / T::from_usize(ntick - 1), RoundingMode::Nearest);
        if !d.is_finite() || d <= T::ZERO {
            return Err(LooseLabelError(()));
        }

        let graphmin = (min / d).floor() * d;
        let graphmax = (max / d).ceil() * d;
        if !graphmin.is_finite() || !graphmax.is_finite() {
            return Err(LooseLabelError(()));
        }

        Ok(Self {
            graphmin,
            graphmax,
            d,
            ntick,
        })
    }

    /// Graph range min.
    pub const fn graphmin(&self) -> T {
        self.graphmin
    }

    /// Graph range max.
    pub const fn graphmax(&self) -> T {
        self.graphmax
    }

    /// Tick mark spacing
    pub const fn d(&self) -> T {
        self.d
    }

    /// Desired number of tick marks.
    pub const fn ntick(&self) -> usize {
        self.ntick
    }

    /// Number of fractional digits to show
    pub fn nfrac(&self) -> usize {
        (-(self.d.log10().floor())).to_usize()
    }
}

impl<T> IntoIterator for LooseLabel<T>
where
    T: Float,
{
    type Item = T;

    type IntoIter = LooseLabelIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        LooseLabelIter { i: 0, inner: self }
    }
}

/// [`LooseLabel`] iterator.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct LooseLabelIter<T> {
    i: usize,
    inner: LooseLabel<T>,
}

impl<T> Iterator for LooseLabelIter<T>
where
    T: Float,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.graphmin() + T::from_usize(self.i) * self.inner.d();
        if item >= self.inner.graphmax() + T::HALF * self.inner.d() {
            return None;
        }
        self.i += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = T::to_usize(
            ((self.inner.graphmax() - self.inner.graphmin()) / self.inner.d() + T::HALF).ceil(),
        ) - self.i;
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for LooseLabelIter<T> where T: Float {}

/// Find a "nice" number approximately equal to x.
#[must_use]
pub fn nicenum<T>(x: T, round: RoundingMode) -> T
where
    T: Float,
{
    let exp = x.log10().floor(); // exponent of x
    let f = x / exp.exp10(); // fractional part of x

    // nice, rounded fraction
    let nf = match round {
        RoundingMode::Nearest => {
            if f < T::ONE_AND_A_HALF {
                T::ONE
            } else if f < T::THREE {
                T::TWO
            } else if f < T::SEVEN {
                T::FIVE
            } else {
                T::TEN
            }
        }
        RoundingMode::Ceil => {
            if f <= T::ONE {
                T::ONE
            } else if f <= T::TWO {
                T::TWO
            } else if f <= T::FIVE {
                T::FIVE
            } else {
                T::TEN
            }
        }
    };

    nf * exp.exp10()
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::vec::Vec;

    use super::*;

    #[test]
    fn example() {
        let iter = LooseLabel::<f32>::new(105., 543., 5).unwrap().into_iter();
        assert_eq!(6, iter.len());
        assert_eq!(
            &[100., 200., 300., 400., 500., 600.],
            iter.collect::<Vec<_>>().as_slice()
        );
    }

    #[test]
    fn fract() {
        let iter = LooseLabel::new(2.03, 2.17, 4).unwrap().into_iter();
        assert_eq!(5, iter.len());
        assert_eq!(
            &[2., 2.05, 2.1, 2.15, 2.2],
            iter.collect::<Vec<_>>().as_slice()
        );
    }
}
