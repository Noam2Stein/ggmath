use std::any::TypeId;

use crate::{
    Usize, VecAligned,
    vector::{Scalar, VecAlignment, VecLen, VecLenEnum, Vector},
};

impl Scalar for bool {
    type InnerAlignedVec2 = glam::BVec2;
    type InnerAlignedVec3 = glam::BVec3A;
    type InnerAlignedVec4 = glam::BVec4A;

    const GARBAGE: Self = false;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = glam::BVec2::FALSE;
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = glam::BVec3A::FALSE;
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = glam::BVec4A::FALSE;

    #[inline(always)]
    fn vec_not<const N: usize, A: VecAlignment>(
        vector: Vector<N, Self, A>,
    ) -> Vector<N, <Self as std::ops::Not>::Output, A>
    where
        Usize<N>: VecLen,
        Self: std::ops::Not<Output: Scalar>,
    {
        if vector.is_aligned() && N == 2 {
            match N {
                2 => {
                    return unsafe {
                        let vector = vector.transmute_vec2();

                        let output = Vector::<2, bool, VecAligned>::from_inner(!vector.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                3 => {
                    return unsafe {
                        let vector = vector.transmute_vec3();

                        let output = Vector::<3, bool, VecAligned>::from_inner(!vector.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                4 => {
                    return unsafe {
                        let vector = vector.transmute_vec4();

                        let output = Vector::<4, bool, VecAligned>::from_inner(!vector.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }

                _ => {}
            }
        }

        vector.map(|b| !b)
    }

    #[inline(always)]
    fn vec_bitand<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as std::ops::BitAnd<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: std::ops::BitAnd<T2, Output: Scalar>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<bool>() {
            match N {
                2 => {
                    return unsafe {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<bool>();

                        let output =
                            Vector::<2, bool, VecAligned>::from_inner(vector.inner & other.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                3 => {
                    return unsafe {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<bool>();

                        let output =
                            Vector::<3, bool, VecAligned>::from_inner(vector.inner & other.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                4 => {
                    return unsafe {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<bool>();

                        let output =
                            Vector::<4, bool, VecAligned>::from_inner(vector.inner & other.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                _ => {}
            }
        }

        Vector::from_fn(|i| vector[i] & other[i])
    }

    #[inline(always)]
    fn vec_bitor<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as std::ops::BitOr<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: std::ops::BitOr<T2, Output: Scalar>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<bool>() {
            match N {
                2 => {
                    return unsafe {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<bool>();

                        let output =
                            Vector::<2, bool, VecAligned>::from_inner(vector.inner | other.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                3 => {
                    return unsafe {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<bool>();

                        let output =
                            Vector::<3, bool, VecAligned>::from_inner(vector.inner | other.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                4 => {
                    return unsafe {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<bool>();

                        let output =
                            Vector::<4, bool, VecAligned>::from_inner(vector.inner | other.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                _ => {}
            }
        }

        Vector::from_fn(|i| vector[i] | other[i])
    }

    #[inline(always)]
    fn vec_bitxor<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, <Self as std::ops::BitXor<T2>>::Output, A>
    where
        Usize<N>: VecLen,
        Self: std::ops::BitXor<T2, Output: Scalar>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<bool>() {
            match N {
                2 => {
                    return unsafe {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<bool>();

                        let output =
                            Vector::<2, bool, VecAligned>::from_inner(vector.inner ^ other.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                3 => {
                    return unsafe {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<bool>();

                        let output =
                            Vector::<3, bool, VecAligned>::from_inner(vector.inner ^ other.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                4 => {
                    return unsafe {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<bool>();

                        let output =
                            Vector::<4, bool, VecAligned>::from_inner(vector.inner ^ other.inner);

                        output
                            .transmute_len()
                            .transmute_scalar()
                            .transmute_alignment()
                    };
                }
                _ => {}
            }
        }

        Vector::from_fn(|i| vector[i] ^ other[i])
    }

    #[inline(always)]
    fn vec_bitand_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: std::ops::BitAndAssign<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<bool>() {
            match N {
                2 => unsafe {
                    let vector = vector.transmute_vec2_mut();
                    let other = other.transmute_vec2().transmute_scalar::<bool>();

                    vector.inner &= other.inner;
                },
                3 => unsafe {
                    let vector = vector.transmute_vec3_mut();
                    let other = other.transmute_vec3().transmute_scalar::<bool>();

                    vector.inner &= other.inner;
                },
                4 => {
                    unsafe {
                        let vector = vector.transmute_vec4_mut();
                        let other = other.transmute_vec4().transmute_scalar::<bool>();

                        vector.inner &= other.inner;
                    };
                }
                _ => {}
            }
        }

        for i in 0..N {
            vector[i] &= other[i];
        }
    }

    #[inline(always)]
    fn vec_bitor_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: std::ops::BitOrAssign<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<bool>() {
            match N {
                2 => unsafe {
                    let vector = vector.transmute_vec2_mut();
                    let other = other.transmute_vec2().transmute_scalar::<bool>();

                    vector.inner |= other.inner;
                },
                3 => unsafe {
                    let vector = vector.transmute_vec3_mut();
                    let other = other.transmute_vec3().transmute_scalar::<bool>();

                    vector.inner |= other.inner;
                },
                4 => unsafe {
                    let vector = vector.transmute_vec4_mut();
                    let other = other.transmute_vec4().transmute_scalar::<bool>();

                    vector.inner |= other.inner;
                },
                _ => {}
            }
        }

        for i in 0..N {
            vector[i] |= other[i];
        }
    }

    #[inline(always)]
    fn vec_bitxor_assign<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl VecAlignment>,
    ) where
        Usize<N>: VecLen,
        Self: std::ops::BitXorAssign<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<bool>() {
            match N {
                2 => unsafe {
                    let vector = vector.transmute_vec2_mut();
                    let other = other.transmute_vec2().transmute_scalar::<bool>();

                    vector.inner ^= other.inner;
                },
                3 => unsafe {
                    let vector = vector.transmute_vec3_mut();
                    let other = other.transmute_vec3().transmute_scalar::<bool>();

                    vector.inner ^= other.inner;
                },
                4 => unsafe {
                    let vector = vector.transmute_vec4_mut();
                    let other = other.transmute_vec4().transmute_scalar::<bool>();

                    vector.inner ^= other.inner;
                },
                _ => {}
            }
        }

        for i in 0..N {
            vector[i] ^= other[i];
        }
    }

    #[inline(always)]
    fn vec_eq<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> bool
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<bool>() {
            match N {
                2 => {
                    return unsafe {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<bool>();

                        vector.inner == other.inner
                    };
                }
                3 => {
                    return unsafe {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<bool>();

                        vector.inner == other.inner
                    };
                }
                4 => {
                    return unsafe {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<bool>();

                        vector.inner == other.inner
                    };
                }
                _ => {}
            }
        }

        vector.eq_mask(other).all_true()
    }

    fn vec_ne<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> bool
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<bool>() {
            match N {
                2 => {
                    return unsafe {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<bool>();

                        vector.inner != other.inner
                    };
                }
                3 => {
                    return unsafe {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<bool>();

                        vector.inner != other.inner
                    };
                }
                4 => {
                    return unsafe {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<bool>();

                        vector.inner != other.inner
                    };
                }
                _ => {}
            }
        }

        !vector.eq(other)
    }

    fn vec_eq_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<bool>() {
            return unsafe {
                let other = other.transmute_scalar_ref::<bool>();

                !(vector ^ other)
            };
        }

        Vector::from_fn(|i| vector[i] == other[i])
    }

    fn vec_ne_mask<const N: usize, A: VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        Usize<N>: VecLen,
        Self: PartialEq<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<bool>() {
            return unsafe {
                let other = other.transmute_scalar_ref::<bool>();

                vector ^ other
            };
        }

        Vector::from_fn(|i| vector[i] != other[i])
    }
}

impl<const N: usize, A: VecAlignment> Vector<N, bool, A>
where
    Usize<N>: VecLen,
{
    /// Returns true if all components of the vector are true.
    #[inline(always)]
    pub fn all_true(self) -> bool {
        unsafe {
            match (Usize::<N>::ENUM, A::IS_ALIGNED) {
                (VecLenEnum::Two, true) => self.transmute_vec2().inner.all(),
                (VecLenEnum::Three, true) => self.transmute_vec3().inner.all(),
                (VecLenEnum::Four, true) => self.transmute_vec4().inner.all(),

                (_, false) => self.all(|b| b),
            }
        }
    }

    /// Returns true if all components of the vector are false.
    #[inline(always)]
    pub fn all_false(self) -> bool {
        unsafe {
            match (Usize::<N>::ENUM, A::IS_ALIGNED) {
                (VecLenEnum::Two, true) => !self.transmute_vec2().inner.any(),
                (VecLenEnum::Three, true) => !self.transmute_vec3().inner.any(),
                (VecLenEnum::Four, true) => !self.transmute_vec4().inner.any(),

                (_, false) => self.all(|b| !b),
            }
        }
    }

    /// Returns true if any component of the vector is true.
    #[inline(always)]
    pub fn any_true(self) -> bool {
        unsafe {
            match (Usize::<N>::ENUM, A::IS_ALIGNED) {
                (VecLenEnum::Two, true) => self.transmute_vec2().inner.any(),
                (VecLenEnum::Three, true) => self.transmute_vec3().inner.any(),
                (VecLenEnum::Four, true) => self.transmute_vec4().inner.any(),

                (_, false) => self.any(|b| b),
            }
        }
    }

    /// Returns true if any component of the vector is false.
    #[inline(always)]
    pub fn any_false(self) -> bool {
        unsafe {
            match (Usize::<N>::ENUM, A::IS_ALIGNED) {
                (VecLenEnum::Two, true) => !self.transmute_vec2().inner.all(),
                (VecLenEnum::Three, true) => !self.transmute_vec3().inner.all(),
                (VecLenEnum::Four, true) => !self.transmute_vec4().inner.all(),

                (_, false) => self.any(|b| !b),
            }
        }
    }

    /// Returns the number of true components in the vector.
    #[inline(always)]
    pub fn count_true(self) -> usize {
        self.count(|b| b)
    }

    /// Returns the number of false components in the vector.
    #[inline(always)]
    pub fn count_false(self) -> usize {
        self.count(|b| !b)
    }
}

impl<const N: usize, A: VecAlignment> Vector<N, bool, A>
where
    Usize<N>: VecLen,
{
    /// Performs `self.all_true()` using a slower implementation that supports const contexts.
    #[inline(always)]
    pub const fn const_all_true(self) -> bool {
        let mut i = 0;
        while i < N {
            if !self.as_array()[i] {
                return false;
            }
            i += 1;
        }

        true
    }

    /// Performs `self.all_false()` using a slower implementation that supports const contexts.
    #[inline(always)]
    pub const fn const_all_false(self) -> bool {
        let mut i = 0;
        while i < N {
            if self.as_array()[i] {
                return false;
            }
            i += 1;
        }

        true
    }

    /// Performs `self.any_true()` using a slower implementation that supports const contexts.
    #[inline(always)]
    pub const fn const_any_true(self) -> bool {
        let mut i = 0;
        while i < N {
            if self.as_array()[i] {
                return true;
            }
            i += 1;
        }

        false
    }

    /// Performs `self.any_false()` using a slower implementation that supports const contexts.
    #[inline(always)]
    pub const fn const_any_false(self) -> bool {
        let mut i = 0;
        while i < N {
            if !self.as_array()[i] {
                return true;
            }
            i += 1;
        }

        false
    }

    /// Performs `self.count_true()` using a slower implementation that supports const contexts.
    #[inline(always)]
    pub const fn const_count_true(self) -> usize {
        let mut i = 0;
        let mut count = 0;
        while i < N {
            if self.as_array()[i] {
                count += 1;
            }
            i += 1;
        }

        count
    }

    /// Performs `self.count_false()` using a slower implementation that supports const contexts.
    #[inline(always)]
    pub const fn const_count_false(self) -> usize {
        let mut i = 0;
        let mut count = 0;
        while i < N {
            if !self.as_array()[i] {
                count += 1;
            }
            i += 1;
        }

        count
    }
}
