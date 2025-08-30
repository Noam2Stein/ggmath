use crate::{vector::{VecAlignment, Vector, VecLen}, Usize};

/// A module with `usize` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod usize_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub Usize => usize);
}

impl<const N: usize, A: VecAlignment> Vector<N, usize, A>
where
    Usize<N>: VecLen,
{
    /// Applies Bitwise NOT to each element of the vector.
	/// 
	/// This function exists to allow vector Bitwise NOT to be used in const contexts.
	#[inline(always)]
	pub const fn not(mut self) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = !self.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Addition to each element of the vector.
	/// 
	/// This function exists to allow vector Addition to be used in const contexts.
	#[inline(always)]
	pub const fn add(mut self, other: Vector<N, usize, impl VecAlignment>) -> Self {
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
	pub const fn sub(mut self, other: Vector<N, usize, impl VecAlignment>) -> Self {
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
	pub const fn mul(mut self, other: Vector<N, usize, impl VecAlignment>) -> Self {
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
	pub const fn div(mut self, other: Vector<N, usize, impl VecAlignment>) -> Self {
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
	pub const fn rem(mut self, other: Vector<N, usize, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] % other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Bitwise AND to each element of the vector.
	/// 
	/// This function exists to allow vector Bitwise AND to be used in const contexts.
	#[inline(always)]
	pub const fn and(mut self, other: Vector<N, usize, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] & other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Bitwise OR to each element of the vector.
	/// 
	/// This function exists to allow vector Bitwise OR to be used in const contexts.
	#[inline(always)]
	pub const fn or(mut self, other: Vector<N, usize, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] | other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Bitwise XOR to each element of the vector.
	/// 
	/// This function exists to allow vector Bitwise XOR to be used in const contexts.
	#[inline(always)]
	pub const fn xor(mut self, other: Vector<N, usize, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] ^ other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Left Shift to each element of the vector.
	/// 
	/// This function exists to allow vector Left Shift to be used in const contexts.
	#[inline(always)]
	pub const fn shl(mut self, other: Vector<N, usize, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] << other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Applies Right Shift to each element of the vector.
	/// 
	/// This function exists to allow vector Right Shift to be used in const contexts.
	#[inline(always)]
	pub const fn shr(mut self, other: Vector<N, usize, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] >> other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
}

impl crate::vector::ScalarZero for usize {
    const ZERO: usize = 0 as Self;
}

impl crate::vector::ScalarOne for usize {
    const ONE: usize = 1 as Self;
}
