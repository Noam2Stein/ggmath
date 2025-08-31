use crate::{vector::{VecAlignment, Vector, VecLen}, Usize};

/// A module with `f32` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod f32_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub F => f32);
}

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
	
	/// Performs `-self`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn neg(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = -self.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self + other`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn add(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] + other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self - other`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn sub(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] - other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self * other`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn mul(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] * other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self / other`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn div(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] / other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self % other`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn rem(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] % other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
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
	
	/// Returns a vector containing elements that represent the sign of each element of `self`.
	/// - `1.0` if the element is positive
	/// - `-1.0` if the element is negative
	/// - `0.0` if the element is zero
	#[inline(always)]
	pub const fn signum(mut self) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].signum();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing a `true` value for each element of `self` that is negative.
	/// 
	/// An element is negative if it has a negative sign, which includes `-0.0`.
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
	
	/// Returns a vector containing a `true` value for each element of `self` that is positive.
	/// 
	/// An element is positive if it has a positive sign, which includes `+0.0`.
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
	
	/// Returns `true` if any element of `self` is NaN.
	#[inline(always)]
	pub const fn is_nan(self) -> bool {
	    self.nan_mask().any_true()
	}
	
	/// Returns a vector containing a `true` value for each element of `self` that is NaN.
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
	
	/// Returns `true` if all elements of `self` are finite.
	#[inline(always)]
	pub const fn is_finite(self) -> bool {
	    self.finite_mask().all_true()
	}
	
	/// Returns a vector containing a `true` value for each element of `self` that is finite.
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
	
	/// Returns a vector containing the square root of each element of `self`.
	#[inline(always)]
	pub fn sqrt(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].sqrt();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns the magnitude of `self`.
	#[inline(always)]
	pub fn mag(self) -> f32 {
	    self.const_mag_sq().sqrt()
	}
	
	/// Returns the square of the magnitude of `self`.
	/// 
	/// This function exists to allow the function to be used in const contexts.
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
	
	/// Returns a vector containing the sine of each element of `self`.
	#[inline(always)]
	pub fn sin(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].sin();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the cosine of each element of `self`.
	#[inline(always)]
	pub fn cos(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].cos();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the tangent of each element of `self`.
	#[inline(always)]
	pub fn tan(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].tan();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the arcsine of each element of `self`.
	#[inline(always)]
	pub fn asin(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].asin();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the arccosine of each element of `self`.
	#[inline(always)]
	pub fn acos(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].acos();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the arctangent of each element of `self`.
	#[inline(always)]
	pub fn atan(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].atan();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the hyperbolic sine of each element of `self`.
	#[inline(always)]
	pub fn sinh(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].sinh();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the hyperbolic cosine of each element of `self`.
	#[inline(always)]
	pub fn cosh(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].cosh();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the hyperbolic tangent of each element of `self`.
	#[inline(always)]
	pub fn tanh(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].tanh();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the hyperbolic arclength sine of each element of `self`.
	#[inline(always)]
	pub fn asinh(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].asinh();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the hyperbolic arclength cosine of each element of `self`.
	#[inline(always)]
	pub fn acosh(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].acosh();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns a vector containing the hyperbolic arclength tangent of each element of `self`.
	#[inline(always)]
	pub fn atanh(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].atanh();
	        i += 1;
	    }
	
	    self
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

