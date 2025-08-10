use std::ops::*;

use super::*;

// Comparison

impl<const N: usize, T: Scalar, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    PartialEq<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
    T: PartialEq<T2>,
{
    #[inline(always)]
    fn eq(&self, other: &Vector<N, T2, A2>) -> bool {
        T::vec_eq(self, other)
    }

    #[inline(always)]
    fn ne(&self, other: &Vector<N, T2, A2>) -> bool {
        T::vec_ne(self, other)
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns a vector of booleans where each element is true if the corresponding elements in the two vectors are equal.
    #[inline(always)]
    pub fn eq_mask<T2: Scalar, A2: VecAlignment>(
        &self,
        other: &Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialEq<T2>,
    {
        T::vec_eq_mask(self, other)
    }

    /// Returns a vector of booleans where each element is true if the corresponding elements in the two vectors are not equal.
    #[inline(always)]
    pub fn ne_mask<T2: Scalar, A2: VecAlignment>(
        &self,
        other: &Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialEq<T2>,
    {
        T::vec_ne_mask(self, other)
    }

    /// Returns a vector of booleans where each element is true if the corresponding element in the vector is less than the other vector.
    #[inline(always)]
    pub fn lt_mask<T2: Scalar, A2: VecAlignment>(
        &self,
        other: &Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_lt_mask(self, other)
    }

    /// Returns a vector of booleans where each element is true if the corresponding element in the vector is greater than the other vector.
    #[inline(always)]
    pub fn gt_mask<T2: Scalar, A2: VecAlignment>(
        &self,
        other: &Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_gt_mask(self, other)
    }

    /// Returns a vector of booleans where each element is true if the corresponding element in the vector is less than or equal to the other vector.
    #[inline(always)]
    pub fn le_mask<T2: Scalar, A2: VecAlignment>(
        &self,
        other: &Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_le_mask(self, other)
    }

    /// Returns a vector of booleans where each element is true if the corresponding element in the vector is greater than or equal to the other vector.
    #[inline(always)]
    pub fn ge_mask<T2: Scalar, A2: VecAlignment>(
        &self,
        other: &Vector<N, T2, A2>,
    ) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_ge_mask(self, other)
    }
}

repetitive! {
    // Unary

    @for [op_trait, op_fn] in [
        ['Neg, 'neg],
        ['Not, 'not],
    ] {
        impl<const N: usize, T: Scalar + @op_trait<Output: Scalar>, A: VecAlignment> @op_trait
            for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            type Output = Vector<N, T::Output, A>;

            #[inline(always)]
            fn @op_fn(self) -> Vector<N, <T as @op_trait>::Output, A> {
                T::@['vec_ op_fn](self)
            }
        }
    }

    // Binary, Assign

    @for [op_trait, op_fn, garbage_fn] in [
        ['Add, 'add, 'ADD_GARBAGE],
        ['Sub, 'sub, 'SUB_GARBAGE],
        ['Mul, 'mul, 'MUL_GARBAGE],
        ['Div, 'div, 'DIV_GARBAGE],
        ['Rem, 'rem, 'REM_GARBAGE],
        ['BitAnd, 'bitand, 'BITAND_GARBAGE],
        ['BitOr, 'bitor, 'BITOR_GARBAGE],
        ['BitXor, 'bitxor, 'BITXOR_GARBAGE],
        ['Shl, 'shl, 'SHL_GARBAGE],
        ['Shr, 'shr, 'SHR_GARBAGE],
    ] {
        // Bin Ops

        impl<
            const N: usize,
            T: Scalar + @op_trait<T2, Output: Scalar>,
            A: VecAlignment,
            T2: Scalar,
            A2: VecAlignment,
        > @op_trait<Vector<N, T2, A2>> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            type Output = Vector<N, T::Output, A>;

            #[inline(always)]
            fn @op_fn(self, rhs: Vector<N, T2, A2>) -> Vector<N, <T as @op_trait<T2>>::Output, A> {
                T::@['vec_ op_fn](self, rhs)
            }
        }

        // Assign Ops

        @let op_assign_trait = @[op_trait 'Assign];
        @let op_assign_fn = @[op_fn '_assign];

        impl<const N: usize, T: Scalar + @op_assign_trait<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
        @op_assign_trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn @op_assign_fn(&mut self, rhs: Vector<N, TRhs, ARhs>) {
                for i in 0..N {
                    self[i].@op_assign_fn(rhs[i]);
                }
            }
        }
    }

    // Scalar Ops

    @for [op_trait, op_fn] in [
        ['Mul, 'mul],
        ['Div, 'div],
        ['Rem, 'rem],
    ] {
        impl<
            const N: usize,
            T: Scalar + @op_trait<Rhs, Output: Scalar>,
            A: VecAlignment,
            Rhs: Scalar,
        > @op_trait<Rhs> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            type Output = <Self as @op_trait<Vector<N, Rhs, A>>>::Output;

            #[inline(always)]
            fn @op_fn(self, rhs: Rhs) -> Self::Output {
                self.@op_fn(Vector::<N, Rhs, A>::splat(rhs))
            }
        }

        @let op_assign_trait = @[op_trait 'Assign];
        @let op_assign_fn = @[op_fn '_assign];

        impl<const N: usize, T: Scalar + @op_assign_trait<Rhs>, A: VecAlignment, Rhs: Scalar>
            @op_assign_trait<Rhs> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn @op_assign_fn(&mut self, rhs: Rhs) {
                self.@op_assign_fn(Vector::<N, Rhs, A>::splat(rhs))
            }
        }
    }
}

// Operator Dependent Functions

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns the sum of all components.
    ///
    /// This uses the `Add` trait to add up the components.
    #[inline(always)]
    pub fn sum(self) -> T
    where
        T: Add<Output = T>,
    {
        self.fold(|a, b| a + b)
    }

    /// Returns the dot product of two vectors.
    ///
    /// This uses the precise trait bounds of `Add` and `Mul` traits to calculate the dot product.
    #[inline(always)]
    pub fn dot(self, other: Vector<N, T, impl VecAlignment>) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        (self * other).sum()
    }
}

impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns the cross product of two vectors.
    ///
    /// This uses the precise trait bounds of `Mul` and `Sub` traits to calculate the cross product.
    #[inline(always)]
    pub fn cross(self, other: Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T>,
    {
        (self.zxy() * other - self * other.zxy()).zxy()
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns the vector with the padding set to the given value, if there is padding.
    ///
    /// This is not useful for most scenarios because the padding value is usually garbage and is not meant to be read.
    /// This is used to test edge cases for `Vec3` SIMD operators that could break upon padding overflow.
    #[inline(always)]
    pub const fn with_padding(mut self, padding: T) -> Self {
        let mut i = N;
        while i < size_of::<Self>() / size_of::<T>() {
            unsafe {
                *self.as_mut_ptr().add(i).as_mut().unwrap_unchecked() = padding;
            };

            i += 1;
        }

        self
    }
}
