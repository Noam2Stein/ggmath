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
	pub fn abs(self) -> Self {
	    if self.is_aligned() {
	        unsafe {
	            match self.len() {
	                2 => return Vector::<2, _, VecAligned>::from_inner(self.transmute_vec2().inner.abs()).transmute_len::<N>().transmute_alignment::<A>(),
	                3 => return Vector::<3, _, VecAligned>::from_inner(self.transmute_vec3().inner.abs()).transmute_len::<N>().transmute_alignment::<A>(),
	                4 => return Vector::<4, _, VecAligned>::from_inner(self.transmute_vec4().inner.abs()).transmute_len::<N>().transmute_alignment::<A>(),
	                _ => {},
	            }
	        }
	    }
	
	    self.map(|x| x.abs())
	}
	
	/// Returns a vector containing elements that represent the sign of each element of `self`.
	/// - `1.0` if the element is positive
	/// - `-1.0` if the element is negative
	/// - `0.0` if the element is zero
	#[inline(always)]
	pub fn signum(self) -> Self {
	    if self.is_aligned() {
	        unsafe {
	            match self.len() {
	                2 => return Vector::<2, _, VecAligned>::from_inner(self.transmute_vec2().inner.signum()).transmute_len::<N>().transmute_alignment::<A>(),
	                3 => return Vector::<3, _, VecAligned>::from_inner(self.transmute_vec3().inner.signum()).transmute_len::<N>().transmute_alignment::<A>(),
	                4 => return Vector::<4, _, VecAligned>::from_inner(self.transmute_vec4().inner.signum()).transmute_len::<N>().transmute_alignment::<A>(),
	                _ => {},
	            }
	        }
	    }
	
	    self.map(|x| x.signum())
	}
	
	/// Returns a vector containing the square root of each element of `self`.
	#[inline(always)]
	pub fn sqrt(self) -> Self {
	    self.map(|x| x.sqrt())
	}
	
	/// Returns a vector containing a `true` value for each element of `self` that is negative.
	/// 
	/// An element is negative if it has a negative sign, which includes `-0.0`.
	#[inline(always)]
	pub fn negative_sign_mask(self) -> Vector<N, bool, A> {
	    self.map(|x| x.is_sign_negative())
	}
	
	/// Returns a vector containing a `true` value for each element of `self` that is positive.
	/// 
	/// An element is positive if it has a positive sign, which includes `+0.0`.
	#[inline(always)]
	pub fn positive_sign_mask(self) -> Vector<N, bool, A> {
	    self.map(|x| x.is_sign_positive())
	}
	
	/// Returns a vector containing a `true` value for each element of `self` that is NaN.
	#[inline(always)]
	pub fn nan_mask(self) -> Vector<N, bool, A> {
	    if self.is_aligned() {
	        unsafe {
	            match self.len() {
	                2 => return Vector::<2, _, VecAligned>::from_inner(self.transmute_vec2().inner.is_nan_mask()).transmute_len::<N>().transmute_alignment::<A>(),
	                3 => return Vector::<3, _, VecAligned>::from_inner(self.transmute_vec3().inner.is_nan_mask()).transmute_len::<N>().transmute_alignment::<A>(),
	                4 => return Vector::<4, _, VecAligned>::from_inner(self.transmute_vec4().inner.is_nan_mask()).transmute_len::<N>().transmute_alignment::<A>(),
	                _ => {},
	            }
	        }
	    }
	
	    self.map(|x| x.is_nan())
	}
	
	/// Returns a vector containing a `true` value for each element of `self` that is finite.
	#[inline(always)]
	pub fn finite_mask(self) -> Vector<N, bool, A> {
	    if self.is_aligned() {
	        unsafe {
	            match self.len() {
	                2 => return Vector::<2, _, VecAligned>::from_inner(self.transmute_vec2().inner.is_finite_mask()).transmute_len::<N>().transmute_alignment::<A>(),
	                3 => return Vector::<3, _, VecAligned>::from_inner(self.transmute_vec3().inner.is_finite_mask()).transmute_len::<N>().transmute_alignment::<A>(),
	                4 => return Vector::<4, _, VecAligned>::from_inner(self.transmute_vec4().inner.is_finite_mask()).transmute_len::<N>().transmute_alignment::<A>(),
	                _ => {},
	            }
	        }
	    }
	
	    self.map(|x| x.is_finite())
	}
	
	/// Returns `true` if any element of `self` is NaN.
	#[inline(always)]
	pub fn is_nan(self) -> bool {
	    if self.is_aligned() {
	        unsafe {
	            match self.len() {
	                2 => return self.transmute_vec2().inner.is_nan(),
	                3 => return self.transmute_vec3().inner.is_nan(),
	                4 => return self.transmute_vec4().inner.is_nan(),
	                _ => {},
	            }
	        }
	    }
	
	    self.nan_mask().any_true()
	}
	
	/// Returns `true` if all elements of `self` are finite.
	#[inline(always)]
	pub fn is_finite(self) -> bool {
	    if self.is_aligned() {
	        unsafe {
	            match self.len() {
	                2 => return self.transmute_vec2().inner.is_finite(),
	                3 => return self.transmute_vec3().inner.is_finite(),
	                4 => return self.transmute_vec4().inner.is_finite(),
	                _ => {},
	            }
	        }
	    }
	
	    self.finite_mask().all_true()
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
	    if self.is_aligned() {
	        unsafe {
	            match self.len() {
	                2 => return Vector::<2, _, VecAligned>::from_inner(self.transmute_vec2().inner.round()).transmute_len::<N>().transmute_alignment::<A>(),
	                3 => return Vector::<3, _, VecAligned>::from_inner(self.transmute_vec3().inner.round()).transmute_len::<N>().transmute_alignment::<A>(),
	                4 => return Vector::<4, _, VecAligned>::from_inner(self.transmute_vec4().inner.round()).transmute_len::<N>().transmute_alignment::<A>(),
	                _ => {},
	            }
	        }
	    }
	
	    self.map(|x| x.round())
	}
	
	/// Returns a vector containing the truncated value of each element of `self`.
	#[inline(always)]
	pub fn trunc(self) -> Self {
	    if self.is_aligned() {
	        unsafe {
	            match self.len() {
	                2 => return Vector::<2, _, VecAligned>::from_inner(self.transmute_vec2().inner.trunc()).transmute_len::<N>().transmute_alignment::<A>(),
	                3 => return Vector::<3, _, VecAligned>::from_inner(self.transmute_vec3().inner.trunc()).transmute_len::<N>().transmute_alignment::<A>(),
	                4 => return Vector::<4, _, VecAligned>::from_inner(self.transmute_vec4().inner.trunc()).transmute_len::<N>().transmute_alignment::<A>(),
	                _ => {},
	            }
	        }
	    }
	
	    self.map(|x| x.trunc())
	}
	
	/// Returns a vector containing the floor value of each element of `self`.
	#[inline(always)]
	pub fn floor(self) -> Self {
	    if self.is_aligned() {
	        unsafe {
	            match self.len() {
	                2 => return Vector::<2, _, VecAligned>::from_inner(self.transmute_vec2().inner.floor()).transmute_len::<N>().transmute_alignment::<A>(),
	                3 => return Vector::<3, _, VecAligned>::from_inner(self.transmute_vec3().inner.floor()).transmute_len::<N>().transmute_alignment::<A>(),
	                4 => return Vector::<4, _, VecAligned>::from_inner(self.transmute_vec4().inner.floor()).transmute_len::<N>().transmute_alignment::<A>(),
	                _ => {},
	            }
	        }
	    }
	
	    self.map(|x| x.floor())
	}
	
	/// Returns a vector containing the ceiling value of each element of `self`.
	#[inline(always)]
	pub fn ceil(self) -> Self {
	    if self.is_aligned() {
	        unsafe {
	            match self.len() {
	                2 => return Vector::<2, _, VecAligned>::from_inner(self.transmute_vec2().inner.ceil()).transmute_len::<N>().transmute_alignment::<A>(),
	                3 => return Vector::<3, _, VecAligned>::from_inner(self.transmute_vec3().inner.ceil()).transmute_len::<N>().transmute_alignment::<A>(),
	                4 => return Vector::<4, _, VecAligned>::from_inner(self.transmute_vec4().inner.ceil()).transmute_len::<N>().transmute_alignment::<A>(),
	                _ => {},
	            }
	        }
	    }
	
	    self.map(|x| x.ceil())
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
    /// Performs `-self` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_neg(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = -self.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self + other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_add(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] + other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self - other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_sub(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] - other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self * other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_mul(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] * other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self / other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_div(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] / other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self % other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_rem(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] % other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.abs()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_abs(mut self) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].abs();
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.signum()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_signum(mut self) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].signum();
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.negative_sign_mask()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_negative_sign_mask(self) -> Vector<N, bool, A> {
	    let mut output = Vector::splat(false);
	
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i].is_sign_negative();
	        i += 1;
	    }
	
	    output
	}
	
	/// Performs `self.positive_sign_mask()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_positive_sign_mask(self) -> Vector<N, bool, A> {
	    let mut output = Vector::splat(false);
	
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i].is_sign_positive();
	        i += 1;
	    }
	
	    output
	}
	
	/// Performs `self.nan_mask()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_nan_mask(self) -> Vector<N, bool, A> {
	    let mut output = Vector::splat(false);
	
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i].is_nan();
	        i += 1;
	    }
	
	    output
	}
	
	/// Performs `self.finite_mask()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_finite_mask(self) -> Vector<N, bool, A> {
	    let mut output = Vector::splat(false);
	
	    let mut i = 0;
	    while i < N {
	        output.as_array_mut()[i] = self.as_array()[i].is_finite();
	        i += 1;
	    }
	
	    output
	}
	
	/// Performs `self.is_nan()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_is_nan(self) -> bool {
	    self.const_nan_mask().const_any_true()
	}
	
	/// Performs `self.is_finite()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_is_finite(self) -> bool {
	    self.const_finite_mask().const_all_true()
	}
	
	/// Performs `self.mag_sq()` using a slower implementation that supports const contexts.
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
	
	/// Performs `self.distance_sq(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_distance_sq(self, other: Self) -> f32 {
	    self.const_sub(other).const_mag_sq()
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

