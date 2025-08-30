use crate::{vector::{VecAlignment, Vector, VecLen}, Usize};

/// A module with `bool` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod bool_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub B => bool);
}

impl<const N: usize, A: VecAlignment> Vector<N, bool, A>
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
	
	/// Applies Bitwise AND to each element of the vector.
	/// 
	/// This function exists to allow vector Bitwise AND to be used in const contexts.
	#[inline(always)]
	pub const fn and(mut self, other: Vector<N, bool, impl VecAlignment>) -> Self {
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
	pub const fn or(mut self, other: Vector<N, bool, impl VecAlignment>) -> Self {
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
	pub const fn xor(mut self, other: Vector<N, bool, impl VecAlignment>) -> Self {
	    let mut i = 0;
	
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] ^ other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
}
