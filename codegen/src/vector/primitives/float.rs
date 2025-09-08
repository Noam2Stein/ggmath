use indoc::formatdoc;

pub fn push_fns(
    primitive: &str,
    functions: &mut Vec<String>,
    const_functions: &mut Vec<String>,
    std_functions: &mut Vec<String>,
    _std_const_functions: &mut Vec<String>,
) {
    functions.push(formatdoc! {r#"
        // The following items are generated for all float types

        /// A vector with all elements set to `NaN`.
        pub const NAN: Self = Self::const_splat({primitive}::NAN);
        /// A vector with all elements set to `Infinity`.
        pub const INFINITY: Self = Self::const_splat({primitive}::INFINITY);
        /// A vector with all elements set to `-Infinity`.
        pub const NEG_INFINITY: Self = Self::const_splat({primitive}::NEG_INFINITY);
        /// A vector with all elements set to `Epsilon`.
        pub const EPSILON: Self = Self::const_splat({primitive}::EPSILON);

        /// Returns a vector containing the signum of each element of `self`.
        /// Signum for each element is:
        /// - `1.0` if the element is positive or `+0.0`
        /// - `-1.0` if the element is negative `-0.0`
        #[inline(always)]
        pub fn signum(self) -> Self {{
            self.map(|x| x.signum())
        }}

        /// Returns a vector of bools with `true` for each element that has a negative sign, including `-0.0`.
        #[inline(always)]
        pub fn negative_sign_mask(self) -> Vector<N, bool, A> {{
            self.map(|x| x.is_sign_negative())
        }}

        /// Returns a vector of bools with `true` for each element that has a positive sign, including `+0.0`.
        #[inline(always)]
        pub fn positive_sign_mask(self) -> Vector<N, bool, A> {{
            self.map(|x| x.is_sign_positive())
        }}

        /// Returns a vector of bools with `true` for each element that is `NaN`.
        #[inline(always)]
        pub fn nan_mask(self) -> Vector<N, bool, A> {{
            self.map(|x| x.is_nan())
        }}

        /// Returns a vector of bools with `true` for each element that is finite.
        #[inline(always)]
        pub fn finite_mask(self) -> Vector<N, bool, A> {{
            self.map(|x| x.is_finite())
        }}

        /// Returns `true` if any element is `NaN`.
        #[inline(always)]
        pub fn is_nan(self) -> bool {{
            self.nan_mask().any_true()
        }}

        /// Returns `true` if all elements are finite.
        #[inline(always)]
        pub fn is_finite(self) -> bool {{
            self.finite_mask().all_true()
        }}

        /// Returns a vector with the same direction as `self`, but with a magnitude of `1`.
        /// If `self` is zero, the result is NaN.
        #[inline(always)]
        pub fn normalize(self) -> Self {{
            self / self.mag()
        }}

        /// Returns a vector with the same direction as `self`, but with a magnitude of `1`.
        /// If `self` is zero, `None` is returned.
        #[inline(always)]
        pub fn checked_normalize(self) -> Option<Self> {{
            let normalized = self.normalize();
            if normalized.is_finite() {{
                Some(normalized)
            }} else {{
                None
            }}
        }}

        /// Returns a vector with the same direction as `self`, but with a magnitude of `1`.
        #[inline(always)]
        pub fn normalize_or(self, default: Self) -> Self {{
            let normalized = self.normalize();
            if normalized.is_finite() {{
                normalized
            }} else {{
                default
            }}
        }}

        /// Returns a vector with the same direction as `self`, but with a magnitude of `1`.
        /// If `self` is zero, zero is returned.
        #[inline(always)]
        pub fn normalize_or_zero(self) -> Self {{
            self.normalize_or(Self::ZERO)
        }}

        /// Linearly interpolates between `self` and `other` based on the interpolation factor `t`,
        /// which is clamped to the range `[0.0, 1.0]`.
        /// 
        /// This function uses the "delta lerp" formula which is:
        /// `a + (b - a) * t`
        /// 
        /// This formula is more numerically stable and is usually faster than the "weighted lerp" formula:
        /// `a * (1.0 - t) + b * t`
        /// 
        /// The other formula can be used by calling `lerp_weighted`.
        /// It is useful when interpolating large values that are very far away from each other.
        #[inline(always)]
        pub fn lerp(self, other: Vector<N, {primitive}, impl VecAlignment>, t: {primitive}) -> Self {{
            self.lerp_unclamped(other, t.clamp(0.0, 1.0))
        }}
        
        /// Linearly interpolates between `self` and `other` based on the interpolation factor `t`,
        /// which is clamped to the range `[0.0, 1.0]`.
        /// 
        /// This function uses the "weighted lerp" formula which is:
        /// `a * (1.0 - t) + b * t`
        /// 
        /// This formula is usually worse than the "delta lerp" formula:
        /// `a + (b - a) * t`
        /// 
        /// This "weighted" formula is useful when interpolating large values that are very far away from each other.
        #[inline(always)]
        pub fn lerp_weighted(self, other: Vector<N, {primitive}, impl VecAlignment>, t: {primitive}) -> Self {{
            self.lerp_unclamped_weighted(other, t.clamp(0.0, 1.0))
        }}

        /// Linearly interpolates between `self` and `other` based on the interpolation factor `t`.
        /// If `t` is outside the range `[0.0, 1.0]`, the result is linearly extrapolated.
        /// 
        /// This function uses the "delta lerp" formula which is:
        /// `a + (b - a) * t`
        /// 
        /// This formula is more numerically stable and is usually faster than the "weighted lerp" formula:
        #[inline(always)]
        pub fn lerp_unclamped(self, other: Vector<N, {primitive}, impl VecAlignment>, t: {primitive}) -> Self {{
            self + (other - self) * t
        }}
        
        /// This "weighted" formula is useful when interpolating large values that are very far away from each other.
        #[inline(always)]
        pub fn lerp_unclamped_weighted(self, other: Vector<N, {primitive}, impl VecAlignment>, t: {primitive}) -> Self {{
            self * (1.0 - t) + other * t
        }}

        /// Moves `self` towards `target` by at most `max_delta`.
        #[inline(always)]
        pub fn move_towards(self, target: Vector<N, {primitive}, impl VecAlignment>, max_delta: {primitive}) -> Self {{
            let delta = target - self;
            let delta_mag = delta.mag();
            if delta_mag <= max_delta || delta_mag <= 1e-4 {{
                return target.to_storage();
            }}
            self + delta / delta_mag * max_delta
        }}
    "#});

    const_functions.push(formatdoc! {r#"
        // The following items are generated for all float types

        /// Returns `self.abs_diff(other)` and supports const contexts.
        #[inline(always)]
        pub const fn const_abs_diff(self, other: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
            self.const_sub(other).const_abs()
        }}

        /// Version of `Vector::negative_sign_mask` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        /// 
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_negative_sign_mask(self) -> Vector<N, bool, A> {{
            let mut output = Vector::const_splat(false);

            let mut i = 0;
            while i < N {{
                output.as_array_mut()[i] = self.as_array()[i].is_sign_negative();
                i += 1;
            }}

            output
        }}

        /// Version of `Vector::positive_sign_mask` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        /// 
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_positive_sign_mask(self) -> Vector<N, bool, A> {{
            let mut output = Vector::const_splat(false);

            let mut i = 0;
            while i < N {{
                output.as_array_mut()[i] = self.as_array()[i].is_sign_positive();
                i += 1;
            }}

            output
        }}

        /// Version of `Vector::nan_mask` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        /// 
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_nan_mask(self) -> Vector<N, bool, A> {{
            let mut output = Vector::const_splat(false);

            let mut i = 0;
            while i < N {{
                output.as_array_mut()[i] = self.as_array()[i].is_nan();
                i += 1;
            }}

            output
        }}

        /// Version of `Vector::finite_mask` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        /// 
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_finite_mask(self) -> Vector<N, bool, A> {{
            let mut output = Vector::const_splat(false);

            let mut i = 0;
            while i < N {{
                output.as_array_mut()[i] = self.as_array()[i].is_finite();
                i += 1;
            }}

            output
        }}

        /// Version of `Vector::is_nan` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        /// 
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_is_nan(self) -> bool {{
            self.const_nan_mask().const_any_true()
        }}

        /// Version of `Vector::is_finite` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        /// 
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_is_finite(self) -> bool {{
            self.const_finite_mask().const_all_true()
        }}
    "#});

    std_functions.push(formatdoc! {r#"
        // The following items are generated for all float types

        /// Returns a vector containing the square root of each element of `self`.
        #[inline(always)]
        pub fn sqrt(self) -> Self {{
            self.map(|x| x.sqrt())
        }}

        /// Returns a vector containing the rounded value of each element of `self`.
        #[inline(always)]
        pub fn round(self) -> Self {{
            self.map(|x| x.round())
        }}

        /// Returns a vector containing the floor value of each element of `self`.
        #[inline(always)]
        pub fn floor(self) -> Self {{
            self.map(|x| x.floor())
        }}

        /// Returns a vector containing the ceiling value of each element of `self`.
        #[inline(always)]
        pub fn ceil(self) -> Self {{
            self.map(|x| x.ceil())
        }}

        /// Returns a vector containing the truncated value of each element of `self`.
        #[inline(always)]
        pub fn trunc(self) -> Self {{
            self.map(|x| x.trunc())
        }}

        /// Returns a vector containing the fractional part of each element of `self` as `self - self.trunc()`.
        #[inline(always)]
        pub fn fract(self) -> Self {{
            self.map(|x| x.fract())
        }}

        /// Returns a vector containing the sine of each element of `self`.
        #[inline(always)]
        pub fn sin(self) -> Self {{
            self.map(|x| x.sin())
        }}

        /// Returns a vector containing the cosine of each element of `self`.
        #[inline(always)]
        pub fn cos(self) -> Self {{
            self.map(|x| x.cos())
        }}

        /// Returns a vector containing the tangent of each element of `self`.
        #[inline(always)]
        pub fn tan(self) -> Self {{
            self.map(|x| x.tan())
        }}

        /// Returns a vector containing the arcsine of each element of `self`.
        #[inline(always)]
        pub fn asin(self) -> Self {{
            self.map(|x| x.asin())
        }}

        /// Returns a vector containing the arccosine of each element of `self`.
        #[inline(always)]
        pub fn acos(self) -> Self {{
            self.map(|x| x.acos())
        }}

        /// Returns a vector containing the arctangent of each element of `self`.
        #[inline(always)]
        pub fn atan(self) -> Self {{
            self.map(|x| x.atan())
        }}

        /// Returns a vector containing the hyperbolic sine of each element of `self`.
        #[inline(always)]
        pub fn sinh(self) -> Self {{
            self.map(|x| x.sinh())
        }}

        /// Returns a vector containing the hyperbolic cosine of each element of `self`.
        #[inline(always)]
        pub fn cosh(self) -> Self {{
            self.map(|x| x.cosh())
        }}

        /// Returns a vector containing the hyperbolic tangent of each element of `self`.
        #[inline(always)]
        pub fn tanh(self) -> Self {{
            self.map(|x| x.tanh())
        }}

        /// Returns a vector containing the hyperbolic arclength sine of each element of `self`.
        #[inline(always)]
        pub fn asinh(self) -> Self {{
            self.map(|x| x.asinh())
        }}

        /// Returns a vector containing the hyperbolic arclength cosine of each element of `self`.
        #[inline(always)]
        pub fn acosh(self) -> Self {{
            self.map(|x| x.acosh())
        }}

        /// Returns a vector containing the hyperbolic arclength tangent of each element of `self`.
        #[inline(always)]
        pub fn atanh(self) -> Self {{
            self.map(|x| x.atanh())
        }}

        /// Returns the magnitude of `self`.
        #[inline(always)]
        pub fn mag(self) -> {primitive} {{
            self.mag_sq().sqrt()
        }}

        /// Returns the Euclidean distance between `self` and `other`.
        #[inline(always)]
        pub fn distance(self, other: Self) -> {primitive} {{
            self.distance_sq(other).sqrt()
        }}
    "#});
}
