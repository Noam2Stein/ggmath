use crate::{ElementOfVector, Simd, Simdness, Vector, sealed::Sealed};

mod f32;
mod f64;

/// TODO: add doc
pub trait FloatElementOfVector<const N: usize, S: Simdness>:
    Sealed + ElementOfVector<N, S>
{
    /// Implementation of `Vector::floor` for this element type.
    fn vec_floor(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::ceil` for this element type.
    fn vec_ceil(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::round` for this element type.
    fn vec_round(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::trunc` for this element type.
    fn vec_trunc(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::fract` for this element type.
    fn vec_fract(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::mul_add` for this element type.
    fn vec_mul_add(
        vec: Vector<N, Self, S>,
        a: Vector<N, Self, S>,
        b: Vector<N, Self, S>,
    ) -> Vector<N, Self, S>;

    /// Implementation of `Vector::div_euclid` for this element type.
    fn vec_div_euclid(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::rem_euclid` for this element type.
    fn vec_rem_euclid(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::sqrt` for this element type.
    fn vec_sqrt(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::sin` for this element type.
    fn vec_sin(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::cos` for this element type.
    fn vec_cos(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::tan` for this element type.
    fn vec_tan(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::asin` for this element type.
    fn vec_asin(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::acos` for this element type.
    fn vec_acos(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::atan` for this element type.
    fn vec_atan(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::recip` for this element type.
    fn vec_recip(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::max` for this element type.
    fn vec_max(vec: Vector<N, Self, S>, other: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::min` for this element type.
    fn vec_min(vec: Vector<N, Self, S>, other: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::midpoint` for this element type.
    fn vec_midpoint(vec: Vector<N, Self, S>, other: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::clamp` for this element type.
    fn vec_clamp(
        vec: Vector<N, Self, S>,
        min: Vector<N, Self, S>,
        max: Vector<N, Self, S>,
    ) -> Vector<N, Self, S>;

    /// Implementation of `Vector::abs` for this element type.
    fn vec_abs(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::signum` for this element type.
    fn vec_signum(vec: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::copysign` for this element type.
    fn vec_copysign(vec: Vector<N, Self, S>, sign: Vector<N, Self, S>) -> Vector<N, Self, S>;

    /// Implementation of `Vector::sum` for this element type.
    fn vec_sum(vec: Vector<N, Self, S>) -> Self;

    /// Implementation of `Vector::product` for this element type.
    fn vec_product(vec: Vector<N, Self, S>) -> Self;
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! impl_float_element_of_vector {
    (impl for $T:ty) => {
        $crate::impl_float_element_of_vector!(for N = 0: impl$(<($($impl_param_tt)*)>)? for $T);
        $crate::impl_float_element_of_vector!(for N = 1: impl$(<($($impl_param_tt)*)>)? for $T);
        $crate::impl_float_element_of_vector!(for N = 2: impl$(<($($impl_param_tt)*)>)? for $T);
        $crate::impl_float_element_of_vector!(for N = 3: impl$(<($($impl_param_tt)*)>)? for $T);
        $crate::impl_float_element_of_vector!(for N = 4: impl$(<($($impl_param_tt)*)>)? for $T);
        $crate::impl_float_element_of_vector!(for N = 5: impl$(<($($impl_param_tt)*)>)? for $T);
        $crate::impl_float_element_of_vector!(for N = 6: impl$(<($($impl_param_tt)*)>)? for $T);
        $crate::impl_float_element_of_vector!(for N = 7: impl$(<($($impl_param_tt)*)>)? for $T);
        $crate::impl_float_element_of_vector!(for N = 8: impl$(<($($impl_param_tt)*)>)? for $T);
    };
    (for N = $N:literal: impl for $T:ty) => {
        impl $crate::FloatElementOfVector<$N, $crate::Simd> for $T {
            #[inline(always)]
            fn vec_floor(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::floor)
            }

            #[inline(always)]
            fn vec_ceil(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::ceil)
            }

            #[inline(always)]
            fn vec_round(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::round)
            }

            #[inline(always)]
            fn vec_trunc(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::trunc)
            }

            #[inline(always)]
            fn vec_fract(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::fract)
            }

            #[inline(always)]
            fn vec_mul_add(
                vec: $crate::Vector<$N, Self, $crate::Simd>,
                a: $crate::Vector<$N, Self, $crate::Simd>,
                b: $crate::Vector<$N, Self, $crate::Simd>,
            ) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.zip(a).zip(b).map(|((x, a), b)| x.mul_add(a, b))
            }

            #[inline(always)]
            fn vec_div_euclid(vec: $crate::Vector<$N, Self, $crate::Simd>, rhs: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.zip(rhs).map(|(x, y)| x.div_euclid(y))
            }

            #[inline(always)]
            fn vec_rem_euclid(vec: $crate::Vector<$N, Self, $crate::Simd>, rhs: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.zip(rhs).map(|(x, y)| x.rem_euclid(y))
            }

            #[inline(always)]
            fn vec_sqrt(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::sqrt)
            }

            #[inline(always)]
            fn vec_sin(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::sin)
            }

            #[inline(always)]
            fn vec_cos(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::cos)
            }

            #[inline(always)]
            fn vec_tan(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::tan)
            }

            #[inline(always)]
            fn vec_asin(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::asin)
            }

            #[inline(always)]
            fn vec_acos(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::acos)
            }

            #[inline(always)]
            fn vec_atan(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::atan)
            }

            #[inline(always)]
            fn vec_recip(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::recip)
            }

            #[inline(always)]
            fn vec_max(vec: $crate::Vector<$N, Self, $crate::Simd>, other: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                core::debug_assert!(vec.iter().all(|x| !x.is_nan()));
                core::debug_assert!(other.iter().all(|x| !x.is_nan()));

                vec.zip(other).map(|(x, y)| x.max(y))
            }

            #[inline(always)]
            fn vec_min(vec: $crate::Vector<$N, Self, $crate::Simd>, other: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                core::debug_assert!(vec.iter().all(|x| !x.is_nan()));
                core::debug_assert!(other.iter().all(|x| !x.is_nan()));

                vec.zip(other).map(|(x, y)| x.min(y))
            }

            #[inline(always)]
            fn vec_midpoint(vec: $crate::Vector<$N, Self, $crate::Simd>, other: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.zip(other).map(|(x, y)| x.midpoint(y))
            }

            #[inline(always)]
            fn vec_clamp(
                vec: $crate::Vector<$N, Self, $crate::Simd>,
                min: $crate::Vector<$N, Self, $crate::Simd>,
                max: $crate::Vector<$N, Self, $crate::Simd>,
            ) -> $crate::Vector<$N, Self, $crate::Simd> {
                core::debug_assert!(min.zip(max).iter().all(|(min, max)| min <= max));
                core::debug_assert!(min.iter().all(|x| !x.is_nan()));
                core::debug_assert!(max.iter().all(|x| !x.is_nan()));

                vec.zip(min).zip(max).map(|((x, min), max)| x.clamp(min, max))
            }

            #[inline(always)]
            fn vec_abs(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::abs)
            }

            #[inline(always)]
            fn vec_signum(vec: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.map(Self::signum)
            }

            #[inline(always)]
            fn vec_copysign(vec: $crate::Vector<$N, Self, $crate::Simd>, sign: $crate::Vector<$N, Self, $crate::Simd>) -> $crate::Vector<$N, Self, $crate::Simd> {
                vec.zip(sign).map(|(x, sign)| x.copysign(sign))
            }

            #[inline(always)]
            fn vec_sum(vec: $crate::Vector<$N, Self, $crate::Simd>) -> Self {
                vec.iter().sum()
            }

            #[inline(always)]
            fn vec_product(vec: $crate::Vector<$N, Self, $crate::Simd>) -> Self {
                vec.iter().product()
            }
        }
    }
}

impl Sealed for f32 {}
impl Sealed for f64 {}

macro_rules! assert_primitive_is_element {
    ($T:ty: $ElementOfVector:ident) => {
        const _: () = {
            const fn helper<
                T: $ElementOfVector<0, Simd>
                    + $ElementOfVector<1, Simd>
                    + $ElementOfVector<2, Simd>
                    + $ElementOfVector<3, Simd>
                    + $ElementOfVector<4, Simd>
                    + $ElementOfVector<5, Simd>
                    + $ElementOfVector<6, Simd>
                    + $ElementOfVector<7, Simd>
                    + $ElementOfVector<8, Simd>,
            >() {
            }

            helper::<$T>();
        };
    };
}
assert_primitive_is_element!(f32: FloatElementOfVector);
assert_primitive_is_element!(f64: FloatElementOfVector);
assert_primitive_is_element!(i8: ElementOfVector);
assert_primitive_is_element!(i16: ElementOfVector);
assert_primitive_is_element!(i32: ElementOfVector);
assert_primitive_is_element!(i64: ElementOfVector);
assert_primitive_is_element!(i128: ElementOfVector);
assert_primitive_is_element!(isize: ElementOfVector);
assert_primitive_is_element!(u8: ElementOfVector);
assert_primitive_is_element!(u16: ElementOfVector);
assert_primitive_is_element!(u32: ElementOfVector);
assert_primitive_is_element!(u64: ElementOfVector);
assert_primitive_is_element!(u128: ElementOfVector);
assert_primitive_is_element!(usize: ElementOfVector);
assert_primitive_is_element!(bool: ElementOfVector);
assert_primitive_is_element!(char: ElementOfVector);
