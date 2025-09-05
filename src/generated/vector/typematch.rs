use crate::{Scalar, Usize, VecAligned, VecAlignment, VecLen, VecLenEnum, VecPacked, Vector};

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Matches the constant length of the vector and calls the appropriate function,
	/// returning `O`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_len<O>(
	    self,
	    f2: impl FnOnce(Vector<2, T, A>) -> O,
	    f3: impl FnOnce(Vector<3, T, A>) -> O,
	    f4: impl FnOnce(Vector<4, T, A>) -> O,
	) -> O {
	    unsafe {
	        match Usize::<N>::ENUM {
	            VecLenEnum::Two => f2(self.transmute_len::<2>()),
	            VecLenEnum::Three => f3(self.transmute_len::<3>()),
	            VecLenEnum::Four => f4(self.transmute_len::<4>()),
	        }
	    }
	}
	
	/// Matches the constant alignment of the vector and calls the appropriate function,
	/// returning `O`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_alignment<O>(
	    self,
	    f_aligned: impl FnOnce(Vector<N, T, VecAligned>) -> O,
	    f_packed: impl FnOnce(Vector<N, T, VecPacked>) -> O,
	) -> O {
	    unsafe {
	        match A::IS_ALIGNED {
	            true => f_aligned(self.transmute_alignment()),
	            false => f_packed(self.transmute_alignment()),
	        }
	    }
	}
	
	/// Checks if `T` is the same type as `T2` and calls the appropriate function,
	/// returning `O`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_scalar_eq<T2: Scalar, O>(
	    self,
	    f_eq: impl FnOnce(Vector<N, T2, A>) -> O,
	    f_ne: impl FnOnce(Vector<N, T, A>) -> O,
	) -> O {
	    unsafe {
	        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<T2>() {
	            f_eq(self.transmute_scalar::<T2>())
	        } else {
	            f_ne(self)
	        }
	    }
	}
	
	/// Matches the constant length of the vector and calls the appropriate function,
	/// returning `O`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_len_ref<O>(
	    &self,
	    f2: impl FnOnce(&Vector<2, T, A>) -> O,
	    f3: impl FnOnce(&Vector<3, T, A>) -> O,
	    f4: impl FnOnce(&Vector<4, T, A>) -> O,
	) -> O {
	    unsafe {
	        match Usize::<N>::ENUM {
	            VecLenEnum::Two => f2(self.transmute_len_ref::<2>()),
	            VecLenEnum::Three => f3(self.transmute_len_ref::<3>()),
	            VecLenEnum::Four => f4(self.transmute_len_ref::<4>()),
	        }
	    }
	}
	
	/// Matches the constant alignment of the vector and calls the appropriate function,
	/// returning `O`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_alignment_ref<O>(
	    &self,
	    f_aligned: impl FnOnce(&Vector<N, T, VecAligned>) -> O,
	    f_packed: impl FnOnce(&Vector<N, T, VecPacked>) -> O,
	) -> O {
	    unsafe {
	        match A::IS_ALIGNED {
	            true => f_aligned(self.transmute_alignment_ref()),
	            false => f_packed(self.transmute_alignment_ref()),
	        }
	    }
	}
	
	/// Checks if `T` is the same type as `T2` and calls the appropriate function,
	/// returning `O`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_scalar_eq_ref<T2: Scalar, O>(
	    &self,
	    f_eq: impl FnOnce(&Vector<N, T2, A>) -> O,
	    f_ne: impl FnOnce(&Vector<N, T, A>) -> O,
	) -> O {
	    unsafe {
	        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<T2>() {
	            f_eq(self.transmute_scalar_ref::<T2>())
	        } else {
	            f_ne(self)
	        }
	    }
	}
	
	/// Matches the constant length of the vector and calls the appropriate function,
	/// returning `O`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_len_mut<O>(
	    &mut self,
	    f2: impl FnOnce(&mut Vector<2, T, A>) -> O,
	    f3: impl FnOnce(&mut Vector<3, T, A>) -> O,
	    f4: impl FnOnce(&mut Vector<4, T, A>) -> O,
	) -> O {
	    unsafe {
	        match Usize::<N>::ENUM {
	            VecLenEnum::Two => f2(self.transmute_len_mut::<2>()),
	            VecLenEnum::Three => f3(self.transmute_len_mut::<3>()),
	            VecLenEnum::Four => f4(self.transmute_len_mut::<4>()),
	        }
	    }
	}
	
	/// Matches the constant alignment of the vector and calls the appropriate function,
	/// returning `O`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_alignment_mut<O>(
	    &mut self,
	    f_aligned: impl FnOnce(&mut Vector<N, T, VecAligned>) -> O,
	    f_packed: impl FnOnce(&mut Vector<N, T, VecPacked>) -> O,
	) -> O {
	    unsafe {
	        match A::IS_ALIGNED {
	            true => f_aligned(self.transmute_alignment_mut()),
	            false => f_packed(self.transmute_alignment_mut()),
	        }
	    }
	}
	
	/// Checks if `T` is the same type as `T2` and calls the appropriate function,
	/// returning `O`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_scalar_eq_mut<T2: Scalar, O>(
	    &mut self,
	    f_eq: impl FnOnce(&mut Vector<N, T2, A>) -> O,
	    f_ne: impl FnOnce(&mut Vector<N, T, A>) -> O,
	) -> O {
	    unsafe {
	        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<T2>() {
	            f_eq(self.transmute_scalar_mut::<T2>())
	        } else {
	            f_ne(self)
	        }
	    }
	}
	
	/// Matches the constant length of the vector and calls the appropriate function,
	/// returning `Vector<N, O, A>`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_len_into_vector<O: Scalar>(
	    self,
	    f_2: impl FnOnce(Vector<2, T, A>) -> Vector<2, O, A>,
	    f_3: impl FnOnce(Vector<3, T, A>) -> Vector<3, O, A>,
	    f_4: impl FnOnce(Vector<4, T, A>) -> Vector<4, O, A>,
	) -> Vector<N, O, A> {
	    unsafe {
	        match Usize::<N>::ENUM {
	            VecLenEnum::Two => f_2(self.transmute_len::<2>()).transmute_len::<N>(),
	            VecLenEnum::Three => f_3(self.transmute_len::<3>()).transmute_len::<N>(),
	            VecLenEnum::Four => f_4(self.transmute_len::<4>()).transmute_len::<N>(),
	        }
	    }
	}
	
	/// Matches the constant alignment of the vector and calls the appropriate function,
	/// returning `Vector<N, O, A>`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_alignment_into_vector<O: Scalar>(
	    self,
	    f_aligned: impl FnOnce(Vector<N, T, VecAligned>) -> Vector<N, O, VecAligned>,
	    f_packed: impl FnOnce(Vector<N, T, VecPacked>) -> Vector<N, O, VecPacked>,
	) -> Vector<N, O, A> {
	    unsafe {
	        match A::IS_ALIGNED {
	            true => f_aligned(self.transmute_alignment::<VecAligned>()).transmute_alignment::<A>(),
	            false => f_packed(self.transmute_alignment::<VecPacked>()).transmute_alignment::<A>(),
	        }
	    }
	}
	
	/// Checks if `T` is the same type as `T2` and calls the appropriate function,
	/// returning `Vector<N, T, A>`.
	/// This is zero cost when compiler optimizations are enabled.
	#[inline(always)]
	pub fn typematch_scalar_eq_into_vector<T2: Scalar>(
	    self,
	    f_eq: impl FnOnce(Vector<N, T2, A>) -> Vector<N, T2, A>,
	    f_ne: impl FnOnce(Vector<N, T, A>) -> Vector<N, T, A>,
	) -> Vector<N, T, A> {
	    unsafe {
	        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<T2>() {
	            f_eq(self.transmute_scalar::<T2>()).transmute_scalar::<T>()
	        } else {
	            f_ne(self)
	        }
	    }
	}
	  
}
