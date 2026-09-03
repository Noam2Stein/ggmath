use core::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    Aligned, Alignment, One, Scalar, Unaligned, Vector, Zero,
    utils::{transmute_mut, transmute_ref},
};

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
    /// The sine of the angle rotating `+X` to `+Y`.
    pub sin: T,
    /// The cosine of the angle rotating `+X` to `+Y`.
    pub cos: T,
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
            .field("sin", &self.sin)
            .field("cos", &self.cos)
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
        todo!()
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
            fn mul(self, _rhs: Self) -> Self::Output {
                todo!()
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
    fn test_vector_mul() {
        todo!()
    }

    #[test]
    fn test_mul() {
        todo!()
    }
}
