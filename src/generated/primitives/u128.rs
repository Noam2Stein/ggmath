use crate::{vector::{VecAlignment, Vector, VecLen}, Usize};

/// A module with `u128` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod u128_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub U128 => u128);
}

impl<const N: usize, A: VecAlignment> Vector<N, u128, A>
where
    Usize<N>: VecLen,
{
    /// Performs `!self`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn not(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = !self.as_array()[i];
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
	
	/// Performs `self << other`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn shl(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] << other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self >> other`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn shr(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] >> other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self & other`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn bitand(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] & other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self | other`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn bitor(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] | other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self ^ other`.
	/// 
	/// This function exists so that it can be called in const contexts.
	#[inline(always)]
	pub const fn bitxor(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] ^ other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self + other` returning `None` if overflow occurs.
	#[inline(always)]
	pub const fn checked_add(mut self, other: Self) -> Option<Self> {
	    let mut i = 0;
	    while i < N {
	        match self.as_array()[i].checked_add(other.as_array()[i]) {
	            Some(value) => self.as_array_mut()[i] = value,
	            None => return None,
	        };
	        
	        i += 1;
	    }
	
	    Some(self)
	}
	
	/// Performs `self - other` returning `None` if overflow occurs.
	#[inline(always)]
	pub const fn checked_sub(mut self, other: Self) -> Option<Self> {
	    let mut i = 0;
	    while i < N {
	        match self.as_array()[i].checked_sub(other.as_array()[i]) {
	            Some(value) => self.as_array_mut()[i] = value,
	            None => return None,
	        };
	
	        i += 1;
	    }
	
	    Some(self)
	}
	
	/// Performs `self * other` returning `None` if overflow occurs.
	#[inline(always)]
	pub const fn checked_mul(mut self, other: Self) -> Option<Self> {
	    let mut i = 0;
	    while i < N {
	        match self.as_array()[i].checked_mul(other.as_array()[i]) {
	            Some(value) => self.as_array_mut()[i] = value,
	            None => return None,
	        };
	
	        i += 1;
	    }
	
	    Some(self)
	}
	
	/// Performs `self / other` returning `None` if overflow occurs.
	#[inline(always)]
	pub const fn checked_div(mut self, other: Self) -> Option<Self> {
	    let mut i = 0;
	    while i < N {
	        match self.as_array()[i].checked_div(other.as_array()[i]) {
	            Some(value) => self.as_array_mut()[i] = value,
	            None => return None,
	        };
	
	        i += 1;
	    }
	
	    Some(self)
	}
	
	/// Performs `self % other` returning `None` if overflow occurs.
	#[inline(always)]
	pub const fn checked_rem(mut self, other: Self) -> Option<Self> {
	    let mut i = 0;
	    while i < N {
	        match self.as_array()[i].checked_rem(other.as_array()[i]) {
	            Some(value) => self.as_array_mut()[i] = value,
	            None => return None,
	        };
	
	        i += 1;
	    }
	
	    Some(self)
	}
	
	/// Performs `-self` wrapping around at the boundary of the type.
	#[inline(always)]
	pub const fn wrapping_neg(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_neg();
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self + other` wrapping around at the boundary of the type.
	#[inline(always)]
	pub const fn wrapping_add(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_add(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self - other` wrapping around at the boundary of the type.
	#[inline(always)]
	pub const fn wrapping_sub(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_sub(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self * other` wrapping around at the boundary of the type.
	#[inline(always)]
	pub const fn wrapping_mul(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_mul(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self / other` wrapping around at the boundary of the type.
	#[inline(always)]
	pub const fn wrapping_div(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_div(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self % other` wrapping around at the boundary of the type.
	#[inline(always)]
	pub const fn wrapping_rem(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_rem(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self + other` saturating at the numeric bounds instead of overflowing.
	#[inline(always)]
	pub const fn saturating_add(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_add(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self - other` saturating at the numeric bounds instead of overflowing.
	#[inline(always)]
	pub const fn saturating_sub(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_sub(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self * other` saturating at the numeric bounds instead of overflowing.
	#[inline(always)]
	pub const fn saturating_mul(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_mul(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self / other` saturating at the numeric bounds instead of overflowing.
	#[inline(always)]
	pub const fn saturating_div(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_div(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
}

impl crate::vector::ScalarZero for u128 {
    const ZERO: u128 = 0 as Self;
}

impl crate::vector::ScalarOne for u128 {
    const ONE: u128 = 1 as Self;
}

