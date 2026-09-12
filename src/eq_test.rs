use crate::{Alignment, Dim, Element, FloatExt, Matrix, PrimitiveFloat, TwoThreeOrFour, Vector};

/// Defines an equality test used by assertions in generic functions.
///
/// Some functions are generic over `T`, but need to validate numeric
/// invariants. For example, when debug assertions are enabled,
/// [`Rotor::inverse`] needs to assert that `self` is normalized.
///
/// These functions cannot use ordinary equality. The equality test must be
/// approximate for floats, exact for integers, and disabled for SoA types,
/// where some lanes may intentionally be invalid when using lanewise selection.
///
/// This trait defines the [`eq_test`] operation, which has appropriate behavior
/// for each type, and is used by generic functions. Float implementations use
/// [`abs_diff_eq`] with `max_abs_diff = 2e-4`, integer implementations use
/// exact equality, and [`wide`] implementations always return `true`.
///
/// [`Rotor::inverse`]: crate::Rotor::inverse
/// [`eq_test`]: Self::eq_test
/// [`abs_diff_eq`]: FloatExt::abs_diff_eq
/// [`wide`]: crate#soa
pub trait EqTest {
    /// Returns whether `self` and `other` pass an equality test.
    ///
    /// See [`EqTest`] for more details.
    fn eq_test(&self, other: &Self) -> bool;
}

impl<T: PrimitiveFloat> EqTest for T {
    #[inline]
    fn eq_test(&self, other: &Self) -> bool {
        self.abs_diff_eq(*other, T::as_from(2e-4))
    }
}

macro_rules! integer_impl {
    ($T:ident) => {
        impl EqTest for $T {
            #[inline]
            fn eq_test(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}
integer_impl!(i8);
integer_impl!(i16);
integer_impl!(i32);
integer_impl!(i64);
integer_impl!(i128);
integer_impl!(isize);
integer_impl!(u8);
integer_impl!(u16);
integer_impl!(u32);
integer_impl!(u64);
integer_impl!(u128);
integer_impl!(usize);

impl<const N: usize, T, A: Alignment> EqTest for Vector<N, T, A>
where
    Dim<N>: TwoThreeOrFour,
    T: Element + EqTest,
{
    #[inline]
    fn eq_test(&self, other: &Self) -> bool {
        (0..N).all(|i| self[i].eq_test(&other[i]))
    }
}

impl<const N: usize, T, A: Alignment> EqTest for Matrix<N, T, A>
where
    Dim<N>: TwoThreeOrFour,
    T: Element + EqTest,
{
    #[inline]
    fn eq_test(&self, other: &Self) -> bool {
        (0..N).all(|i| self[i].eq_test(&other[i]))
    }
}

#[cfg(feature = "wide")]
mod wide_impl {
    use crate::EqTest;

    macro_rules! wide_impl {
        ($Wide:ident) => {
            impl EqTest for wide::$Wide {
                #[inline(always)]
                fn eq_test(&self, _other: &Self) -> bool {
                    true
                }
            }
        };
    }
    wide_impl!(f32x4);
    wide_impl!(f32x8);
    wide_impl!(f32x16);
    wide_impl!(f64x2);
    wide_impl!(f64x4);
    wide_impl!(f64x8);
    wide_impl!(i8x16);
    wide_impl!(i8x32);
    wide_impl!(i8x64);
    wide_impl!(i16x8);
    wide_impl!(i16x16);
    wide_impl!(i16x32);
    wide_impl!(i32x4);
    wide_impl!(i32x8);
    wide_impl!(i32x16);
    wide_impl!(i64x2);
    wide_impl!(i64x4);
    wide_impl!(i64x8);
    wide_impl!(u8x16);
    wide_impl!(u8x32);
    wide_impl!(u8x64);
    wide_impl!(u16x8);
    wide_impl!(u16x16);
    wide_impl!(u16x32);
    wide_impl!(u32x4);
    wide_impl!(u32x8);
    wide_impl!(u32x16);
    wide_impl!(u64x2);
    wide_impl!(u64x4);
    wide_impl!(u64x8);
}
