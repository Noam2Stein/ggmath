use crate::{Alignment, Length, Scalar, SupportedLength, Vector, utils::specialize};

impl<const N: usize, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// Returns the maximum between the components of `self` and `other`.
    #[inline]
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_max(self, other))
    }

    /// Returns the minimum between the components of `self` and `other`.
    #[inline]
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_min(self, other))
    }

    /// Clamps the components of `self` between the components of `min` and
    /// `max`.
    #[inline]
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        assert!((0..N).all(|i| min[i] <= max[i]), "min <= max");

        self.max(min).min(max)
    }

    /// Returns the absolute values of the components of `self`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_abs(self))
    }

    /// Returns the signum of the components of `self`.
    ///
    /// For each component:
    ///
    /// - `0` if the component is zero
    /// - `1` if the component is positive
    /// - `-1` if the component is negative
    #[inline]
    #[must_use]
    pub fn signum(self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_signum(self))
    }

    /// Computes `self + rhs`, returning `None` if overflow occured.
    #[inline]
    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        specialize!(<T as IntBackend<N, A>>::vec_checked_add(self, rhs))
    }

    /// Computes `self - rhs`, returning `None` if overflow occured.
    #[inline]
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        specialize!(<T as IntBackend<N, A>>::vec_checked_sub(self, rhs))
    }

    /// Computes `self * rhs`, returning `None` if overflow occured.
    #[inline]
    #[must_use]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        specialize!(<T as IntBackend<N, A>>::vec_checked_mul(self, rhs))
    }

    /// Computes `self / rhs`, returning `None` if division by zero or overflow
    /// occured.
    #[inline]
    #[must_use]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        specialize!(<T as IntBackend<N, A>>::vec_checked_div(self, rhs))
    }

    /// Computes `self % rhs`, returning `None` if division by zero or overflow
    /// occurred.
    #[inline]
    #[must_use]
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        specialize!(<T as IntBackend<N, A>>::vec_checked_rem(self, rhs))
    }

    /// Computes `self + rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[inline]
    #[must_use]
    pub fn saturating_add(self, rhs: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_saturating_add(self, rhs))
    }

    /// Computes `self - rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[inline]
    #[must_use]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_saturating_sub(self, rhs))
    }

    /// Computes `self * rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    #[inline]
    #[must_use]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_saturating_mul(self, rhs))
    }

    /// Computes `self / rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline]
    #[must_use]
    pub fn saturating_div(self, rhs: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_saturating_div(self, rhs))
    }

    /// Computes `self + rhs`, wrapping around at the boundary of the type.
    #[inline]
    #[must_use]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_wrapping_add(self, rhs))
    }

    /// Computes `self - rhs`, wrapping around at the boundary of the type.
    #[inline]
    #[must_use]
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_wrapping_sub(self, rhs))
    }

    /// Computes `self * rhs`, wrapping around at the boundary of the type.
    #[inline]
    #[must_use]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_wrapping_mul(self, rhs))
    }

    /// Computes `self / rhs`, wrapping around at the boundary of the type.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline]
    #[must_use]
    pub fn wrapping_div(self, rhs: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_wrapping_div(self, rhs))
    }

    /// Computes `self % rhs`, wrapping around at the boundary of the type.
    ///
    /// # Panics
    ///
    /// This function will panic if `rhs` is zero.
    #[inline]
    #[must_use]
    pub fn wrapping_rem(self, rhs: Self) -> Self {
        specialize!(<T as IntBackend<N, A>>::vec_wrapping_rem(self, rhs))
    }
}

pub(crate) trait IntBackend<const N: usize, A: Alignment>: Scalar
where
    Length<N>: SupportedLength,
{
    #[inline]
    fn vec_max(vec: Vector<N, T, A>, other: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| if vec[i] > other[i] { vec[i] } else { other[i] })
    }

    #[inline]
    fn vec_min(vec: Vector<N, T, A>, other: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| if vec[i] < other[i] { vec[i] } else { other[i] })
    }

    #[inline]
    fn vec_abs(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::abs)
    }

    #[inline]
    fn vec_signum(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::signum)
    }

    #[inline]
    fn vec_checked_add(mut vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Option<Vector<N, T, A>> {
        for i in 0..N {
            vec[i] = vec[i].checked_add(rhs[i])?;
        }

        Some(vec)
    }

    #[inline]
    fn vec_checked_sub(mut vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Option<Vector<N, T, A>> {
        for i in 0..N {
            vec[i] = vec[i].checked_sub(rhs[i])?;
        }

        Some(vec)
    }

    #[inline]
    fn vec_checked_mul(mut vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Option<Vector<N, T, A>> {
        for i in 0..N {
            vec[i] = vec[i].checked_mul(rhs[i])?;
        }

        Some(vec)
    }

    #[inline]
    fn vec_checked_div(mut vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Option<Vector<N, T, A>> {
        for i in 0..N {
            vec[i] = vec[i].checked_div(rhs[i])?;
        }

        Some(vec)
    }

    #[inline]
    fn vec_checked_rem(mut vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Option<Vector<N, T, A>> {
        for i in 0..N {
            vec[i] = vec[i].checked_rem(rhs[i])?;
        }
        Some(vec)
    }

    #[inline]
    fn vec_saturating_add(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].saturating_add(rhs[i]))
    }

    #[inline]
    fn vec_saturating_sub(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].saturating_sub(rhs[i]))
    }

    #[inline]
    fn vec_saturating_mul(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].saturating_mul(rhs[i]))
    }

    #[inline]
    fn vec_saturating_div(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].saturating_div(rhs[i]))
    }

    #[inline]
    fn vec_wrapping_add(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].wrapping_add(rhs[i]))
    }

    #[inline]
    fn vec_wrapping_sub(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].wrapping_sub(rhs[i]))
    }

    #[inline]
    fn vec_wrapping_mul(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].wrapping_mul(rhs[i]))
    }

    #[inline]
    fn vec_wrapping_div(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].wrapping_div(rhs[i]))
    }

    #[inline]
    fn vec_wrapping_rem(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].wrapping_rem(rhs[i]))
    }
}
