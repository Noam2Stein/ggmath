use std::any::TypeId;

use crate::{VecAligned, Vector, vector::Scalar};

impl Scalar for f64 {
    type InnerAlignedVec2 = glam::DVec2;
    type InnerAlignedVec3 = glam::DVec3;
    type InnerAlignedVec4 = glam::DVec4;

    const GARBAGE: Self = 0.0;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = glam::DVec2::ZERO;
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = glam::DVec3::ZERO;
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = glam::DVec4::ZERO;

    fn vec_neg<const N: usize, A: crate::VecAlignment>(
        vector: crate::Vector<N, Self, A>,
    ) -> crate::Vector<N, <Self as std::ops::Neg>::Output, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::Neg<Output: Scalar>,
    {
        if vector.is_aligned() {
            unsafe {
                match N {
                    2 => {
                        return Vector::<2, _, VecAligned>::from_inner(
                            -vector.transmute_vec2().inner,
                        )
                        .transmute_len::<N>()
                        .transmute_alignment::<A>();
                    }
                    3 => {
                        return Vector::<3, _, VecAligned>::from_inner(
                            -vector.transmute_vec3().inner,
                        )
                        .transmute_len::<N>()
                        .transmute_alignment::<A>();
                    }
                    4 => {
                        return Vector::<4, _, VecAligned>::from_inner(
                            -vector.transmute_vec4().inner,
                        )
                        .transmute_len::<N>()
                        .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        vector.map(|x| -x)
    }

    fn vec_add<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, <Self as std::ops::Add<T2>>::Output, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::Add<T2, Output: Scalar>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output =
                            Vector::<2, Self, VecAligned>::from_inner(vector.inner + other.inner);

                        return output
                            .transmute_len::<N>()
                            .transmute_scalar::<<Self as std::ops::Add<T2>>::Output>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] + other[i])
    }

    fn vec_sub<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, <Self as std::ops::Sub<T2>>::Output, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::Sub<T2, Output: Scalar>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output =
                            Vector::<2, Self, VecAligned>::from_inner(vector.inner - other.inner);

                        return output
                            .transmute_len::<N>()
                            .transmute_scalar::<<Self as std::ops::Sub<T2>>::Output>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] - other[i])
    }

    fn vec_mul<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, <Self as std::ops::Mul<T2>>::Output, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::Mul<T2, Output: Scalar>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output =
                            Vector::<2, Self, VecAligned>::from_inner(vector.inner * other.inner);

                        return output
                            .transmute_len::<N>()
                            .transmute_scalar::<<Self as std::ops::Mul<T2>>::Output>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] * other[i])
    }

    fn vec_div<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, <Self as std::ops::Div<T2>>::Output, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::Div<T2, Output: Scalar>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output =
                            Vector::<2, Self, VecAligned>::from_inner(vector.inner / other.inner);

                        return output
                            .transmute_len::<N>()
                            .transmute_scalar::<<Self as std::ops::Div<T2>>::Output>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] / other[i])
    }

    fn vec_rem<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: Vector<N, Self, A>,
        other: Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, <Self as std::ops::Rem<T2>>::Output, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::Rem<T2, Output: Scalar>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output =
                            Vector::<2, Self, VecAligned>::from_inner(vector.inner % other.inner);

                        return output
                            .transmute_len::<N>()
                            .transmute_scalar::<<Self as std::ops::Rem<T2>>::Output>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] % other[i])
    }

    fn vec_add_assign<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl crate::VecAlignment>,
    ) where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::AddAssign<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        return {
                            let vector = vector.transmute_vec2_mut();
                            let other = other.transmute_vec2().transmute_scalar::<Self>();

                            vector.inner += other.inner;
                        };
                    }
                    3 => {
                        return {
                            let vector = vector.transmute_vec3_mut();
                            let other = other.transmute_vec3().transmute_scalar::<Self>();

                            vector.inner += other.inner;
                        };
                    }
                    4 => {
                        return {
                            let vector = vector.transmute_vec4_mut();
                            let other = other.transmute_vec4().transmute_scalar::<Self>();

                            vector.inner += other.inner;
                        };
                    }
                    _ => {}
                }
            }
        }

        for i in 0..N {
            vector[i] += other[i];
        }
    }

    fn vec_sub_assign<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl crate::VecAlignment>,
    ) where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::SubAssign<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        return {
                            let vector = vector.transmute_vec2_mut();
                            let other = other.transmute_vec2().transmute_scalar::<Self>();

                            vector.inner -= other.inner;
                        };
                    }
                    3 => {
                        return {
                            let vector = vector.transmute_vec3_mut();
                            let other = other.transmute_vec3().transmute_scalar::<Self>();

                            vector.inner -= other.inner;
                        };
                    }
                    4 => {
                        return {
                            let vector = vector.transmute_vec4_mut();
                            let other = other.transmute_vec4().transmute_scalar::<Self>();

                            vector.inner -= other.inner;
                        };
                    }
                    _ => {}
                }
            }
        }

        for i in 0..N {
            vector[i] -= other[i];
        }
    }

    fn vec_mul_assign<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl crate::VecAlignment>,
    ) where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::MulAssign<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        return {
                            let vector = vector.transmute_vec2_mut();
                            let other = other.transmute_vec2().transmute_scalar::<Self>();

                            vector.inner *= other.inner;
                        };
                    }
                    3 => {
                        return {
                            let vector = vector.transmute_vec3_mut();
                            let other = other.transmute_vec3().transmute_scalar::<Self>();

                            vector.inner *= other.inner;
                        };
                    }
                    4 => {
                        return {
                            let vector = vector.transmute_vec4_mut();
                            let other = other.transmute_vec4().transmute_scalar::<Self>();

                            vector.inner *= other.inner;
                        };
                    }
                    _ => {}
                }
            }
        }

        for i in 0..N {
            vector[i] *= other[i];
        }
    }

    fn vec_div_assign<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl crate::VecAlignment>,
    ) where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::DivAssign<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        return {
                            let vector = vector.transmute_vec2_mut();
                            let other = other.transmute_vec2().transmute_scalar::<Self>();

                            vector.inner /= other.inner;
                        };
                    }
                    3 => {
                        return {
                            let vector = vector.transmute_vec3_mut();
                            let other = other.transmute_vec3().transmute_scalar::<Self>();

                            vector.inner /= other.inner;
                        };
                    }
                    4 => {
                        return {
                            let vector = vector.transmute_vec4_mut();
                            let other = other.transmute_vec4().transmute_scalar::<Self>();

                            vector.inner /= other.inner;
                        };
                    }
                    _ => {}
                }
            }
        }

        for i in 0..N {
            vector[i] /= other[i];
        }
    }

    fn vec_rem_assign<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &mut Vector<N, Self, A>,
        other: Vector<N, T2, impl crate::VecAlignment>,
    ) where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::RemAssign<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        return {
                            let vector = vector.transmute_vec2_mut();
                            let other = other.transmute_vec2().transmute_scalar::<Self>();

                            vector.inner %= other.inner;
                        };
                    }
                    3 => {
                        return {
                            let vector = vector.transmute_vec3_mut();
                            let other = other.transmute_vec3().transmute_scalar::<Self>();

                            vector.inner %= other.inner;
                        };
                    }
                    4 => {
                        return {
                            let vector = vector.transmute_vec4_mut();
                            let other = other.transmute_vec4().transmute_scalar::<Self>();

                            vector.inner %= other.inner;
                        };
                    }
                    _ => {}
                }
            }
        }

        for i in 0..N {
            vector[i] %= other[i];
        }
    }

    fn vec_abs_diff<const N: usize, A: crate::VecAlignment>(
        vector: Vector<N, Self, A>,
        other: Vector<N, Self, impl crate::VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialOrd + std::ops::Sub<Output = Self>,
    {
        (vector - other).abs()
    }

    fn vec_eq<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl crate::VecAlignment>,
    ) -> bool
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialEq<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        return vector.transmute_vec2().inner
                            == other.transmute_vec2().transmute_scalar::<Self>().inner;
                    }
                    3 => {
                        return vector.transmute_vec3().inner
                            == other.transmute_vec3().transmute_scalar::<Self>().inner;
                    }
                    4 => {
                        return vector.transmute_vec4().inner
                            == other.transmute_vec4().transmute_scalar::<Self>().inner;
                    }
                    _ => {}
                }
            }
        }

        vector.eq_mask(other).all_true()
    }

    fn vec_ne<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl crate::VecAlignment>,
    ) -> bool
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialEq<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        return vector.transmute_vec2().inner
                            != other.transmute_vec2().transmute_scalar::<Self>().inner;
                    }
                    3 => {
                        return vector.transmute_vec3().inner
                            != other.transmute_vec3().transmute_scalar::<Self>().inner;
                    }
                    4 => {
                        return vector.transmute_vec4().inner
                            != other.transmute_vec4().transmute_scalar::<Self>().inner;
                    }
                    _ => {}
                }
            }
        }

        vector.ne_mask(other).any_true()
    }

    fn vec_eq_mask<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialEq<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output = vector.inner.cmpeq(other.inner);

                        return Vector::<2, bool, VecAligned>::from_inner(output)
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    3 => {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<Self>();

                        let output = vector.inner.cmpeq(other.inner);

                        return Vector::<3, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    4 => {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<Self>();

                        let output = vector.inner.cmpeq(other.inner);

                        return Vector::<4, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] == other[i])
    }

    fn vec_ne_mask<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialEq<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output = vector.inner.cmpne(other.inner);

                        return Vector::<2, bool, VecAligned>::from_inner(output)
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    3 => {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<Self>();

                        let output = vector.inner.cmpne(other.inner);

                        return Vector::<3, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    4 => {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<Self>();

                        let output = vector.inner.cmpne(other.inner);

                        return Vector::<4, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] != other[i])
    }

    fn vec_lt_mask<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialOrd<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output = vector.inner.cmplt(other.inner);

                        return Vector::<2, bool, VecAligned>::from_inner(output)
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    3 => {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<Self>();

                        let output = vector.inner.cmplt(other.inner);

                        return Vector::<3, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    4 => {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<Self>();

                        let output = vector.inner.cmplt(other.inner);

                        return Vector::<4, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] < other[i])
    }

    fn vec_gt_mask<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialOrd<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output = vector.inner.cmpgt(other.inner);

                        return Vector::<2, bool, VecAligned>::from_inner(output)
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    3 => {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<Self>();

                        let output = vector.inner.cmpgt(other.inner);

                        return Vector::<3, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    4 => {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<Self>();

                        let output = vector.inner.cmpgt(other.inner);

                        return Vector::<4, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] > other[i])
    }

    fn vec_le_mask<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialOrd<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output = vector.inner.cmple(other.inner);

                        return Vector::<2, bool, VecAligned>::from_inner(output)
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    3 => {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<Self>();

                        let output = vector.inner.cmple(other.inner);

                        return Vector::<3, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    4 => {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<Self>();

                        let output = vector.inner.cmple(other.inner);

                        return Vector::<4, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] <= other[i])
    }

    fn vec_ge_mask<const N: usize, A: crate::VecAlignment, T2: Scalar>(
        vector: &Vector<N, Self, A>,
        other: &Vector<N, T2, impl crate::VecAlignment>,
    ) -> Vector<N, bool, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialOrd<T2>,
    {
        if vector.is_aligned() && other.is_aligned() && TypeId::of::<T2>() == TypeId::of::<Self>() {
            unsafe {
                match N {
                    2 => {
                        let vector = vector.transmute_vec2();
                        let other = other.transmute_vec2().transmute_scalar::<Self>();

                        let output = vector.inner.cmpge(other.inner);

                        return Vector::<2, bool, VecAligned>::from_inner(output)
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    3 => {
                        let vector = vector.transmute_vec3();
                        let other = other.transmute_vec3().transmute_scalar::<Self>();

                        let output = vector.inner.cmpge(other.inner);

                        return Vector::<3, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    4 => {
                        let vector = vector.transmute_vec4();
                        let other = other.transmute_vec4().transmute_scalar::<Self>();

                        let output = vector.inner.cmpge(other.inner);

                        return Vector::<4, bool, VecAligned>::from_array(output.into())
                            .transmute_len::<N>()
                            .transmute_alignment::<A>();
                    }
                    _ => {}
                }
            }
        }

        Vector::from_fn(|i| vector[i] >= other[i])
    }

    fn vec_sum<const N: usize, A: crate::VecAlignment>(vector: Vector<N, Self, A>) -> Self
    where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::Add<Output = Self>,
    {
        if vector.is_aligned() {
            unsafe {
                match N {
                    2 => {
                        return vector.transmute_vec2().inner.element_sum();
                    }
                    3 => {
                        return vector.transmute_vec3().inner.element_sum();
                    }
                    4 => {
                        return vector.transmute_vec4().inner.element_sum();
                    }
                    _ => {}
                }
            }
        }

        vector.fold(|acc, x| acc + x)
    }

    fn vec_product<const N: usize, A: crate::VecAlignment>(vector: Vector<N, Self, A>) -> Self
    where
        crate::Usize<N>: crate::VecLen,
        Self: std::ops::Mul<Output = Self>,
    {
        if vector.is_aligned() {
            unsafe {
                match N {
                    2 => {
                        return vector.transmute_vec2().inner.element_product();
                    }
                    3 => {
                        return vector.transmute_vec3().inner.element_product();
                    }
                    4 => {
                        return vector.transmute_vec4().inner.element_product();
                    }
                    _ => {}
                }
            }
        }

        vector.fold(|acc, x| acc * x)
    }
}
