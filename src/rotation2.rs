use core::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    Aligned, Alignment, One, Scalar, Unaligned, Vector, Zero,
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
pub struct Rotation2<T, A: Alignment>(pub(crate) Vector<2, T, A>)
where
    T: Scalar;

/// TODO
pub type Rot2<T> = Rotation2<T, Unaligned>;

/// TODO
pub type Rot2A<T> = Rotation2<T, Aligned>;

impl<T, A: Alignment> Clone for Rotation2<T, A>
where
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, A: Alignment> Copy for Rotation2<T, A> where T: Scalar {}

#[doc(hidden)]
#[repr(C)]
pub struct Rot2Fields<T> {
    /// The cosine of the angle.
    pub cos: T,
    /// The sine of the angle rotating `+X` to `+Y`.
    pub sin: T,
}

impl<T, A: Alignment> Deref for Rotation2<T, A>
where
    T: Scalar,
{
    type Target = Rot2Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Rotation2<T, A>` is guaranteed to begin with 2 consecutive
        // values of `T`, and so begin with `Rot2Fields<T>`.
        unsafe { transmute_ref::<Rotation2<T, A>, Rot2Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Rotation2<T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Rotation2<T, A>` is guaranteed to begin with 2 consecutive
        // values of `T`, and so begin with `Rot2Fields<T>`.
        unsafe { transmute_mut::<Rotation2<T, A>, Rot2Fields<T>>(self) }
    }
}

impl<T, A: Alignment> Debug for Rotation2<T, A>
where
    T: Scalar + Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rot2")
            .field("cos", &self.cos)
            .field("sin", &self.sin)
            .finish()
    }
}

impl<T, A: Alignment> PartialEq for Rotation2<T, A>
where
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl<T, A: Alignment> Eq for Rotation2<T, A> where T: Scalar + Eq {}

impl<T, A: Alignment> Hash for Rotation2<T, A>
where
    T: Scalar + Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T, A: Alignment> Default for Rotation2<T, A>
where
    T: Scalar + Zero + One,
{
    /// TODO
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

macro_rules! impl_neg {
    ($(#[$doc:meta])*) => {
        impl<T, A: Alignment> Neg for Rotation2<T, A>
        where
            T: Scalar + Neg<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                Self(-self.0)
            }
        }

        impl<T, A: Alignment> Neg for &Rotation2<T, A>
        where
            T: Scalar + Neg<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                Rotation2(-self.0)
            }
        }
    };
}
impl_neg!(
    /// TODO
);

macro_rules! impl_add {
    ($(#[$doc:meta])*) => {
        impl<T, A: Alignment> Add for Rotation2<T, A>
        where
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl<T, A: Alignment> Add<&Rotation2<T, A>> for Rotation2<T, A>
        where
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: &Rotation2<T, A>) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl<T, A: Alignment> Add<Rotation2<T, A>> for &Rotation2<T, A>
        where
            T: Scalar + Add<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Rotation2<T, A>) -> Self::Output {
                Rotation2(self.0 + rhs.0)
            }
        }

        impl<T, A: Alignment> Add<&Rotation2<T, A>> for &Rotation2<T, A>
        where
            T: Scalar + Add<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: &Rotation2<T, A>) -> Self::Output {
                Rotation2(self.0 + rhs.0)
            }
        }

        impl<T, A: Alignment> AddAssign for Rotation2<T, A>
        where
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: Self) {
                *self = Self(self.0 + rhs.0);
            }
        }

        impl<T, A: Alignment> AddAssign<&Rotation2<T, A>> for Rotation2<T, A>
        where
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: &Rotation2<T, A>) {
                *self = Self(self.0 + rhs.0);
            }
        }
    };
}
impl_add!(
    /// TODO
);

macro_rules! impl_sub {
    ($(#[$doc:meta])*) => {
        impl<T, A: Alignment> Sub for Rotation2<T, A>
        where
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl<T, A: Alignment> Sub<&Rotation2<T, A>> for Rotation2<T, A>
        where
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: &Rotation2<T, A>) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl<T, A: Alignment> Sub<Rotation2<T, A>> for &Rotation2<T, A>
        where
            T: Scalar + Sub<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Rotation2<T, A>) -> Self::Output {
                Rotation2(self.0 - rhs.0)
            }
        }

        impl<T, A: Alignment> Sub<&Rotation2<T, A>> for &Rotation2<T, A>
        where
            T: Scalar + Sub<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: &Rotation2<T, A>) -> Self::Output {
                Rotation2(self.0 - rhs.0)
            }
        }

        impl<T, A: Alignment> SubAssign for Rotation2<T, A>
        where
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: Self) {
                *self = Self(self.0 - rhs.0);
            }
        }

        impl<T, A: Alignment> SubAssign<&Rotation2<T, A>> for Rotation2<T, A>
        where
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: &Rotation2<T, A>) {
                *self = Self(self.0 - rhs.0);
            }
        }
    };
}
impl_sub!(
    /// TODO
);

macro_rules! impl_mul_scalar {
    ($(#[$doc:meta])*) => {
        impl<T, A: Alignment> Mul<T> for Rotation2<T, A>
        where
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl<T, A: Alignment> Mul<&T> for Rotation2<T, A>
        where
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl<T, A: Alignment> Mul<T> for &Rotation2<T, A>
        where
            T: Scalar + Mul<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                Rotation2(self.0 * rhs)
            }
        }

        impl<T, A: Alignment> Mul<&T> for &Rotation2<T, A>
        where
            T: Scalar + Mul<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                Rotation2(self.0 * rhs)
            }
        }

        impl<T, A: Alignment> MulAssign<T> for Rotation2<T, A>
        where
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: T) {
                *self = Self(self.0 * rhs);
            }
        }

        impl<T, A: Alignment> MulAssign<&T> for Rotation2<T, A>
        where
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &T) {
                *self = Self(self.0 * rhs);
            }
        }
    };
}
impl_mul_scalar!(
    /// TODO
);

macro_rules! impl_vector_mul {
    ($(#[$doc:meta])*) => {
        impl<T, A: Alignment> Mul<Rotation2<T, A>> for Vector<2, T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotation2<T, A>) -> Self::Output {
                Self::new(
                    self.x * rhs.cos - self.y * rhs.sin,
                    self.y * rhs.cos + self.x * rhs.sin,
                )
            }
        }

        impl<T, A: Alignment> Mul<&Rotation2<T, A>> for Vector<2, T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotation2<T, A>) -> Self::Output {
                self * *rhs
            }
        }

        impl<T, A: Alignment> Mul<Rotation2<T, A>> for &Vector<2, T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<2, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotation2<T, A>) -> Self::Output {
                *self * rhs
            }
        }

        impl<T, A: Alignment> Mul<&Rotation2<T, A>> for &Vector<2, T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<2, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotation2<T, A>) -> Self::Output {
                *self * *rhs
            }
        }

        impl<T, A: Alignment> MulAssign<Rotation2<T, A>> for Vector<2, T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Rotation2<T, A>) {
                *self = *self * rhs;
            }
        }

        impl<T, A: Alignment> MulAssign<&Rotation2<T, A>> for Vector<2, T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Rotation2<T, A>) {
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
        impl<T, A: Alignment> Mul for Rotation2<T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Self) -> Self::Output {
                Self::from_cos_sin(
                    self.cos * rhs.cos - self.sin * rhs.sin,
                    self.sin * rhs.cos + self.cos * rhs.sin,
                )
            }
        }

        impl<T, A: Alignment> Mul<&Rotation2<T, A>> for Rotation2<T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotation2<T, A>) -> Self::Output {
                self * *rhs
            }
        }

        impl<T, A: Alignment> Mul<Rotation2<T, A>> for &Rotation2<T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotation2<T, A>) -> Self::Output {
                *self * rhs
            }
        }

        impl<T, A: Alignment> Mul<&Rotation2<T, A>> for &Rotation2<T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotation2<T, A>) -> Self::Output {
                *self * *rhs
            }
        }

        impl<T, A: Alignment> MulAssign for Rotation2<T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl<T, A: Alignment> MulAssign<&Rotation2<T, A>> for Rotation2<T, A>
        where
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Rotation2<T, A>) {
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
        impl<T, A: Alignment> Div<T> for Rotation2<T, A>
        where
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl<T, A: Alignment> Div<&T> for Rotation2<T, A>
        where
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl<T, A: Alignment> Div<T> for &Rotation2<T, A>
        where
            T: Scalar + Div<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                Rotation2(self.0 / rhs)
            }
        }

        impl<T, A: Alignment> Div<&T> for &Rotation2<T, A>
        where
            T: Scalar + Div<Output = T>,
        {
            type Output = Rotation2<T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                Rotation2(self.0 / rhs)
            }
        }

        impl<T, A: Alignment> DivAssign<T> for Rotation2<T, A>
        where
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: T) {
                *self = Self(self.0 / rhs);
            }
        }

        impl<T, A: Alignment> DivAssign<&T> for Rotation2<T, A>
        where
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: &T) {
                *self = Self(self.0 / rhs);
            }
        }
    };
}
impl_div_scalar!(
    /// TODO
);

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Rotation2, Vector,
        test_utils::{assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_deref() {
        for_types!(|T: PrimitiveNumber, A| {
            let cos = T::as_from(7);
            let sin = T::as_from(5);
            let rotation = Rotation2::<T, A>::from_cos_sin(cos, sin);

            assert_eq!(rotation.cos, cos);
            assert_eq!(rotation.sin, sin);
        });
    }

    #[test]
    fn test_deref_mut() {
        for_types!(|T: PrimitiveNumber, A| {
            let mut cos = T::as_from(7);
            let mut sin = T::as_from(5);
            let mut rotation = Rotation2::<T, A>::from_cos_sin(cos, sin);

            assert_eq!(&mut rotation.cos, &mut cos);
            assert_eq!(&mut rotation.sin, &mut sin);
        });
    }

    #[test]
    fn test_debug() {
        for_types!(|T: PrimitiveNumber, A| {
            for rotation in random_iter::<Rotation2<T, A>>() {
                assert_eq!(
                    format!("{rotation:?}"),
                    format!(
                        "Rot2 {{ cos: {:?}, sin: {:?} }}",
                        rotation.cos, rotation.sin
                    )
                );
            }
        });
    }

    #[test]
    fn test_vector_mul() {
        for_types!(|T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<2, T, A>>().filter(|v| v.length() <= 1e5) {
                assert_test_eq!(vector * Rotation2::IDENTITY, vector, 0.0 = -0.0);
            }
            for (vector, angle) in random_iter::<(Vector<2, T, A>, T)>() {
                assert_test_eq!(vector * Rotation2::from_angle(angle), vector.rotate(angle));
            }
        });
    }

    #[test]
    fn test_mul() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, [a, b]) in random_iter::<(Vector<2, T, A>, [Rotation2<T, A>; 2])>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotation2::IDENTITY).normalize());

                assert_test_eq!(
                    vector * (a * b),
                    vector * a * b,
                    abs <= vector.length() * 1e-5,
                    0.0 = -0.0,
                    INFINITY = NAN
                );
            }
        });
    }
}
