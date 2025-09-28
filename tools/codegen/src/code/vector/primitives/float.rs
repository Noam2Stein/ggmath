use genco::{ quote};
use strum::IntoEnumIterator;

use crate::{code::vector::primitives::{PrimitiveSrcMod, PrimitiveTestMod}, iter::{Axis, Length, PrimitiveFloat, PrimitiveInt, Simdness}};



pub fn push_src(primitive: PrimitiveFloat, output: &mut PrimitiveSrcMod) {
    output.impl_items.push(quote! {
        $("// The following code is generated for all float primitives")

        $("/// A vector of all minimum values.")
        pub const MIN: Self = Self::const_splat($primitive::MIN);
        $("/// A vector of all maximum values.")
        pub const MAX: Self = Self::const_splat($primitive::MAX);
        $("/// A vector with all elements set to `NaN`.")
        pub const NAN: Self = Self::const_splat($primitive::NAN);
        $("/// A vector with all elements set to `Infinity`.")
        pub const INFINITY: Self = Self::const_splat($primitive::INFINITY);
        $("/// A vector with all elements set to `-Infinity`.")
        pub const NEG_INFINITY: Self = Self::const_splat($primitive::NEG_INFINITY);
        $("/// A vector with all elements set to `Epsilon`.")
        pub const EPSILON: Self = Self::const_splat($primitive::EPSILON);

        $(
            for primitive2 in PrimitiveFloat::iter().filter(|&primitive2| primitive2 != primitive) join($['\n']) =>

            $(format!("/// Converts `self` to a vector of `{primitive2}` elements."))
            #[inline(always)]
            pub fn as_$(primitive2)(self) -> Vector<N, $primitive2, S> {
                self.map(|x| x as $primitive2)
            }
        )

        $(
            for primitive2 in PrimitiveInt::iter() join($['\n']) =>

            $(format!("/// Converts `self` to a vector of `{primitive2}` elements."))
            #[inline(always)]
            pub fn as_$(primitive2)(self) -> Vector<N, $primitive2, S> {
                self.map(|x| x as $primitive2)
            }
        )

        $("/// Returns a vector of the Euclidean division of each element by `other`.")
        #[inline(always)]
        pub fn div_euclid(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).div_euclid(other.index(i)))
        }

        $("/// Returns a vector of the remainder of the Euclidean division of each element by `other`.")
        #[inline(always)]
        pub fn rem_euclid(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).rem_euclid(other.index(i)))
        }

        $("/// Returns a vector of the minimum of each element between `self` and `other`.")
        #[inline(always)]
        pub fn min(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).min(other.index(i)))
        }
        
        $("/// Returns a vector of the maximum of each element between `self` and `other`.")
        #[inline(always)]
        pub fn max(self, other: Self) -> Self {
            Vector::from_fn(|i| self.index(i).max(other.index(i)))
        }
        
        $("/// Returns a vector with each element clamped between `min` and `max`.")
        #[inline(always)]
        pub fn clamp(self, min: Self, max: Self) -> Self {
            self.max(min).min(max)
        }

        $("/// Returns the minimum element in the vector.")
        #[inline(always)]
        pub fn min_element(self) -> $primitive {
            self.reduce(|a, b| a.min(b))
        }

        $("/// Returns the maximum element in the vector.")
        #[inline(always)]
        pub fn max_element(self) -> $primitive {
            self.reduce(|a, b| a.max(b))
        }

        $("/// Returns a vector containing the signum of each element of `self`.")
        $("/// Signum for each element is:")
        $("/// - `1.0` if the element is positive or `+0.0`")
        $("/// - `-1.0` if the element is negative `-0.0`")
        #[inline(always)]
        pub fn signum(self) -> Self {
            self.map(|x| x.signum())
        }

        $("/// Returns a vector containing the absolute value of each element of `self`.")
        #[inline(always)]
        pub fn abs(self) -> Self {
            self.map(|x| x.abs())
        }

        $("/// Returns a vector containing the squared distance between each element of `self` and `other`.")
        #[inline(always)]
        pub fn distance_sq(self, other: Self) -> $primitive {
            (self - other).mag_sq()
        }

        $("/// Returns a vector of bools with `true` for each element that has a negative sign, including `-0.0`.")
        #[inline(always)]
        pub fn negative_sign_mask(self) -> Vector<N, bool, S> {
            self.map(|x| x.is_sign_negative())
        }

        $("/// Returns a vector of bools with `true` for each element that has a positive sign, including `+0.0`.")
        #[inline(always)]
        pub fn positive_sign_mask(self) -> Vector<N, bool, S> {
            self.map(|x| x.is_sign_positive())
        }

        $("/// Returns a vector of bools with `true` for each element that is `NaN`.")
        #[inline(always)]
        pub fn nan_mask(self) -> Vector<N, bool, S> {
            self.map(|x| x.is_nan())
        }

        $("/// Returns a vector of bools with `true` for each element that is finite.")
        #[inline(always)]
        pub fn finite_mask(self) -> Vector<N, bool, S> {
            self.map(|x| x.is_finite())
        }

        $("/// Returns `true` if any element is `NaN`.")
        #[inline(always)]
        pub fn is_nan(self) -> bool {
            self.nan_mask().any_true()
        }

        $("/// Returns `true` if all elements are finite.")
        #[inline(always)]
        pub fn is_finite(self) -> bool {
            self.finite_mask().all_true()
        }

        $("/// Returns a vector with the same direction as `self`, but with a magnitude of `1`.")
        $("/// If `self` is zero, `NaN` is returned.")
        #[inline(always)]
        pub fn normalize(self) -> Self {
            self / self.mag()
        }

        $("/// Returns a vector with the same direction as `self`, but with a magnitude of `1`.")
        $("/// If `self` is zero, `None` is returned.")
        #[inline(always)]
        pub fn checked_normalize(self) -> Option<Self> {
            let normalized = self.normalize();
            if normalized.is_finite() {
                Some(normalized)
            } else {
                None
            }
        }

        $("/// Returns a vector with the same direction as `self`, but with a magnitude of `1`.")
        #[inline(always)]
        pub fn normalize_or(self, default: Self) -> Self {
            let normalized = self.normalize();
            if normalized.is_finite() {
                normalized
            } else {
                default
            }
        }

        $("/// Returns a vector with the same direction as `self`, but with a magnitude of `1`.")
        $("/// If `self` is zero, zero is returned.")
        #[inline(always)]
        pub fn normalize_or_zero(self) -> Self {
            self.normalize_or(Self::ZERO)
        }

        $("/// Linearly interpolates between `self` and `other` based on the interpolation factor `t`,")
        $("/// which is clamped to the range `[0.0, 1.0]`.")
        $("///")
        $("/// This function uses the \"delta lerp\" formula:")
        $("/// `a + (b - a) * t`,")
        $("/// which is more numerically stable and is usually faster than the \"weighted lerp\" formula:")
        $("/// `a * (1.0 - t) + b * t`.")
        $("///")
        $("/// The weighted formula can be used by calling [`Self::lerp_weighted`],")
        $("/// and is more numerically stable when interpolating large values that are far away from each other.")
        #[inline(always)]
        pub fn lerp(self, other: Self, t: $primitive) -> Self {
            self.lerp_unclamped(other, t.clamp(0.0, 1.0))
        }
        
        $("/// Linearly interpolates between `self` and `other` based on the interpolation factor `t`,")
        $("/// which is clamped to the range `[0.0, 1.0]`.")
        $("///")
        $("/// This function uses the \"weighted lerp\" formula:")
        $("/// `a * (1.0 - t) + b * t`")
        $("/// which is less numerically stable and usually slower than the \"delta lerp\" formula:")
        $("/// `a + (b - a) * t`.")
        $("///")
        $("/// This weighted formula is more numerically stable when interpolating large values that are far away from each other.")
        #[inline(always)]
        pub fn lerp_weighted(self, other: Self, t: $primitive) -> Self {
            self.lerp_unclamped_weighted(other, t.clamp(0.0, 1.0))
        }

        $("/// Linearly interpolates between `self` and `other` based on the interpolation factor `t`,")
        $("/// or linearly extrapolates if `t` is outside the range `[0.0, 1.0]`.")
        $("///")
        $("/// This function uses the \"delta lerp\" formula:")
        $("/// `a + (b - a) * t`")
        $("/// which is more numerically stable and is usually faster than the \"weighted lerp\" formula:")
        $("/// `a * (1.0 - t) + b * t`.")
        $("///")
        $("/// The weighted formula can be used by calling [`FloatExt::lerp_unclamped_weighted`],")
        $("/// and is more numerically stable when interpolating large values that are far away from each other.")
        #[inline(always)]
        pub fn lerp_unclamped(self, other: Self, t: $primitive) -> Self {
            self + (other - self) * t
        }
        
        $("/// Linearly interpolates between `self` and `other` based on the interpolation factor `t`,")
        $("/// or linearly extrapolates if `t` is outside the range `[0.0, 1.0]`.")
        $("///")
        $("/// This function uses the \"weighted lerp\" formula:")
        $("/// `a * (1.0 - t) + b * t`")
        $("/// which is less numerically stable and usually slower than the \"delta lerp\" formula:")
        $("/// `a + (b - a) * t`.")
        $("///")
        $("/// This weighted formula is more numerically stable when interpolating large values that are far away from each other.")
        #[inline(always)]
        pub fn lerp_unclamped_weighted(self, other: Self, t: $primitive) -> Self {
            self * (1.0 - t) + other * t
        }

        $("/// Moves `self` towards `target` by at most `max_delta`.")
        #[inline(always)]
        pub fn move_towards(self, target: Self, max_delta: $primitive) -> Self {
            let delta = target - self;
            let delta_mag = delta.mag();
            if delta_mag <= max_delta || delta_mag <= 1e-4 {
                return target;
            }
            self + delta / delta_mag * max_delta
        }

        $("/// Returns the projection of `self` onto `other`.")
        #[inline(always)]
        pub fn project_onto(self, other: Self) -> Self {
            other * self.dot(other) * (1.0 / other.mag_sq())
        }

        $("/// Returns the projection of `self` onto `other`,")
        $("/// where `other` must be normalized.")
        $("///")
        $("/// This is faster than `project_onto`.")
        #[inline(always)]
        pub fn project_onto_normalized(self, other: Self) -> Self {
            #[cfg(debug_assertions)]
            assert!(other.mag_sq() == 1.0, "other must be normalized");

            other * self.dot(other)
        }

        $("/// Returns the rejection of `self` from `other`.")
        #[inline(always)]
        pub fn reject_from(self, other: Self) -> Self {
            self - self.project_onto(other)
        }

        $("/// Returns the rejection of `self` from `other`,")
        $("/// where `other` must be normalized.")
        $("///")
        $("/// This is faster than `reject_from`.")
        #[inline(always)]
        pub fn reject_from_normalized(self, other: Self) -> Self {
            #[cfg(debug_assertions)]
            assert!(other.mag_sq() == 1.0, "other must be normalized");

            self - self.project_onto_normalized(other)
        }

        $("/// Returns the reflection of `self` off of `normal`.")
        $("///")
        $("/// `normal` must be normalized.")
        #[inline(always)]
        pub fn reflect(self, normal: Self) -> Self {
            #[cfg(debug_assertions)]
            assert!(normal.mag_sq() == 1.0, "normal must be normalized");

            self - normal * (2.0 * self.dot(normal))
        }

        $("/// Returns the refraction of `self` through `normal` for the given ratio of indices of refraction.")
        $("///")
        $("/// `self` and `normal` must be normalized.")
        #[inline(always)]
        pub fn refract(self, normal: Self, eta: $primitive) -> Self {
            #[cfg(debug_assertions)]
            assert!(self.mag_sq() == 1.0, "self must be normalized");

            #[cfg(debug_assertions)]
            assert!(normal.mag_sq() == 1.0, "normal must be normalized");

            let n_dot_i = normal.dot(self);
            let k = 1.0 - eta * eta * (1.0 - n_dot_i * n_dot_i);
            if k >= 0.0 {
                self * eta - normal * (eta * n_dot_i + k.sqrt())
            } else {
                Self::ZERO
            }
        }
    });

    output.std_impl_items.push(quote! {
        $("// The following code is generated for all float primitives")

        $("/// Returns a vector containing the square root of each element of `self`.")
        #[inline(always)]
        pub fn sqrt(self) -> Self {
            self.map(|x| x.sqrt())
        }

        $("/// Returns a vector containing the rounded value of each element of `self`.")
        #[inline(always)]
        pub fn round(self) -> Self {
            self.map(|x| x.round())
        }

        $("/// Returns a vector containing the floor value of each element of `self`.")
        #[inline(always)]
        pub fn floor(self) -> Self {
            self.map(|x| x.floor())
        }

        $("/// Returns a vector containing the ceiling value of each element of `self`.")
        #[inline(always)]
        pub fn ceil(self) -> Self {
            self.map(|x| x.ceil())
        }

        $("/// Returns a vector containing the truncated value of each element of `self`.")
        #[inline(always)]
        pub fn trunc(self) -> Self {
            self.map(|x| x.trunc())
        }

        $("/// Returns a vector containing the fractional part of each element of `self` as `self - self.trunc()`.")
        #[inline(always)]
        pub fn fract(self) -> Self {
            self.map(|x| x.fract())
        }

        $("/// Returns a vector containing the sine of each element of `self`.")
        #[inline(always)]
        pub fn sin(self) -> Self {
            self.map(|x| x.sin())
        }

        $("/// Returns a vector containing the cosine of each element of `self`.")
        #[inline(always)]
        pub fn cos(self) -> Self {
            self.map(|x| x.cos())
        }

        $("/// Returns a vector containing the tangent of each element of `self`.")
        #[inline(always)]
        pub fn tan(self) -> Self {
            self.map(|x| x.tan())
        }

        $("/// Returns a vector containing the arcsine of each element of `self`.")
        #[inline(always)]
        pub fn asin(self) -> Self {
            self.map(|x| x.asin())
        }

        $("/// Returns a vector containing the arccosine of each element of `self`.")
        #[inline(always)]
        pub fn acos(self) -> Self {
            self.map(|x| x.acos())
        }

        $("/// Returns a vector containing the arctangent of each element of `self`.")  
        #[inline(always)]
        pub fn atan(self) -> Self {
            self.map(|x| x.atan())
        }

        $("/// Returns a vector containing the hyperbolic sine of each element of `self`.")
        #[inline(always)]
        pub fn sinh(self) -> Self {
            self.map(|x| x.sinh())
        }

        $("/// Returns a vector containing the hyperbolic cosine of each element of `self`.")
        #[inline(always)]
        pub fn cosh(self) -> Self {
            self.map(|x| x.cosh())
        }

        $("/// Returns a vector containing the hyperbolic tangent of each element of `self`.")
        #[inline(always)]
        pub fn tanh(self) -> Self {
            self.map(|x| x.tanh())
        }

        $("/// Returns a vector containing the hyperbolic arclength sine of each element of `self`.")
        #[inline(always)]
        pub fn asinh(self) -> Self {
            self.map(|x| x.asinh())
        }

        $("/// Returns a vector containing the hyperbolic arclength cosine of each element of `self`.")
        #[inline(always)]
        pub fn acosh(self) -> Self {
            self.map(|x| x.acosh())
        }

        $("/// Returns a vector containing the hyperbolic arclength tangent of each element of `self`.")
        #[inline(always)]
        pub fn atanh(self) -> Self {
            self.map(|x| x.atanh())
        }

        $("/// Returns the magnitude of `self`.")
        #[inline(always)]
        pub fn mag(self) -> $primitive {
            self.mag_sq().sqrt()
        }

        $("/// Returns the Euclidean distance between `self` and `other`.")
        #[inline(always)]
        pub fn distance(self, other: Self) -> $primitive {
            self.distance_sq(other).sqrt()
        }

        $("/// Returns the angle in radians between `self` and `other` in the range `[0.0, π]`.")
        #[inline(always)]
        pub fn angle(self, other: Self) -> $primitive {
            (self.dot(other) / (self.mag_sq() * other.mag_sq()).sqrt()).acos()
        }
    });

    output.std_len_impl_items.entry(Length::N2).or_default().push(quote! {
        // The following code is generated for all float primitives

        $("/// Returns the signed angle in radians between `self` and `other` in the range `[-π, π]`.")
        #[inline(always)]
        pub fn signed_angle(self, other: Self) -> $primitive {
            self.angle(other) * self.perp_dot(other).signum()
        }
    });

    output.trait_impls.push(quote! {
        impl ScalarZero for $primitive {
            const ZERO: Self = 0.0;

            $(
                for n in Length::iter() join($['\r']) =>

                const VEC$(n)_ZERO: Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([0.0; $n]);
            )
        }

        impl ScalarOne for $primitive {
            const ONE: Self = 1.0;

            $(
                for n in Length::iter() join($['\r']) =>

                const VEC$(n)_ONE: Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([1.0; $n]);
            )

            $(
                for n in Length::iter() join($['\n']) => $(
                    for i in 0..n.as_usize() join($['\r']) =>

                    const VEC$(n)_$(Axis(i).uppercase_str()): Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([$(
                        for i2 in 0..n.as_usize() join(, ) => $(if i2 == i { 1.0 } else { 0.0 })
                    )]);
                )
            )
        }

        impl ScalarNegOne for $primitive {
            const NEG_ONE: Self = -1.0;

            $(
                for n in Length::iter() join($['\r']) =>

                const VEC$(n)_NEG_ONE: Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([-1.0; $n]);
            )

            $(
                for n in Length::iter() join($['\n']) => $(
                    for i in 0..n.as_usize() join($['\r']) =>

                    const VEC$(n)_NEG_$(Axis(i).uppercase_str()): Vec$(n)<$primitive> = Vec$(n)::<$primitive>::const_from_array([$(
                        for i2 in 0..n.as_usize() join(, ) => $(if i2 == i { -1.0 } else { 0.0 })
                    )]);
                )
            )
        }
    });
}

pub fn push_tests(primitive: PrimitiveFloat, output: &mut PrimitiveTestMod) {
    output.push_tests_for_vector(primitive, |n, s| quote! {
        $(let vec_lowercase = &format!("{t_prefix}vec{n}{s_postfix}", t_prefix = primitive.prefix_lowercase(), s_postfix = s.postfix_lowercase()))
        $(let vec_camelcase = &format!("{t_prefix}Vec{n}{s_postfix}", t_prefix = primitive.prefix_uppercase(), s_postfix = s.postfix_uppercase()))

        $(let values = (0..n.as_usize()).map(|i| (i as f64) * 1.3).collect::<Vec<f64>>())
        $(let values_str = &values.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $(let values2 = (0..n.as_usize()).map(|i| ((i + n.as_usize()) as f64) * 5.4).collect::<Vec<f64>>())
        $(let values2_str = &values2.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $(let values2_with_nan = (0..n.as_usize()).map(|i| if i == 1 { f64::NAN } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_nan_str = &values2_with_nan.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $(let values2_with_inf = (0..n.as_usize()).map(|i| if i == 1 { f64::INFINITY } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_inf_str = &values2_with_inf.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $(let values2_with_zero = (0..n.as_usize()).map(|i| if i == 1 { 0.0 } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_zero_str = &values2_with_zero.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $(let values2_with_neg_zero = (0..n.as_usize()).map(|i| if i == 1 { -0.0 } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_neg_zero_str = &values2_with_neg_zero.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))
        
        $(let values2_with_zero_and_neg_zero = (0..n.as_usize()).map(|i| if i == 0 { -0.0 } else if i == 1 { 0.0 } else { ((i + n.as_usize()) as f64) * 5.4 }).collect::<Vec<f64>>())
        $(let values2_with_zero_and_neg_zero_str = &values2_with_zero_and_neg_zero.iter().map(|&x| float_lit(x, primitive)).collect::<Vec<String>>().join(", "))

        $("// The following code is generated for all float primitives")

        #[test]
        fn test_$(vec_lowercase)_neg() {
            assert_approx_vec_eq!(
                -$vec_lowercase!($values_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(-values[i], primitive))))
            );
            assert_approx_vec_eq!(
                -$vec_lowercase!($values2_with_nan_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(-values2_with_nan[i], primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_add() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) + $vec_lowercase!($values2_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) + $vec_lowercase!($values2_with_nan_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2_with_nan[i], primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_sub() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) - $vec_lowercase!($values2_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] - values2[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) - $vec_lowercase!($values2_with_nan_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] - values2_with_nan[i], primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_mul() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) * $vec_lowercase!($values2_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] * values2[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) * $vec_lowercase!($values2_with_nan_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] * values2_with_nan[i], primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_div() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) / $vec_lowercase!($values2_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values2[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) / $vec_lowercase!($values2_with_nan_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values2_with_nan[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) / $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2[i] * (i as f64 - 1.0), primitive)))),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / (values2[i] * (i as f64 - 1.0)), primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_rem() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) % $vec_lowercase!($values2_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] % values2[i], primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str) % $vec_lowercase!($values2_with_nan_str),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] % values2_with_nan[i], primitive))))
            );
        }

        $(
            if n == Length::N4 && s == Simdness::NonSimd =>

            #[test]
            fn test_$(vec_lowercase)_add_assign() {
                let mut vec = $vec_lowercase!($values_str);
                vec += $vec_lowercase!($values2_str);
                assert_approx_vec_eq!(
                    vec,
                    $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
                );
            }

            #[test]
            fn test_$(vec_lowercase)_neg_ref() {
                assert_approx_vec_eq!(
                    -&$vec_lowercase!($values_str),
                    $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(-values[i], primitive))))
                );
            }

            #[test]
            fn test_$(vec_lowercase)_add_ref() {
                assert_approx_vec_eq!(
                    $vec_lowercase!($values_str) + &$vec_lowercase!($values2_str),
                    $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
                );
                assert_approx_vec_eq!(
                    &$vec_lowercase!($values_str) + $vec_lowercase!($values2_str),
                    $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
                );
                assert_approx_vec_eq!(
                    &$vec_lowercase!($values_str) + &$vec_lowercase!($values2_str),
                    $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
                );
            }

            #[test]
            fn test_$(vec_lowercase)_add_assign_ref() {
                let mut vec = $vec_lowercase!($values_str);
                vec += &$vec_lowercase!($values2_str);
                assert_approx_vec_eq!(
                    vec,
                    $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + values2[i], primitive))))
                );
            }

            #[test]
            fn test_$(vec_lowercase)_add_scalar() {
                assert_approx_vec_eq!(
                    $vec_lowercase!($values_str) + 1.0,
                    $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] + 1.0, primitive))))
                );
            }
        )

        #[test]
        fn test_$(vec_lowercase)_sum() {
            assert_approx_eq!(
                $vec_lowercase!($values_str).sum(),
                $(float_lit(values.iter().sum::<f64>(), primitive))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_product() {
            assert_approx_eq!(
                $vec_lowercase!($values2_str).product(),
                $(float_lit(values2.iter().product::<f64>(), primitive))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_mag_sq() {
            assert_approx_eq!(
                $vec_lowercase!($values_str).mag_sq(),
                $(float_lit(values.iter().map(|&x| x * x).sum::<f64>(), primitive))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_dot() {
            assert_approx_eq!(
                $vec_lowercase!($values_str).dot($vec_lowercase!($values2_str)),
                $(float_lit(values.iter().zip(values2.iter()).map(|(&x, &y)| x * y).sum::<f64>(), primitive))
            );
        }

        $(
            if n == Length::N2 =>

            #[test]
            fn test_$(vec_lowercase)_perp() {
                assert_approx_vec_eq!(
                    $vec_lowercase!(1.0$primitive, 0.0$primitive).perp(),
                    $vec_lowercase!(0.0$primitive, 1.0$primitive),
                );
            }

            #[test]
            fn test_$(vec_lowercase)_perp_cw() {
                assert_approx_vec_eq!(
                    $vec_lowercase!(1.0$primitive, 0.0$primitive).perp_cw(),
                    $vec_lowercase!(0.0$primitive, -1.0$primitive),
                );
            }
        )

        #[test]
        fn test_$(vec_lowercase)_div_euclid() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_str).div_euclid($vec_lowercase!($values_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2[i].div_euclid(values[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).div_euclid($vec_lowercase!($values2_with_nan_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].div_euclid(values2_with_nan[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).div_euclid($vec_lowercase!($values2_with_zero_and_neg_zero_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].div_euclid(values2_with_zero_and_neg_zero[i]), primitive))))
            );
        }
        
        #[test]
        fn test_$(vec_lowercase)_rem_euclid() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_str).rem_euclid($vec_lowercase!($values_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2[i].rem_euclid(values[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).rem_euclid($vec_lowercase!($values2_with_nan_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].rem_euclid(values2_with_nan[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).rem_euclid($vec_lowercase!($values2_with_zero_and_neg_zero_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].rem_euclid(values2_with_zero_and_neg_zero[i]), primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_min() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).min($vec_lowercase!($values2_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].min(values2[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).min($vec_lowercase!($values2_with_nan_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].min(values2_with_nan[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_with_zero_str).min($vec_lowercase!($values2_with_neg_zero_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero[i].min(values2_with_neg_zero[i]), primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_max() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).max($vec_lowercase!($values2_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].max(values2[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).max($vec_lowercase!($values2_with_nan_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].max(values2_with_nan[i]), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_with_zero_str).max($vec_lowercase!($values2_with_neg_zero_str)),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero[i].max(values2_with_neg_zero[i]), primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_min_element() {
            assert_approx_eq!(
                $vec_lowercase!($values_str).min_element(),
                $(float_lit(values.iter().cloned().reduce(f64::min).unwrap(), primitive))
            );
            assert_approx_eq!(
                $vec_lowercase!($values2_with_nan_str).min_element(),
                $(float_lit(values2_with_nan.iter().cloned().reduce(f64::min).unwrap(), primitive))
            );
            assert_approx_eq!(
                $vec_lowercase!($values2_with_zero_and_neg_zero_str).min_element(),
                $(float_lit(values2_with_zero_and_neg_zero.iter().cloned().reduce(f64::min).unwrap(), primitive))
            );
        }
        
        #[test]
        fn test_$(vec_lowercase)_max_element() {
            assert_approx_eq!(
                $vec_lowercase!($values_str).max_element(),
                $(float_lit(values.iter().cloned().reduce(f64::max).unwrap(), primitive))
            );
            assert_approx_eq!(
                $vec_lowercase!($values2_with_nan_str).max_element(),
                $(float_lit(values2_with_nan.iter().cloned().reduce(f64::max).unwrap(), primitive))
            );
            assert_approx_eq!(
                $vec_lowercase!($values2_with_zero_and_neg_zero_str).max_element(),
                $(float_lit(values2_with_zero_and_neg_zero.iter().cloned().reduce(f64::max).unwrap(), primitive))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_signum() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).signum(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].signum(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_with_nan_str).signum(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_nan[i].signum(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_with_zero_and_neg_zero_str).signum(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero_and_neg_zero[i].signum(), primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_abs() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).abs(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i].abs(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_with_nan_str).abs(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_nan[i].abs(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_with_zero_and_neg_zero_str).abs(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_zero_and_neg_zero[i].abs(), primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_positive_sign_mask() {
            assert_eq!(
                $vec_lowercase!($values_str).positive_sign_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values[i].is_sign_positive().to_string())))
            );
            assert_eq!(
                $vec_lowercase!($values2_with_nan_str).positive_sign_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values2_with_nan[i].is_sign_positive().to_string())))
            );
            assert_eq!(
                $vec_lowercase!($values2_with_zero_and_neg_zero_str).positive_sign_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values2_with_zero_and_neg_zero[i].is_sign_positive().to_string())))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_negative_sign_mask() {
            assert_eq!(
                $vec_lowercase!($values_str).negative_sign_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values[i].is_sign_negative().to_string())))
            );
            assert_eq!(
                $vec_lowercase!($values2_with_nan_str).negative_sign_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values2_with_nan[i].is_sign_negative().to_string())))
            );
            assert_eq!(
                $vec_lowercase!($values2_with_zero_and_neg_zero_str).negative_sign_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values2_with_zero_and_neg_zero[i].is_sign_negative().to_string())))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_nan_mask() {
            assert_eq!(
                $vec_lowercase!($values_str).nan_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values[i].is_nan().to_string())))
            );
            assert_eq!(
                $vec_lowercase!($values2_with_nan_str).nan_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values2_with_nan[i].is_nan().to_string())))
            );
            assert_eq!(
                $vec_lowercase!($values2_with_inf_str).nan_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values2_with_inf[i].is_nan().to_string())))
            );
        }
        
        #[test]
        fn test_$(vec_lowercase)_finite_mask() {
            assert_eq!(
                $vec_lowercase!($values_str).finite_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values[i].is_finite().to_string())))
            );
            assert_eq!(
                $vec_lowercase!($values2_with_nan_str).finite_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values2_with_nan[i].is_finite().to_string())))
            );
            assert_eq!(
                $vec_lowercase!($values2_with_inf_str).finite_mask(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(values2_with_inf[i].is_finite().to_string())))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_is_nan() {
            assert_eq!(
                $vec_lowercase!($values_str).is_nan(),
                $(values.iter().any(|&x| x.is_nan()).to_string())
            );
            assert_eq!(
                $vec_lowercase!($values2_with_nan_str).is_nan(),
                $(values2_with_nan.iter().any(|&x| x.is_nan()).to_string())
            );
            assert_eq!(
                $vec_lowercase!($values2_with_inf_str).is_nan(),
                $(values2_with_inf.iter().any(|&x| x.is_nan()).to_string())
            );
        }

        #[test]
        fn test_$(vec_lowercase)_is_finite() {
            assert_eq!(
                $vec_lowercase!($values_str).is_finite(),
                $(values.iter().all(|&x| x.is_finite()).to_string())
            );
            assert_eq!(
                $vec_lowercase!($values2_with_nan_str).is_finite(),
                $(values2_with_nan.iter().all(|&x| x.is_finite()).to_string())
            );
            assert_eq!(
                $vec_lowercase!($values2_with_inf_str).is_finite(),
                $(values2_with_inf.iter().all(|&x| x.is_finite()).to_string())
            );
        }

        #[test]
        fn test_$(vec_lowercase)_normalize() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).normalize(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_with_nan_str).normalize(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values2_with_nan[i] / values2_with_nan.iter().map(|&x| x * x).sum::<f64>().sqrt(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_camelcase::<$primitive>::ZERO.normalize(),
                $vec_lowercase!($(for _ in 0..n.as_usize() join(, ) => $(float_lit(0.0 / 0.0, primitive))))
            );
        }

        #[test]
        fn test_$(vec_lowercase)_checked_normalize() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).checked_normalize().unwrap(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), primitive))))
            );
            assert_eq!(
                $vec_lowercase!($values2_with_nan_str).checked_normalize(),
                None
            );
            assert_eq!(
                $vec_camelcase::<$primitive>::ZERO.checked_normalize(),
                None
            );
        }

        #[test]
        fn test_$(vec_lowercase)_normalize_or() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).normalize_or($vec_camelcase::<$primitive>::MAX),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_with_nan_str).normalize_or($vec_camelcase::<$primitive>::MAX),
                $vec_camelcase::<$primitive>::MAX
            );
            assert_approx_vec_eq!(
                $vec_camelcase::<$primitive>::ZERO.normalize_or($vec_camelcase::<$primitive>::MAX),
                $vec_camelcase::<$primitive>::MAX
            );
        }

        #[test]
        fn test_$(vec_lowercase)_normalize_or_zero() {
            assert_approx_vec_eq!(
                $vec_lowercase!($values_str).normalize_or_zero(),
                $vec_lowercase!($(for i in 0..n.as_usize() join(, ) => $(float_lit(values[i] / values.iter().map(|&x| x * x).sum::<f64>().sqrt(), primitive))))
            );
            assert_approx_vec_eq!(
                $vec_lowercase!($values2_with_nan_str).normalize_or_zero(),
                $vec_camelcase::<$primitive>::ZERO
            );
            assert_approx_vec_eq!(
                $vec_camelcase::<$primitive>::ZERO.normalize_or_zero(),
                $vec_camelcase::<$primitive>::ZERO
            );
        }
    });
}

fn float_lit(value: f64, primitive: PrimitiveFloat) -> String {
    if value.is_nan() {
        format!("{primitive}::NAN")
    } else if value.is_infinite() {
        if value.is_sign_positive() {
            format!("{primitive}::INFINITY")
        } else {
            format!("{primitive}::NEG_INFINITY")
        }
    } else {
        let mut lit = format!("{value:.4}").trim_end_matches("0").to_string();
        if lit.ends_with(".") {
            lit += "0";
        }

        format!("{}{primitive}", lit)
    }
}