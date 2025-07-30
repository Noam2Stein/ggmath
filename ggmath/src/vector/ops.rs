use std::{any::TypeId, mem::transmute_copy, ops::*};

use super::*;

// Note:
// Alot of operator traits cannot be derived because they have a generic `Rhs` type.
// If we were to derive them, the implementation would only support `Rhs = Self`.

// Comparison

impl<const N: usize, T: Scalar + PartialEq<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
    PartialEq<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Vector<N, TRhs, ARhs>) -> bool {
        self.array
            .iter()
            .zip(other.array.iter())
            .all(|(a, b)| *a == *b)
    }
}

macro_loop! {
    // Unary

    @for [op_trait, op_fn] in [
        [Neg, neg],
        [Not, not],
    ] {
        impl<const N: usize, T: Scalar + @op_trait<Output: Scalar>, A: VecAlignment> @op_trait
            for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            type Output = Vector<N, T::Output, A>;

            #[inline(always)]
            fn @op_fn(self) -> Vector<N, <T as @op_trait>::Output, A> {
                self.map(T::@op_fn)
            }
        }
    }

    // Binary, Assign

    @for [op_trait, op_fn, garbage_fn] in [
        [Add, add, ADD_GARBAGE],
        [Sub, sub, SUB_GARBAGE],
        [Mul, mul, MUL_GARBAGE],
        [Div, div, DIV_GARBAGE],
        [Rem, rem, REM_GARBAGE],
        [BitAnd, bitand, BITAND_GARBAGE],
        [BitOr, bitor, BITOR_GARBAGE],
        [BitXor, bitxor, BITXOR_GARBAGE],
        [Shl, shl, SHL_GARBAGE],
        [Shr, shr, SHR_GARBAGE],
    ] {
        // Bin Ops

        impl<
            const N: usize,
            T: Scalar + @op_trait<TRhs, Output: Scalar>,
            A: VecAlignment,
            TRhs: Scalar,
            ARhs: VecAlignment,
        > @op_trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            type Output = Vector<N, T::Output, A>;

            #[inline(always)]
            fn @op_fn(self, rhs: Vector<N, TRhs, ARhs>) -> Vector<N, <T as @op_trait<TRhs>>::Output, A> {
                // This function has alot of conditions over constant values.
                // Those are removed at compile time by the compiler,
                // and help it select the optimal way to perform the operator.

                'simd_optimization: {
                    let garbage_fn = match T::@garbage_fn {
                        Some(garbage_fn) => garbage_fn,
                        None => break 'simd_optimization,
                    };

                    let is_all_self = types_match::<TRhs, T>() && types_match::<T::Output, T>();

                    let is_vec4_aligned = size_align_matches::<Self, Vec4<T>>();
                    let rhs_is_vec4_aligned = size_align_matches::<Vector<N, TRhs, ARhs>, Vec4<TRhs>>();

                    if !is_all_self
                        || !is_vec4_aligned
                        || !rhs_is_vec4_aligned
                    {
                        break 'simd_optimization;
                    };

                    let self_vec4 = unsafe { transmute_copy::<Self, Vec4<T>>(&self) };
                    let rhs_vec4 = unsafe { transmute_copy::<Vector<N, TRhs, ARhs>, Vec4<TRhs>>(&rhs) };
                    let rhs_t_vec4 = unsafe { transmute_copy::<Vector<N, TRhs, ARhs>, Vec4<T>>(&rhs) };

                    let output_vec4 = unsafe {
                        Vec4::from_fn(|i| if i < N {
                            T::@op_fn(self_vec4[i], rhs_vec4[i])
                        } else {
                            let garbage_output = garbage_fn(self_vec4[i], rhs_t_vec4[i]);

                            transmute_copy::<T, T::Output>(&garbage_output)
                        })
                    };

                    return unsafe {
                        transmute_copy::<Vec4<T::Output>, Vector<N, T::Output, A>>(&output_vec4)
                    };
                }

                self.map_rhs(rhs, |self_, rhs|T::@op_fn(self_, rhs))
            }
        }

        // Assign Ops

        @let op_assign_trait = @op_trait + Assign;
        @let op_assign_fn = @op_fn + _assign;

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
        [Mul, mul],
        [Div, div],
        [Rem, rem],
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

        @let op_assign_trait = @op_trait + Assign;
        @let op_assign_fn = @op_fn + _assign;

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
        self.map_rhs(other, |a, b| a * b).sum()
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

fn size_align_matches<T1, T2>() -> bool {
    size_of::<T1>() == size_of::<T2>() && align_of::<T1>() == align_of::<T2>()
}

fn types_match<T1: 'static, T2: 'static>() -> bool {
    TypeId::of::<T1>() == TypeId::of::<T2>()
}
