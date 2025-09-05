use crate::{vector::*, Usize};

impl<const N: usize, A: VecAlignment> Vector<N, u128, A>
where
    Usize<N>: VecLen,
{
    /// Performs `-self` returning `None` if overflow occurs.
	#[inline(always)]
	pub fn checked_neg(self) -> Option<Self> {
	    self.map(|x| x.checked_neg()).flatten()
	}
	
	/// Performs `self + other` returning `None` if overflow occurs.
	#[inline(always)]
	pub fn checked_add(self, other: Self) -> Option<Self> {
	    Vector::from_fn(|i| self[i].checked_add(other[i])).flatten()
	}
	
	/// Performs `self - other` returning `None` if overflow occurs.
	#[inline(always)]
	pub fn checked_sub(self, other: Self) -> Option<Self> {
	    Vector::from_fn(|i| self[i].checked_sub(other[i])).flatten()
	}
	
	/// Performs `self * other` returning `None` if overflow occurs.
	#[inline(always)]
	pub fn checked_mul(self, other: Self) -> Option<Self> {
	    Vector::from_fn(|i| self[i].checked_mul(other[i])).flatten()
	}
	
	/// Performs `self / other` returning `None` if overflow occurs.
	#[inline(always)]
	pub fn checked_div(self, other: Self) -> Option<Self> {
	    Vector::from_fn(|i| self[i].checked_div(other[i])).flatten()
	}
	
	/// Performs `self % other` returning `None` if overflow occurs.
	#[inline(always)]
	pub fn checked_rem(self, other: Self) -> Option<Self> {
	    Vector::from_fn(|i| self[i].checked_rem(other[i])).flatten()
	}
	
	/// Performs `-self` wrapping around at the boundary of the type.
	#[inline(always)]
	pub fn wrapping_neg(self) -> Self {
	    self.map(|x| x.wrapping_neg())
	}
	
	/// Performs `self + other` wrapping around at the boundary of the type.
	#[inline(always)]
	pub fn wrapping_add(self, other: Self) -> Self {
	    Vector::from_fn(|i| self[i].wrapping_add(other[i]))
	}
	
	/// Performs `self - other` wrapping around at the boundary of the type.
	#[inline(always)]
	pub fn wrapping_sub(self, other: Self) -> Self {
	    Vector::from_fn(|i| self[i].wrapping_sub(other[i]))
	}
	
	/// Performs `self * other` wrapping around at the boundary of the type.
	#[inline(always)]
	pub fn wrapping_mul(self, other: Self) -> Self {
	    Vector::from_fn(|i| self[i].wrapping_mul(other[i]))
	}
	
	/// Performs `self / other` wrapping around at the boundary of the type.
	#[inline(always)]
	pub fn wrapping_div(self, other: Self) -> Self {
	    Vector::from_fn(|i| self[i].wrapping_div(other[i]))
	}
	
	/// Performs `self % other` wrapping around at the boundary of the type.
	#[inline(always)]
	pub fn wrapping_rem(self, other: Self) -> Self {
	    Vector::from_fn(|i| self[i].wrapping_rem(other[i]))
	}
	
	/// Performs `self + other` saturating when an overflow occurs.
	#[inline(always)]
	pub fn saturating_add(self, other: Self) -> Self {
	    Vector::from_fn(|i| self[i].saturating_add(other[i]))
	}
	
	/// Performs `self - other` saturating when an overflow occurs.
	#[inline(always)]
	pub fn saturating_sub(self, other: Self) -> Self {
	    Vector::from_fn(|i| self[i].saturating_sub(other[i]))
	}
	
	/// Performs `self * other` saturating when an overflow occurs.
	#[inline(always)]
	pub fn saturating_mul(self, other: Self) -> Self {
	    Vector::from_fn(|i| self[i].saturating_mul(other[i]))
	}
	
	/// Performs `self / other` saturating when an overflow occurs.
	#[inline(always)]
	pub fn saturating_div(self, other: Self) -> Self {
	    Vector::from_fn(|i| self[i].saturating_div(other[i]))
	}
	
}

impl<const N: usize, A: VecAlignment> Vector<N, u128, A>
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
	
	/// Performs `!self` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_not(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = !self.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self + other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_add(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] + other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self - other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_sub(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] - other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self * other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_mul(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] * other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self / other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_div(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] / other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self % other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_rem(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] % other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self << other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_shl(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] << other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self >> other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_shr(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] >> other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self & other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_bitand(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] & other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self | other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_bitor(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] | other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self ^ other` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_bitxor(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i] ^ other.as_array()[i];
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.checked_neg()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_checked_neg(mut self) -> Option<Self> {
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
	
	/// Performs `self.checked_add(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_checked_add(mut self, other: Self) -> Option<Self> {
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
	
	/// Performs `self.checked_sub(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_checked_sub(mut self, other: Self) -> Option<Self> {
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
	
	/// Performs `se.checked_mul(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_checked_mul(mut self, other: Self) -> Option<Self> {
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
	
	/// Performs `self.checked_div(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_checked_div(mut self, other: Self) -> Option<Self> {
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
	
	/// Performs `self.checked_rem(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_checked_rem(mut self, other: Self) -> Option<Self> {
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
	
	/// Performs `self.wrapping_neg()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_wrapping_neg(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_neg();
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.wrapping_add(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_wrapping_add(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_add(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.wrapping_sub(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_wrapping_sub(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_sub(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.wrapping_mul(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_wrapping_mul(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_mul(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.wrapping_div(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_wrapping_div(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_div(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.wrapping_rem(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_wrapping_rem(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].wrapping_rem(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.saturating_neg()` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_saturating_neg(mut self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_neg();
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.saturating_add(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_saturating_add(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_add(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.saturating_sub(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_saturating_sub(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_sub(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.saturating_mul(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_saturating_mul(mut self, other: Self) -> Self {
	    let mut i = 0;
	    while i < N {
	        self.as_array_mut()[i] = self.as_array()[i].saturating_mul(other.as_array()[i]);
	        i += 1;
	    }
	
	    self
	}
	
	/// Performs `self.saturating_div(other)` using a slower implementation that supports const contexts.
	#[inline(always)]
	pub const fn const_saturating_div(mut self, other: Self) -> Self {
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

