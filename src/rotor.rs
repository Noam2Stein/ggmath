use core::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, One, Scalar, Unaligned, Vector, Zero,
    length::Three,
    utils::{transmute_mut, transmute_ref},
};

// These submodules have empty lines between them so that rustfmt does not
// incorrectly reorder them. The order is important since it impacts the order
// of `impl` blocks in rustdoc's output.
//
// The contents of the `generic` submodule *would* be simply put in this root
// module, but due to a rustdoc bug, that would cause functionality generic over
// `T` to be shown after all submodule functionality.

mod generic;

mod float;

#[cfg(feature = "wide")]
mod wide;

#[cfg(feature = "wide")]
mod wide_float;

/// TODO
#[repr(transparent)]
#[expect(private_bounds)]
pub struct Rotor<const N: usize, T, A: Alignment>(
    /// Currently, since only 3D rotors are supported, the internal field is
    /// always a vector4.
    ///
    /// If additional dimensions are ever supported, this will need to be
    /// changed.
    pub(crate) Vector<4, T, A>,
)
where
    Length<N>: Three,
    T: Scalar;

/// TODO
pub type Rotor3<T> = Rotor<3, T, Unaligned>;

/// TODO
pub type Rotor3A<T> = Rotor<3, T, Aligned>;

impl<const N: usize, T, A: Alignment> Clone for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar,
{
}

#[doc(hidden)]
#[repr(C)]
pub struct Rotor3Fields<T> {
    /// TODO
    pub yz: T,
    /// TODO
    pub zx: T,
    /// TODO
    pub xy: T,
    /// TODO
    pub s: T,
}

impl<T, A: Alignment> Deref for Rotor<3, T, A>
where
    T: Scalar,
{
    type Target = Rotor3Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Rotor<3, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Rotor3Fields<T>`.
        unsafe { transmute_ref::<Rotor<3, T, A>, Rotor3Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Rotor<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Rotor<3, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Rotor3Fields<T>`.
        unsafe { transmute_mut::<Rotor<3, T, A>, Rotor3Fields<T>>(self) }
    }
}

impl<const N: usize, T, A: Alignment> Debug for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Debug,
{
    #[inline]
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl<const N: usize, T, A: Alignment> Eq for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}

impl<const N: usize, T, A: Alignment> Default for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Zero + One,
{
    /// Returns [`IDENTITY`].
    ///
    /// [`IDENTITY`]: Self::IDENTITY
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

macro_rules! impl_neg {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Neg for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                todo!()
            }
        }

        impl<const N: usize, T, A: Alignment> Neg for &Rotor<N, T, A>
        where
        Length<N>: Three,
        T: Scalar + Neg<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                -*self
            }
        }
    };
}
impl_neg!(
    /// TODO
);

macro_rules! impl_add {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Add for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, _rhs: Self) -> Self::Output {
                todo!()
            }
        }

        impl<const N: usize, T, A: Alignment> Add<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: &Self) -> Self::Output {
                self + *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Add<Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Add<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self + rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Add for &Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Add<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                *self + *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> AddAssign for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> AddAssign<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: &Self) {
                *self = *self + rhs;
            }
        }
    };
}
impl_add!(
    /// TODO
);

macro_rules! impl_sub {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Sub for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, _rhs: Self) -> Self::Output {
                todo!()
            }
        }

        impl<const N: usize, T, A: Alignment> Sub<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: &Self) -> Self::Output {
                self - *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Sub<Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self - rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Sub for &Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                *self - *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> SubAssign for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> SubAssign<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: &Self) {
                *self = *self - rhs;
            }
        }
    };
}
impl_sub!(
    /// TODO
);

macro_rules! impl_mul_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<T> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, _rhs: T) -> Self::Output {
                todo!()
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<T> for &Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                *self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for &Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                *self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<T> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: T) {
                *self = *self * rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&T> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &T) {
                *self = *self * *rhs;
            }
        }
    };
}
impl_mul_scalar!(
    /// TODO
);

macro_rules! impl_vector_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, _rhs: Rotor<N, T, A>) -> Self::Output {
                todo!()
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Rotor<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                *self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Rotor<N, T, A>) {
                *self = *self * rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Rotor<N, T, A>) {
                *self = *self * *rhs;
            }
        }
    };
}
impl_vector_mul!(
    /// TODO
);

macro_rules! impl_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, _rhs: Self) -> Self::Output {
                todo!()
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                *self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Rotor<N, T, A>) {
                *self = *self * *rhs;
            }
        }
    };
}
impl_mul!(
    /// TODO
);

macro_rules! impl_div_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Div<T> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, _rhs: T) -> Self::Output {
                todo!()
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                self / *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Div<T> for &Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Div<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                *self / rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for &Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Div<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                *self / *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> DivAssign<T> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: T) {
                *self = *self / rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> DivAssign<&T> for Rotor<N, T, A>
        where
            Length<N>: Three,
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: &T) {
                *self = *self / *rhs;
            }
        }
    };
}
impl_div_scalar!(
    /// TODO
);

// SAFETY: Rotors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Send` the list also is, and the padding is `Send` too.
unsafe impl<const N: usize, T, A: Alignment> Send for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Send,
{
}

// SAFETY: Rotors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Sync` the list also is, and the padding is `Sync` too.
unsafe impl<const N: usize, T, A: Alignment> Sync for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Sync,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Unpin,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + UnwindSafe,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + RefUnwindSafe,
{
}

#[cfg(test)]
mod tests {
    extern crate std;

    #[test]
    fn test_identity() {
        todo!()
    }

    #[test]
    fn test_conjugate() {
        todo!()
    }

    #[test]
    fn test_dot() {
        todo!()
    }

    #[test]
    fn test_length_squared() {
        todo!()
    }

    #[test]
    fn test_to_alignment() {
        todo!()
    }

    #[test]
    fn test_align() {
        todo!()
    }

    #[test]
    fn test_unalign() {
        todo!()
    }

    #[test]
    fn test_deref() {
        todo!()
    }

    #[test]
    fn test_deref_mut() {
        todo!()
    }

    #[test]
    fn test_debug() {
        todo!()
    }

    #[test]
    fn test_eq() {
        todo!()
    }

    #[test]
    fn test_ne() {
        todo!()
    }

    #[test]
    fn test_default() {
        todo!()
    }

    #[test]
    fn test_neg() {
        todo!()
    }

    #[test]
    fn test_add() {
        todo!()
    }

    #[test]
    fn test_sub() {
        todo!()
    }

    #[test]
    fn test_mul_scalar() {
        todo!()
    }

    #[test]
    fn test_vector_mul() {
        todo!()
    }

    #[test]
    fn test_mul() {
        todo!()
    }

    #[test]
    fn test_div_scalar() {
        todo!()
    }
}
