use std::mem::transmute;

use super::*;

// Get

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    pub const fn get(self, idx: usize) -> Option<T> {
        if idx < N {
            Some(unsafe { self.get_unchecked(idx) })
        } else {
            None
        }
    }
}

// Get Unchecked

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    pub const unsafe fn get_unchecked(self, idx: usize) -> T {
        unsafe { *self.get_ref_unchecked(idx) }
    }
}

// Get Ref

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    pub const fn get_ref(&self, idx: usize) -> Option<&T> {
        if idx < N {
            Some(unsafe { self.get_ref_unchecked(idx) })
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn get_2_ref(&self, idx: usize) -> Option<&Vec2P<T>> {
        self.get_vec_ref::<2>(idx)
    }

    #[inline(always)]
    pub const fn get_3_ref(&self, idx: usize) -> Option<&Vec3P<T>> {
        self.get_vec_ref::<3>(idx)
    }

    #[inline(always)]
    pub const fn get_4_ref(&self, idx: usize) -> Option<&Vec4P<T>> {
        self.get_vec_ref::<4>(idx)
    }

    #[inline(always)]
    pub const fn get_vec_ref<const N_OUT: usize>(
        &self,
        idx: usize,
    ) -> Option<&Vector<N_OUT, T, VecPacked>>
    where
        MaybeVecLen<N_OUT>: VecLen,
    {
        if idx + N_OUT <= N {
            Some(unsafe { self.get_vec_ref_unchecked(idx) })
        } else {
            None
        }
    }
}

// Get Ref Unchecked

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    pub const unsafe fn get_ref_unchecked(&self, idx: usize) -> &T {
        unsafe { self.as_ptr().add(idx).as_ref().unwrap_unchecked() }
    }

    #[inline(always)]
    pub const unsafe fn get_2_ref_unchecked(&self, idx: usize) -> &Vec2P<T> {
        unsafe { self.get_vec_ref_unchecked::<2>(idx) }
    }

    #[inline(always)]
    pub const unsafe fn get_3_ref_unchecked(&self, idx: usize) -> &Vec3P<T> {
        unsafe { self.get_vec_ref_unchecked::<3>(idx) }
    }

    #[inline(always)]
    pub const unsafe fn get_4_ref_unchecked(&self, idx: usize) -> &Vec4P<T> {
        unsafe { self.get_vec_ref_unchecked::<4>(idx) }
    }

    #[inline(always)]
    pub const unsafe fn get_vec_ref_unchecked<const N_OUT: usize>(
        &self,
        idx: usize,
    ) -> &Vector<N_OUT, T, VecPacked>
    where
        MaybeVecLen<N_OUT>: VecLen,
    {
        unsafe { transmute(self.get_ref_unchecked(idx)) }
    }
}

// Get Mut

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.array.get_mut(idx)
    }

    #[inline(always)]
    pub fn get_2_mut(&mut self, idx: usize) -> Option<&mut Vec2P<T>> {
        self.get_vec_mut::<2>(idx)
    }

    #[inline(always)]
    pub fn get_3_mut(&mut self, idx: usize) -> Option<&mut Vec3P<T>> {
        self.get_vec_mut::<3>(idx)
    }

    #[inline(always)]
    pub fn get_4_mut(&mut self, idx: usize) -> Option<&mut Vec4P<T>> {
        self.get_vec_mut::<4>(idx)
    }

    #[inline(always)]
    pub fn get_vec_mut<const N_OUT: usize>(
        &mut self,
        idx: usize,
    ) -> Option<&mut Vector<N_OUT, T, VecPacked>>
    where
        MaybeVecLen<N_OUT>: VecLen,
    {
        if idx + N_OUT <= N {
            Some(unsafe { self.get_vec_mut_unchecked(idx) })
        } else {
            None
        }
    }
}

// Get Mut Unchecked

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    pub unsafe fn get_mut_unchecked(&mut self, idx: usize) -> &mut T {
        unsafe { self.array.get_unchecked_mut(idx) }
    }

    #[inline(always)]
    pub unsafe fn get_2_mut_unchecked(&mut self, idx: usize) -> &mut Vec2P<T> {
        unsafe { self.get_vec_mut_unchecked::<2>(idx) }
    }

    #[inline(always)]
    pub unsafe fn get_3_mut_unchecked(&mut self, idx: usize) -> &mut Vec3P<T> {
        unsafe { self.get_vec_mut_unchecked::<3>(idx) }
    }

    #[inline(always)]
    pub unsafe fn get_4_mut_unchecked(&mut self, idx: usize) -> &mut Vec4P<T> {
        unsafe { self.get_vec_mut_unchecked::<4>(idx) }
    }

    #[inline(always)]
    pub unsafe fn get_vec_mut_unchecked<const N_OUT: usize>(
        &mut self,
        idx: usize,
    ) -> &mut Vector<N_OUT, T, VecPacked>
    where
        MaybeVecLen<N_OUT>: VecLen,
    {
        unsafe { transmute(self.get_mut_unchecked(idx)) }
    }
}
