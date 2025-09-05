use crate::{Scalar, Usize, VecAligned, VecAlignment, VecLen, VecPacked, Vector};

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Transmutes `self` to a vector of a different length.
	///
	/// Only use this if you know that `N2` is the same as `N`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_len<const N2: usize>(self) -> Vector<N2, T, A>
	where
	    Usize<N2>: VecLen,
	{
	    unsafe { std::mem::transmute_copy::<Vector<N, T, A>, Vector<N2, T, A>>(&self) }
	}
	
	/// Transmutes `self` to a vector of a different scalar type.
	///
	/// Only use this if you are sure that `T2` has the same size and alignment as `T`,
	/// and that `T` is transmutable to `T2`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_scalar<T2: Scalar>(self) -> Vector<N, T2, A> {
	    unsafe { std::mem::transmute_copy::<Vector<N, T, A>, Vector<N, T2, A>>(&self) }
	}
	
	/// Transmutes `self` to a vector of a different alignment.
	///
	/// Only use this if you know that `A2` is the same as `A`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_alignment<A2: VecAlignment>(self) -> Vector<N, T, A2> {
	    unsafe { std::mem::transmute_copy::<Vector<N, T, A>, Vector<N, T, A2>>(&self) }
	}
	
	/// Transmutes `self` to `Vector<2, T, VecAligned>`.
	/// 
	/// Only use this if you know that `A` is `VecAligned` and `N` is `2`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec2(self) -> Vector<2, T, VecAligned> {
	    unsafe { std::mem::transmute_copy::<Vector<N, T, A>, Vector<2, T, VecAligned>>(&self) }
	}
	
	/// Transmutes `self` to `Vector<2, T, VecPacked>`.
	/// 
	/// Only use this if you know that `A` is `VecPacked`, and `N` is `2`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec2p(self) -> Vector<2, T, VecPacked> {
	    unsafe { std::mem::transmute_copy::<Vector<N, T, A>, Vector<2, T, VecPacked>>(&self) }
	}
	
	/// Transmutes `self` to `Vector<3, T, VecAligned>`.
	/// 
	/// Only use this if you know that `A` is `VecAligned` and `N` is `3`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec3(self) -> Vector<3, T, VecAligned> {
	    unsafe { std::mem::transmute_copy::<Vector<N, T, A>, Vector<3, T, VecAligned>>(&self) }
	}
	
	/// Transmutes `self` to `Vector<3, T, VecPacked>`.
	/// 
	/// Only use this if you know that `A` is `VecPacked`, and `N` is `3`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec3p(self) -> Vector<3, T, VecPacked> {
	    unsafe { std::mem::transmute_copy::<Vector<N, T, A>, Vector<3, T, VecPacked>>(&self) }
	}
	
	/// Transmutes `self` to `Vector<4, T, VecAligned>`.
	/// 
	/// Only use this if you know that `A` is `VecAligned` and `N` is `4`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec4(self) -> Vector<4, T, VecAligned> {
	    unsafe { std::mem::transmute_copy::<Vector<N, T, A>, Vector<4, T, VecAligned>>(&self) }
	}
	
	/// Transmutes `self` to `Vector<4, T, VecPacked>`.
	/// 
	/// Only use this if you know that `A` is `VecPacked`, and `N` is `4`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec4p(self) -> Vector<4, T, VecPacked> {
	    unsafe { std::mem::transmute_copy::<Vector<N, T, A>, Vector<4, T, VecPacked>>(&self) }
	}
	
	/// Transmutes `self` to a vector of a different length.
	///
	/// Only use this if you know that `N2` is the same as `N`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_len_ref<const N2: usize>(&self) -> &Vector<N2, T, A>
	where
	    Usize<N2>: VecLen,
	{
	    unsafe { std::mem::transmute_copy::<&Vector<N, T, A>, &Vector<N2, T, A>>(&self) }
	}
	
	/// Transmutes `self` to a vector of a different scalar type.
	///
	/// Only use this if you are sure that `T2` has the same size and alignment as `T`,
	/// and that `T` is transmutable to `T2`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_scalar_ref<T2: Scalar>(&self) -> &Vector<N, T2, A> {
	    unsafe { std::mem::transmute_copy::<&Vector<N, T, A>, &Vector<N, T2, A>>(&self) }
	}
	
	/// Transmutes `self` to a vector of a different alignment.
	///
	/// Only use this if you know that `A2` is the same as `A`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_alignment_ref<A2: VecAlignment>(&self) -> &Vector<N, T, A2> {
	    unsafe { std::mem::transmute_copy::<&Vector<N, T, A>, &Vector<N, T, A2>>(&self) }
	}
	
	/// Transmutes `self` to `_refVector<2, T, VecAligned>`.
	/// 
	/// Only use this if you know that `A` is `VecAligned` and `N` is `2`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec2_ref(&self) -> &Vector<2, T, VecAligned> {
	    unsafe { std::mem::transmute_copy::<&Vector<N, T, A>, &Vector<2, T, VecAligned>>(&self) }
	}
	
	/// Transmutes `self` to `_refVector<2, T, VecPacked>`.
	/// 
	/// Only use this if you know that `A` is `VecPacked`, and `N` is `2`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec2p_ref(&self) -> &Vector<2, T, VecPacked> {
	    unsafe { std::mem::transmute_copy::<&Vector<N, T, A>, &Vector<2, T, VecPacked>>(&self) }
	}
	
	/// Transmutes `self` to `_refVector<3, T, VecAligned>`.
	/// 
	/// Only use this if you know that `A` is `VecAligned` and `N` is `3`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec3_ref(&self) -> &Vector<3, T, VecAligned> {
	    unsafe { std::mem::transmute_copy::<&Vector<N, T, A>, &Vector<3, T, VecAligned>>(&self) }
	}
	
	/// Transmutes `self` to `_refVector<3, T, VecPacked>`.
	/// 
	/// Only use this if you know that `A` is `VecPacked`, and `N` is `3`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec3p_ref(&self) -> &Vector<3, T, VecPacked> {
	    unsafe { std::mem::transmute_copy::<&Vector<N, T, A>, &Vector<3, T, VecPacked>>(&self) }
	}
	
	/// Transmutes `self` to `_refVector<4, T, VecAligned>`.
	/// 
	/// Only use this if you know that `A` is `VecAligned` and `N` is `4`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec4_ref(&self) -> &Vector<4, T, VecAligned> {
	    unsafe { std::mem::transmute_copy::<&Vector<N, T, A>, &Vector<4, T, VecAligned>>(&self) }
	}
	
	/// Transmutes `self` to `_refVector<4, T, VecPacked>`.
	/// 
	/// Only use this if you know that `A` is `VecPacked`, and `N` is `4`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec4p_ref(&self) -> &Vector<4, T, VecPacked> {
	    unsafe { std::mem::transmute_copy::<&Vector<N, T, A>, &Vector<4, T, VecPacked>>(&self) }
	}
	
	/// Transmutes `self` to a vector of a different length.
	///
	/// Only use this if you know that `N2` is the same as `N`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_len_mut<const N2: usize>(&mut self) -> &mut Vector<N2, T, A>
	where
	    Usize<N2>: VecLen,
	{
	    unsafe { std::mem::transmute_copy::<&mut Vector<N, T, A>, &mut Vector<N2, T, A>>(&self) }
	}
	
	/// Transmutes `self` to a vector of a different scalar type.
	///
	/// Only use this if you are sure that `T2` has the same size and alignment as `T`,
	/// and that `T` is transmutable to `T2`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_scalar_mut<T2: Scalar>(&mut self) -> &mut Vector<N, T2, A> {
	    unsafe { std::mem::transmute_copy::<&mut Vector<N, T, A>, &mut Vector<N, T2, A>>(&self) }
	}
	
	/// Transmutes `self` to a vector of a different alignment.
	///
	/// Only use this if you know that `A2` is the same as `A`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_alignment_mut<A2: VecAlignment>(&mut self) -> &mut Vector<N, T, A2> {
	    unsafe { std::mem::transmute_copy::<&mut Vector<N, T, A>, &mut Vector<N, T, A2>>(&self) }
	}
	
	/// Transmutes `self` to `_mutVector<2, T, VecAligned>`.
	/// 
	/// Only use this if you know that `A` is `VecAligned` and `N` is `2`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec2_mut(&mut self) -> &mut Vector<2, T, VecAligned> {
	    unsafe { std::mem::transmute_copy::<&mut Vector<N, T, A>, &mut Vector<2, T, VecAligned>>(&self) }
	}
	
	/// Transmutes `self` to `_mutVector<2, T, VecPacked>`.
	/// 
	/// Only use this if you know that `A` is `VecPacked`, and `N` is `2`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec2p_mut(&mut self) -> &mut Vector<2, T, VecPacked> {
	    unsafe { std::mem::transmute_copy::<&mut Vector<N, T, A>, &mut Vector<2, T, VecPacked>>(&self) }
	}
	
	/// Transmutes `self` to `_mutVector<3, T, VecAligned>`.
	/// 
	/// Only use this if you know that `A` is `VecAligned` and `N` is `3`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec3_mut(&mut self) -> &mut Vector<3, T, VecAligned> {
	    unsafe { std::mem::transmute_copy::<&mut Vector<N, T, A>, &mut Vector<3, T, VecAligned>>(&self) }
	}
	
	/// Transmutes `self` to `_mutVector<3, T, VecPacked>`.
	/// 
	/// Only use this if you know that `A` is `VecPacked`, and `N` is `3`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec3p_mut(&mut self) -> &mut Vector<3, T, VecPacked> {
	    unsafe { std::mem::transmute_copy::<&mut Vector<N, T, A>, &mut Vector<3, T, VecPacked>>(&self) }
	}
	
	/// Transmutes `self` to `_mutVector<4, T, VecAligned>`.
	/// 
	/// Only use this if you know that `A` is `VecAligned` and `N` is `4`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec4_mut(&mut self) -> &mut Vector<4, T, VecAligned> {
	    unsafe { std::mem::transmute_copy::<&mut Vector<N, T, A>, &mut Vector<4, T, VecAligned>>(&self) }
	}
	
	/// Transmutes `self` to `_mutVector<4, T, VecPacked>`.
	/// 
	/// Only use this if you know that `A` is `VecPacked`, and `N` is `4`.
	/// If this is not the case, it is undefined behavior.
	#[inline(always)]
	pub const unsafe fn transmute_vec4p_mut(&mut self) -> &mut Vector<4, T, VecPacked> {
	    unsafe { std::mem::transmute_copy::<&mut Vector<N, T, A>, &mut Vector<4, T, VecPacked>>(&self) }
	}
	
}
