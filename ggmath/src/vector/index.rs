use std::{
    mem::transmute,
    ops::{Index, IndexMut},
};

use super::*;

// Get

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns the component at the given index, or `None` if the index is out of bounds.
    #[inline(always)]
    pub const fn get(self, idx: usize) -> Option<T> {
        if idx < N {
            Some(unsafe { self.get_unchecked(idx) })
        } else {
            None
        }
    }

    /// Returns the component at the given index, or panics if the index is out of bounds.
    ///
    /// This only exists because you can't use the index operator in const contexts.
    /// This emulates it without the verbose `.get().unwrap()`.
    ///
    /// When const traits are stable, this will be removed.
    #[inline(always)]
    pub const fn index(self, idx: usize) -> T {
        self.get(idx).unwrap()
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Index<usize> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.as_array_ref().index(index)
    }
}

// Get Unchecked

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns the component at the given index, without bounds checking.
    #[inline(always)]
    pub const unsafe fn get_unchecked(self, idx: usize) -> T {
        unsafe { *self.get_ref_unchecked(idx) }
    }
}

// Get Ref

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns a reference to the component at the given index, or `None` if the index is out of bounds.
    #[inline(always)]
    pub const fn get_ref(&self, idx: usize) -> Option<&T> {
        if idx < N {
            Some(unsafe { self.get_ref_unchecked(idx) })
        } else {
            None
        }
    }

    /// Returns a reference to the component at the given index, or panics if the index is out of bounds.
    ///
    /// This only exists because you can't use the index operator in const contexts.
    /// This emulates it without the verbose `.get_ref().unwrap()`.
    ///
    /// When const traits are stable, this will be removed.
    #[inline(always)]
    pub const fn index_ref(&self, idx: usize) -> &T {
        self.get_ref(idx).unwrap()
    }

    /// Returns a reference to the 2 contiguous components starting at the given index, or `None` if the index is out of bounds.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const fn get_2_ref(&self, idx: usize) -> Option<&Vec2P<T>> {
        self.get_vec_ref::<2>(idx)
    }

    /// Returns a reference to the 3 contiguous components starting at the given index, or `None` if the index is out of bounds.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const fn get_3_ref(&self, idx: usize) -> Option<&Vec3P<T>> {
        self.get_vec_ref::<3>(idx)
    }

    /// Returns a reference to the 4 contiguous components starting at the given index, or `None` if the index is out of bounds.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const fn get_4_ref(&self, idx: usize) -> Option<&Vec4P<T>> {
        self.get_vec_ref::<4>(idx)
    }

    /// Returns a reference to the sub-vector starting at the given index, or `None` if the index is out of bounds.
    ///
    /// This is a generic version of the `get_2_ref`, `get_3_ref`, and `get_4_ref` methods.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const fn get_vec_ref<const N_OUT: usize>(
        &self,
        idx: usize,
    ) -> Option<&Vector<N_OUT, T, VecPacked>>
    where
        Usize<N_OUT>: VecLen,
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
    Usize<N>: VecLen,
{
    /// Returns a reference to the component at the given index, without bounds checking.
    #[inline(always)]
    pub const unsafe fn get_ref_unchecked(&self, idx: usize) -> &T {
        unsafe { self.as_ptr().add(idx).as_ref().unwrap_unchecked() }
    }

    /// Returns a reference to the 2 contiguous components starting at the given index, without bounds checking.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const unsafe fn get_2_ref_unchecked(&self, idx: usize) -> &Vec2P<T> {
        unsafe { self.get_vec_ref_unchecked::<2>(idx) }
    }

    /// Returns a reference to the 3 contiguous components starting at the given index, without bounds checking.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const unsafe fn get_3_ref_unchecked(&self, idx: usize) -> &Vec3P<T> {
        unsafe { self.get_vec_ref_unchecked::<3>(idx) }
    }

    /// Returns a reference to the 4 contiguous components starting at the given index, without bounds checking.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const unsafe fn get_4_ref_unchecked(&self, idx: usize) -> &Vec4P<T> {
        unsafe { self.get_vec_ref_unchecked::<4>(idx) }
    }

    /// Returns a reference to the sub-vector starting at the given index, without bounds checking.
    ///
    /// This is a generic version of the `get_2_ref_unchecked`, `get_3_ref_unchecked`, and `get_4_ref_unchecked` methods.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const unsafe fn get_vec_ref_unchecked<const N_OUT: usize>(
        &self,
        idx: usize,
    ) -> &Vector<N_OUT, T, VecPacked>
    where
        Usize<N_OUT>: VecLen,
    {
        unsafe { transmute(self.get_ref_unchecked(idx)) }
    }
}

// Get Mut

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns a mutable reference to the component at the given index, or `None` if the index is out of bounds.
    #[inline(always)]
    pub const fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx < N {
            Some(unsafe { self.get_mut_unchecked(idx) })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the component at the given index, or panics if the index is out of bounds.
    ///
    /// This only exists because you can't use the index operator in const contexts.
    /// This emulates it without the verbose `.get_mut().unwrap()`.
    ///
    /// When const traits are stable, this will be removed.
    #[inline(always)]
    pub const fn index_mut(&mut self, idx: usize) -> &mut T {
        self.get_mut(idx).unwrap()
    }

    /// Returns a mutable reference to the 2 contiguous components starting at the given index, or `None` if the index is out of bounds.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const fn get_2_mut(&mut self, idx: usize) -> Option<&mut Vec2P<T>> {
        self.get_vec_mut::<2>(idx)
    }

    /// Returns a mutable reference to the 3 contiguous components starting at the given index, or `None` if the index is out of bounds.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const fn get_3_mut(&mut self, idx: usize) -> Option<&mut Vec3P<T>> {
        self.get_vec_mut::<3>(idx)
    }

    /// Returns a mutable reference to the 4 contiguous components starting at the given index, or `None` if the index is out of bounds.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const fn get_4_mut(&mut self, idx: usize) -> Option<&mut Vec4P<T>> {
        self.get_vec_mut::<4>(idx)
    }

    /// Returns a mutable reference to the sub-vector starting at the given index, or `None` if the index is out of bounds.
    ///
    /// This is a generic version of the `get_2_mut`, `get_3_mut`, and `get_4_mut` methods.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const fn get_vec_mut<const N_OUT: usize>(
        &mut self,
        idx: usize,
    ) -> Option<&mut Vector<N_OUT, T, VecPacked>>
    where
        Usize<N_OUT>: VecLen,
    {
        if idx + N_OUT <= N {
            Some(unsafe { self.get_vec_mut_unchecked(idx) })
        } else {
            None
        }
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> IndexMut<usize> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_array_mut().index_mut(index)
    }
}

// Get Mut Unchecked

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns a mutable reference to the component at the given index, without bounds checking.
    #[inline(always)]
    pub const unsafe fn get_mut_unchecked(&mut self, idx: usize) -> &mut T {
        unsafe { self.as_mut_ptr().add(idx).as_mut().unwrap_unchecked() }
    }

    /// Returns a mutable reference to the 2 contiguous components starting at the given index, without bounds checking.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const unsafe fn get_2_mut_unchecked(&mut self, idx: usize) -> &mut Vec2P<T> {
        unsafe { self.get_vec_mut_unchecked::<2>(idx) }
    }

    /// Returns a mutable reference to the 3 contiguous components starting at the given index, without bounds checking.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const unsafe fn get_3_mut_unchecked(&mut self, idx: usize) -> &mut Vec3P<T> {
        unsafe { self.get_vec_mut_unchecked::<3>(idx) }
    }

    /// Returns a mutable reference to the 4 contiguous components starting at the given index, without bounds checking.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const unsafe fn get_4_mut_unchecked(&mut self, idx: usize) -> &mut Vec4P<T> {
        unsafe { self.get_vec_mut_unchecked::<4>(idx) }
    }

    /// Returns a mutable reference to the sub-vector starting at the given index, without bounds checking.
    ///
    /// This is a generic version of the `get_2_mut_unchecked`, `get_3_mut_unchecked`, and `get_4_mut_unchecked` methods.
    ///
    /// This returns a packed vector because an aligned vector has padding,
    /// which the returned reference doesn't have.
    #[inline(always)]
    pub const unsafe fn get_vec_mut_unchecked<const N_OUT: usize>(
        &mut self,
        idx: usize,
    ) -> &mut Vector<N_OUT, T, VecPacked>
    where
        Usize<N_OUT>: VecLen,
    {
        unsafe { transmute(self.get_mut_unchecked(idx)) }
    }
}
