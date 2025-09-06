use crate::{vector::*, Usize};

impl<const N: usize, A: VecAlignment> Vector<N, f32, A>
where
    Usize<N>: VecLen,
{
    /// A vector with all elements set to `NaN`.
	pub const NAN: Self = Self::splat(f32::NAN);
	/// A vector with all elements set to `Infinity`.
	pub const INFINITY: Self = Self::splat(f32::INFINITY);
	/// A vector with all elements set to `-Infinity`.
	pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);
	/// A vector with all elements set to `Epsilon`.
	pub const EPSILON: Self = Self::splat(f32::EPSILON);
	/// A vector with all elements set to `Min`.
	pub const MIN: Self = Self::splat(f32::MIN);
	/// A vector with all elements set to `Max`.
	pub const MAX: Self = Self::splat(f32::MAX);
	
	/// Returns a vector containing the absolute value of each element of `self`.
	#[inline(always)]
	pub const fn abs(mut self) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].abs();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the signum of each element of `self`.
	/// Signum for each element is:
	/// - `1.0` if the element is positive or `+0.0`
	/// - `-1.0` if the element is negative `-0.0`
	#[inline(always)]
	pub const fn signum(mut self) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].signum();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector of bools with `true` for each element that has a negative sign, including `-0.0`.
	#[inline(always)]
	pub const fn negative_sign_mask(self) -> Vector<N, bool, A> {
	    let mut output = Vector::splat(false);
	
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i].is_sign_negative();
	        i += 1;
	    }
	
	    output
	}
	
	/// Returns a vector of bools with `true` for each element that has a positive sign, including `+0.0`.
	#[inline(always)]
	pub const fn positive_sign_mask(self) -> Vector<N, bool, A> {
	    let mut output = Vector::splat(false);
	
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i].is_sign_positive();
	        i += 1;
	    }
	
	    output
	}
	
	/// Returns a vector of bools with `true` for each element that is `NaN`.
	#[inline(always)]
	pub const fn nan_mask(self) -> Vector<N, bool, A> {
	    let mut output = Vector::splat(false);
	
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i].is_nan();
	        i += 1;
	    }
	
	    output
	}
	
	/// Returns a vector of bools with `true` for each element that is finite.
	#[inline(always)]
	pub const fn finite_mask(self) -> Vector<N, bool, A> {
	    let mut output = Vector::splat(false);
	
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i].is_finite();
	        i += 1;
	    }
	
	    output
	}
	
	/// Returns `true` if any element is `NaN`.
	#[inline(always)]
	pub const fn is_nan(self) -> bool {
	    self.nan_mask().any_true()
	}
	
	/// Returns `true` if all elements are finite.
	#[inline(always)]
	pub const fn is_finite(self) -> bool {
	    self.finite_mask().all_true()
	}
	
	/// Returns a vector containing the square root of each element of `self`.
	#[inline(always)]
	pub fn sqrt(self) -> Self {
	    self.map(|x| x.sqrt())
	}
	
	/// Returns the magnitude of `self`.
	#[inline(always)]
	pub fn mag(self) -> f32 {
	    self.mag_sq().sqrt()
	}
	
	/// Returns the Euclidean distance between `self` and `other`.
	#[inline(always)]
	pub fn distance(self, other: Self) -> f32 {
	    self.distance_sq(other).sqrt()
	}
	
	/// Returns a vector containing the rounded value of each element of `self`.
	#[inline(always)]
	pub fn round(self) -> Self {
	    self.map(|x| x.round())
	}
	
	/// Returns a vector containing the floor value of each element of `self`.
	#[inline(always)]
	pub fn floor(self) -> Self {
	    self.map(|x| x.floor())
	}
	
	/// Returns a vector containing the ceiling value of each element of `self`.
	#[inline(always)]
	pub fn ceil(self) -> Self {
	    self.map(|x| x.ceil())
	}
	
	/// Returns a vector containing the truncated value of each element of `self`.
	#[inline(always)]
	pub fn trunc(self) -> Self {
	    self.map(|x| x.trunc())
	}
	
	/// Returns a vector containing the fractional part of each element of `self` as `self - self.trunc()`.
	#[inline(always)]
	pub fn fract(self) -> Self {
	    self.map(|x| x.fract())
	}
	
	/// Returns a vector containing the sine of each element of `self`.
	#[inline(always)]
	pub fn sin(self) -> Self {
	    self.map(|x| x.sin())
	}
	
	/// Returns a vector containing the cosine of each element of `self`.
	#[inline(always)]
	pub fn cos(self) -> Self {
	    self.map(|x| x.cos())
	}
	
	/// Returns a vector containing the tangent of each element of `self`.
	#[inline(always)]
	pub fn tan(self) -> Self {
	    self.map(|x| x.tan())
	}
	
	/// Returns a vector containing the arcsine of each element of `self`.
	#[inline(always)]
	pub fn asin(self) -> Self {
	    self.map(|x| x.asin())
	}
	
	/// Returns a vector containing the arccosine of each element of `self`.
	#[inline(always)]
	pub fn acos(self) -> Self {
	    self.map(|x| x.acos())
	}
	
	/// Returns a vector containing the arctangent of each element of `self`.
	#[inline(always)]
	pub fn atan(self) -> Self {
	    self.map(|x| x.atan())
	}
	
	/// Returns a vector containing the hyperbolic sine of each element of `self`.
	#[inline(always)]
	pub fn sinh(self) -> Self {
	    self.map(|x| x.sinh())
	}
	
	/// Returns a vector containing the hyperbolic cosine of each element of `self`.
	#[inline(always)]
	pub fn cosh(self) -> Self {
	    self.map(|x| x.cosh())
	}
	
	/// Returns a vector containing the hyperbolic tangent of each element of `self`.
	#[inline(always)]
	pub fn tanh(self) -> Self {
	    self.map(|x| x.tanh())
	}
	
	/// Returns a vector containing the hyperbolic arclength sine of each element of `self`.
	#[inline(always)]
	pub fn asinh(self) -> Self {
	    self.map(|x| x.asinh())
	}
	
	/// Returns a vector containing the hyperbolic arclength cosine of each element of `self`.
	#[inline(always)]
	pub fn acosh(self) -> Self {
	    self.map(|x| x.acosh())
	}
	
	/// Returns a vector containing the hyperbolic arclength tangent of each element of `self`.
	#[inline(always)]
	pub fn atanh(self) -> Self {
	    self.map(|x| x.atanh())
	}
	
}

impl<const N: usize, A: VecAlignment> Vector<N, f32, A>
where
    Usize<N>: VecLen,
{
    /// Returns `-self` and supports const contexts.
	#[inline(always)]
	pub const fn const_neg(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = -self.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self + other` and supports const contexts.
	#[inline(always)]
	pub const fn const_add(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] + other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self - other` and supports const contexts.
	#[inline(always)]
	pub const fn const_sub(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] - other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self * other` and supports const contexts.
	#[inline(always)]
	pub const fn const_mul(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] * other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self / other` and supports const contexts.
	#[inline(always)]
	pub const fn const_div(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] / other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self % other` and supports const contexts.
	#[inline(always)]
	pub const fn const_rem(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] % other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self.mag_sq()` and supports const contexts.
	#[inline(always)]
	pub const fn const_mag_sq(self) -> f32 {
	    let mut output = 0.0;
	
	    let mut i = 0;
	    while i < N {
	        output += self.as_array()[i] * self.as_array()[i];
	        i += 1;
	    }
	
	    output
	}
	
	/// Returns `self.distance_sq(other)` and supports const contexts.
	#[inline(always)]
	pub const fn const_distance_sq(self, other: Self) -> f32 {
	    self.const_sub(other).const_mag_sq()
	}
	
	/// Returns `self == other` and supports const contexts.
	#[inline(always)]
	pub const fn const_eq(self, other: Vector<N, f32, impl VecAlignment>) -> bool {
	    let mut i = 0;
	    while i < N {
	        if self.as_array()[i] != other.as_array()[i] {
	            return false;
	        }
	        i += 1;
	    }
	    true
	}
	
	/// Returns `self != other` and supports const contexts.
	#[inline(always)]
	pub const fn const_ne(self, other: Vector<N, f32, impl VecAlignment>) -> bool {
	    let mut i = 0;
	    while i < N {
	        if self.as_array()[i] != other.as_array()[i] {
	            return true;
	        }
	        i += 1;
	    }
	    false
	}
	
	/// Returns `self.eq_mask(other)` and supports const contexts.
	pub const fn const_eq_mask(
	    self,
	    other: Vector<N, f32, impl VecAlignment>,
	) -> Vector<N, bool, A> {
	    let mut output = Vector::<N, bool, A>::splat(false);
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i] == other.as_array()[i];
	        i += 1;
	    }
	    output
	}
	
	/// Returns `self.ne_mask(other)` and supports const contexts.
	pub const fn const_ne_mask(
	    self,
	    other: Vector<N, f32, impl VecAlignment>,
	) -> Vector<N, bool, A> {
	    let mut output = Vector::<N, bool, A>::splat(false);
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i] != other.as_array()[i];
	        i += 1;
	    }
	    output
	}
	
	/// Returns `self.lt_mask(other)` and supports const contexts.
	pub const fn const_lt_mask(
	    self,
	    other: Vector<N, f32, impl VecAlignment>,
	) -> Vector<N, bool, A> {
	    let mut output = Vector::<N, bool, A>::splat(false);
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i] < other.as_array()[i];
	        i += 1;
	    }
	    output
	}
	
	/// Returns `self.gt_mask(other)` and supports const contexts.
	pub const fn const_gt_mask(
	    self,
	    other: Vector<N, f32, impl VecAlignment>,
	) -> Vector<N, bool, A> {
	    let mut output = Vector::<N, bool, A>::splat(false);
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i] > other.as_array()[i];
	        i += 1;
	    }
	    output
	}
	
	/// Returns `self.le_mask(other)` and supports const contexts.
	pub const fn const_le_mask(
	    self,
	    other: Vector<N, f32, impl VecAlignment>,
	) -> Vector<N, bool, A> {
	    let mut output = Vector::<N, bool, A>::splat(false);
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i] <= other.as_array()[i];
	        i += 1;
	    }
	    output
	}
	
	/// Returns `self.ge_mask(other)` and supports const contexts.
	pub const fn const_ge_mask(
	    self,
	    other: Vector<N, f32, impl VecAlignment>,
	) -> Vector<N, bool, A> {
	    let mut output = Vector::<N, bool, A>::splat(false);
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i] >= other.as_array()[i];
	        i += 1;
	    }
	    output
	}
	
}

impl crate::vector::ScalarZero for f32 {
    const ZERO: f32 = 0 as Self;
}

impl crate::vector::ScalarOne for f32 {
    const ONE: f32 = 1 as Self;
}

impl crate::vector::ScalarNegOne for f32 {
    const NEG_ONE: f32 = -1 as Self;
}

