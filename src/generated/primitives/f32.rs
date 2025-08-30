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
    /// Applies Negation to each element of the vector.
	/// 
	/// This function exists to allow vector Negation to be used in const contexts.
	#[inline(always)]
	pub const fn neg(mut self) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = -self.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Addition to each element of the vector.
	/// 
	/// This function exists to allow vector Addition to be used in const contexts.
	#[inline(always)]
	pub const fn add(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] + other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Subtraction to each element of the vector.
	/// 
	/// This function exists to allow vector Subtraction to be used in const contexts.
	#[inline(always)]
	pub const fn sub(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] - other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Multiplication to each element of the vector.
	/// 
	/// This function exists to allow vector Multiplication to be used in const contexts.
	#[inline(always)]
	pub const fn mul(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] * other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Division to each element of the vector.
	/// 
	/// This function exists to allow vector Division to be used in const contexts.
	#[inline(always)]
	pub const fn div(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] / other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Remainder to each element of the vector.
	/// 
	/// This function exists to allow vector Remainder to be used in const contexts.
	#[inline(always)]
	pub const fn rem(mut self, other: Vector<N, f32, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] % other.as_array()[i];
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
