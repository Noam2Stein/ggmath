#[allow(dead_code)]
pub trait LibmFloatExt: Sized {
    fn floor(self) -> Self;

    fn ceil(self) -> Self;

    fn round(self) -> Self;

    fn trunc(self) -> Self;

    fn mul_add(self, a: Self, b: Self) -> Self;

    fn div_euclid(self, rhs: Self) -> Self;

    fn rem_euclid(self, rhs: Self) -> Self;

    fn powf(self, n: Self) -> Self;

    fn sqrt(self) -> Self;

    fn exp(self) -> Self;

    fn exp2(self) -> Self;

    fn ln(self) -> Self;

    fn log2(self) -> Self;

    fn sin(self) -> Self;

    fn cos(self) -> Self;

    fn tan(self) -> Self;

    fn asin(self) -> Self;

    fn acos(self) -> Self;

    fn atan(self) -> Self;

    fn sin_cos(self) -> (Self, Self);
}

impl LibmFloatExt for f32 {
    #[inline]
    fn floor(self) -> Self {
        libm::floorf(self)
    }

    #[inline]
    fn ceil(self) -> Self {
        libm::ceilf(self)
    }

    #[inline]
    fn round(self) -> Self {
        libm::roundf(self)
    }

    #[inline]
    fn trunc(self) -> Self {
        libm::truncf(self)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        libm::fmaf(self, a, b)
    }

    #[inline]
    fn div_euclid(self, rhs: Self) -> Self {
        // Based on https://doc.rust-lang.org/src/std/f32.rs.html#293
        let q = (self / rhs).trunc();
        if self % rhs < 0.0 {
            return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
        }

        q
    }

    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        let r = self % rhs;

        if r < 0.0 { r + rhs.abs() } else { r }
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        libm::powf(self, n)
    }

    #[inline]
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }

    #[inline]
    fn exp(self) -> Self {
        libm::expf(self)
    }

    #[inline]
    fn exp2(self) -> Self {
        libm::exp2f(self)
    }

    #[inline]
    fn ln(self) -> Self {
        libm::logf(self)
    }

    #[inline]
    fn log2(self) -> Self {
        libm::log2f(self)
    }

    #[inline]
    fn sin(self) -> Self {
        libm::sinf(self)
    }

    #[inline]
    fn cos(self) -> Self {
        libm::cosf(self)
    }

    #[inline]
    fn tan(self) -> Self {
        libm::tanf(self)
    }

    #[inline]
    fn asin(self) -> Self {
        libm::asinf(self)
    }

    #[inline]
    fn acos(self) -> Self {
        libm::acosf(self)
    }

    #[inline]
    fn atan(self) -> Self {
        libm::atanf(self)
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        libm::sincosf(self)
    }
}

impl LibmFloatExt for f64 {
    #[inline]
    fn floor(self) -> Self {
        libm::floor(self)
    }

    #[inline]
    fn ceil(self) -> Self {
        libm::ceil(self)
    }

    #[inline]
    fn round(self) -> Self {
        libm::round(self)
    }

    #[inline]
    fn trunc(self) -> Self {
        libm::trunc(self)
    }

    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        libm::fma(self, a, b)
    }

    #[inline]
    fn div_euclid(self, rhs: Self) -> Self {
        // Based on https://doc.rust-lang.org/src/std/f32.rs.html#293
        let q = (self / rhs).trunc();
        if self % rhs < 0.0 {
            return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
        }

        q
    }

    #[inline]
    fn rem_euclid(self, rhs: Self) -> Self {
        let r = self % rhs;

        if r < 0.0 { r + rhs.abs() } else { r }
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        libm::pow(self, n)
    }

    #[inline]
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }

    #[inline]
    fn exp(self) -> Self {
        libm::exp(self)
    }

    #[inline]
    fn exp2(self) -> Self {
        libm::exp2(self)
    }

    #[inline]
    fn ln(self) -> Self {
        libm::log(self)
    }

    #[inline]
    fn log2(self) -> Self {
        libm::log2(self)
    }

    #[inline]
    fn sin(self) -> Self {
        libm::sin(self)
    }

    #[inline]
    fn cos(self) -> Self {
        libm::cos(self)
    }

    #[inline]
    fn tan(self) -> Self {
        libm::tan(self)
    }

    #[inline]
    fn asin(self) -> Self {
        libm::asin(self)
    }

    #[inline]
    fn acos(self) -> Self {
        libm::acos(self)
    }

    #[inline]
    fn atan(self) -> Self {
        libm::atan(self)
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        libm::sincos(self)
    }
}
