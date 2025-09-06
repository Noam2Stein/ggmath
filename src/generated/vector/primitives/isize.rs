use crate::{vector::*, Usize};

impl<const N: usize, A: VecAlignment> Vector<N, isize, A>
where
    Usize<N>: VecLen,
{
    /// Returns `-self` or `None` if there is an overflow.
	#[inline(always)]
	pub const fn checked_neg(mut self) -> Option<Self> {
	    let mut i = 0;
	    while i < N {
	        match self.as_array()[i].checked_neg() {
	            Some(value) => self.as_array_mut()[i] = value,
	            None => return None,
	        };
	        
	        i += 1;
	    }
	
	    Some(self)
	}
	
	/// Returns `self + other` or `None` if there is an overflow.
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
	
	/// Returns `self - other` or `None` if there is an overflow.
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
	
	/// Returns `self * other` or `None` if there is an overflow.
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
	
	/// Returns `self / other` or `None` if there is an overflow.
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
	
	/// Returns `self % other` or `None` if there is an overflow.
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
	
	/// Returns `-self` with wrapping arithmetic.
	#[inline(always)]
	pub const fn wrapping_neg(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_neg();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self + other` with wrapping arithmetic.
	#[inline(always)]
	pub const fn wrapping_add(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_add(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self - other` with wrapping arithmetic.
	#[inline(always)]
	pub const fn wrapping_sub(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_sub(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self * other` with wrapping arithmetic.
	#[inline(always)]
	pub const fn wrapping_mul(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_mul(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self / other` with wrapping arithmetic.
	#[inline(always)]
	pub const fn wrapping_div(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_div(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self % other` with wrapping arithmetic.
	#[inline(always)]
	pub const fn wrapping_rem(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_rem(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `-self` with saturating arithmetic.
	#[inline(always)]
	pub const fn saturating_neg(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_neg();
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self + other` with saturating arithmetic.
	#[inline(always)]
	pub const fn saturating_add(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_add(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self - other` with saturating arithmetic.
	#[inline(always)]
	pub const fn saturating_sub(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_sub(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self * other` with saturating arithmetic.
	#[inline(always)]
	pub const fn saturating_mul(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_mul(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self / other` with saturating arithmetic.
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

impl<const N: usize, A: VecAlignment> Vector<N, isize, A>
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
	
	/// Returns `!self` and supports const contexts.
	#[inline(always)]
	pub const fn const_not(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = !self.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self + other` and supports const contexts.
	#[inline(always)]
	pub const fn const_add(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] + other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self - other` and supports const contexts.
	#[inline(always)]
	pub const fn const_sub(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] - other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self * other` and supports const contexts.
	#[inline(always)]
	pub const fn const_mul(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] * other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self / other` and supports const contexts.
	#[inline(always)]
	pub const fn const_div(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] / other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self % other` and supports const contexts.
	#[inline(always)]
	pub const fn const_rem(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] % other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self << other` and supports const contexts.
	#[inline(always)]
	pub const fn const_shl(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] << other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self >> other` and supports const contexts.
	#[inline(always)]
	pub const fn const_shr(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] >> other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self & other` and supports const contexts.
	#[inline(always)]
	pub const fn const_bitand(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] & other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self | other` and supports const contexts.
	#[inline(always)]
	pub const fn const_bitor(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] | other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self ^ other` and supports const contexts.
	#[inline(always)]
	pub const fn const_bitxor(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] ^ other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Returns `self == other` and supports const contexts.
	#[inline(always)]
	pub const fn const_eq(self, other: Vector<N, isize, impl VecAlignment>) -> bool {
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
	pub const fn const_ne(self, other: Vector<N, isize, impl VecAlignment>) -> bool {
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
	    other: Vector<N, isize, impl VecAlignment>,
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
	    other: Vector<N, isize, impl VecAlignment>,
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
	    other: Vector<N, isize, impl VecAlignment>,
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
	    other: Vector<N, isize, impl VecAlignment>,
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
	    other: Vector<N, isize, impl VecAlignment>,
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
	    other: Vector<N, isize, impl VecAlignment>,
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

#[cfg(feature = "std")]
impl<const N: usize, A: VecAlignment> Vector<N, isize, A>
where
    Usize<N>: VecLen,
{
    
}

#[cfg(feature = "std")]
impl<const N: usize, A: VecAlignment> Vector<N, isize, A>
where
    Usize<N>: VecLen,
{
    
}

impl crate::vector::ScalarZero for isize {
    const ZERO: isize = 0 as Self;
}

impl crate::vector::ScalarOne for isize {
    const ONE: isize = 1 as Self;
}

impl crate::vector::ScalarNegOne for isize {
    const NEG_ONE: isize = -1 as Self;
}

