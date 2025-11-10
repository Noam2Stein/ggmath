use crate::{
    ScalarNegOne, ScalarOne, ScalarZero, SupportedVecLen, VecLen, Vector, vector::specialize,
};

impl<const N: usize> Vector<N, f>
where
    VecLen<N>: SupportedVecLen,
{
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn floor(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_floor(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn ceil(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_ceil(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn round(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_round(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn trunc(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_trunc(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn fract(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_fract(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_mul_add(self, a, b))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn div_euclid(self, rhs: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_div_euclid(self, rhs))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_rem_euclid(self, rhs))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn sqrt(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_sqrt(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn sin(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_sin(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn cos(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_cos(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn tan(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_tan(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn asin(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_asin(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn acos(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_acos(self))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn atan(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_atan(self))
    }

    #[inline(always)]
    pub fn recip(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_recip(self))
    }

    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_max(self, other))
    }

    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_min(self, other))
    }

    #[inline(always)]
    pub fn midpoint(self, other: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_midpoint(self, other))
    }

    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_clamp(self, min, max))
    }

    #[inline(always)]
    pub fn abs(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_abs(self))
    }

    #[inline(always)]
    pub fn signum(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_signum(self))
    }

    #[inline(always)]
    pub fn copysign(self, sign: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_copysign(self, sign))
    }

    #[inline(always)]
    pub fn element_sum(self) -> f {
        specialize!(<f as FloatBackend<N>>::vec_element_sum(self))
    }

    #[inline(always)]
    pub fn element_product(self) -> f {
        specialize!(<f as FloatBackend<N>>::vec_element_product(self))
    }

    #[inline(always)]
    pub fn max_element(self) -> f {
        specialize!(<f as FloatBackend<N>>::vec_max_element(self))
    }

    #[inline(always)]
    pub fn min_element(self) -> f {
        specialize!(<f as FloatBackend<N>>::vec_min_element(self))
    }

    #[inline(always)]
    pub fn dot(self, other: Self) -> f {
        (self * other).element_sum()
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn length(self) -> f {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    pub fn length_squared(self) -> f {
        self.dot(self)
    }

    #[cfg(feature = "std")]
    #[must_use]
    #[inline(always)]
    pub fn normalize(self) -> Self {
        let result = self / self.length();

        debug_assert!(result.iter().all(<f>::is_finite));

        result
    }

    #[cfg(feature = "std")]
    #[must_use]
    #[inline(always)]
    pub fn try_normalize(self) -> Option<Self> {
        let rcp = 1.0 / self.length();
        if rcp.is_finite() && rcp > 0.0 {
            Some(self * rcp)
        } else {
            None
        }
    }

    #[cfg(feature = "std")]
    #[must_use]
    #[inline(always)]
    pub fn normalize_or(self, fallback: Self) -> Self {
        self.try_normalize().unwrap_or(fallback)
    }
}

impl ScalarZero for f {
    const ZERO: Self = 0.0;
}

impl ScalarOne for f {
    const ONE: Self = 1.0;
}

impl ScalarNegOne for f {
    const NEG_ONE: Self = -1.0;
}

pub(in crate::vector) trait FloatBackend<const N: usize>
where
    VecLen<N>: SupportedVecLen,
{
    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_floor(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::floor)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_ceil(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::ceil)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_round(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::round)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_trunc(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::trunc)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_fract(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::fract)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_mul_add(vec: Vector<N, f>, a: Vector<N, f>, b: Vector<N, f>) -> Vector<N, f> {
        Vector::from_fn(|i| vec[i].mul_add(a[i], b[i]))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_div_euclid(vec: Vector<N, f>, rhs: Vector<N, f>) -> Vector<N, f> {
        Vector::from_fn(|i| vec[i].div_euclid(rhs[i]))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_rem_euclid(vec: Vector<N, f>, rhs: Vector<N, f>) -> Vector<N, f> {
        Vector::from_fn(|i| vec[i].rem_euclid(rhs[i]))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_sqrt(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::sqrt)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_sin(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::sin)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_cos(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::cos)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_tan(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::tan)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_asin(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::asin)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_acos(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::acos)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_atan(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::atan)
    }

    #[inline(always)]
    fn vec_recip(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::recip)
    }

    #[inline(always)]
    fn vec_max(vec: Vector<N, f>, other: Vector<N, f>) -> Vector<N, f> {
        debug_assert!(!vec.iter().any(f::is_nan));
        debug_assert!(!other.iter().any(f::is_nan));

        Vector::from_fn(|i| vec[i].max(other[i]))
    }

    #[inline(always)]
    fn vec_min(vec: Vector<N, f>, other: Vector<N, f>) -> Vector<N, f> {
        debug_assert!(!vec.iter().any(f::is_nan));
        debug_assert!(!other.iter().any(f::is_nan));

        Vector::from_fn(|i| vec[i].min(other[i]))
    }

    #[inline(always)]
    fn vec_midpoint(vec: Vector<N, f>, other: Vector<N, f>) -> Vector<N, f> {
        Vector::from_fn(|i| vec[i].midpoint(other[i]))
    }

    #[inline(always)]
    fn vec_clamp(vec: Vector<N, f>, min: Vector<N, f>, max: Vector<N, f>) -> Vector<N, f> {
        debug_assert!(!vec.iter().any(f::is_nan));
        debug_assert!(!min.iter().any(f::is_nan));
        debug_assert!(!max.iter().any(f::is_nan));
        debug_assert!(min.iter().zip(max).all(|(min, max)| min <= max));

        Vector::from_fn(|i| vec[i].max(min[i]).min(max[i]))
    }

    #[inline(always)]
    fn vec_abs(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::abs)
    }

    #[inline(always)]
    fn vec_signum(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::signum)
    }

    #[inline(always)]
    fn vec_copysign(vec: Vector<N, f>, sign: Vector<N, f>) -> Vector<N, f> {
        Vector::from_fn(|i| vec[i].copysign(sign[i]))
    }

    #[inline(always)]
    fn vec_element_sum(vec: Vector<N, f>) -> f {
        vec.to_array().iter().sum()
    }

    #[inline(always)]
    fn vec_element_product(vec: Vector<N, f>) -> f {
        vec.to_array().iter().product()
    }

    #[inline(always)]
    fn vec_max_element(vec: Vector<N, f>) -> f {
        debug_assert!(!vec.iter().any(|x| x.is_nan()));

        vec.iter().reduce(|a, b| a.max(b)).unwrap()
    }

    #[inline(always)]
    fn vec_min_element(vec: Vector<N, f>) -> f {
        debug_assert!(!vec.iter().any(|x| x.is_nan()));

        vec.iter().reduce(|a, b| a.min(b)).unwrap()
    }
}
