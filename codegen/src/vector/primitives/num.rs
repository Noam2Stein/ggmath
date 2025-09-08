use indoc::formatdoc;

pub fn push_fns(
    primitive: &str,
    functions: &mut Vec<String>,
    const_functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _std_const_functions: &mut Vec<String>,
) {
    functions.push(formatdoc! {r#"
        // The following items are generated for all number types

        /// A vector of all minimum values.
        pub const MIN: Self = Self::const_splat({primitive}::MIN);
        /// A vector of all maximum values.
        pub const MAX: Self = Self::const_splat({primitive}::MAX);
    "#});

    const_functions.push(formatdoc! {r#"
        // The following items are generated for all number types

        /// Returns `self + other` and supports const contexts.
        #[inline(always)]
        pub const fn const_add(mut self, other: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i] + other.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Returns `self - other` and supports const contexts.
        #[inline(always)]
        pub const fn const_sub(mut self, other: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i] - other.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Returns `self * other` and supports const contexts.
        #[inline(always)]
        pub const fn const_mul(mut self, other: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i] * other.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Returns `self / other` and supports const contexts.
        #[inline(always)]
        pub const fn const_div(mut self, other: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i] / other.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Returns `self % other` and supports const contexts.
        #[inline(always)]
        pub const fn const_rem(mut self, other: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i] % other.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Returns `self == other` and supports const contexts.
        #[inline(always)]
        pub const fn const_eq(self, other: Vector<N, {primitive}, impl VecAlignment>) -> bool {{
            let mut i = 0;
            while i < N {{
                if self.as_array()[i] != other.as_array()[i] {{
                    return false;
                }}
                i += 1;
            }}
            true
        }}

        /// Returns `self != other` and supports const contexts.
        #[inline(always)]
        pub const fn const_ne(self, other: Vector<N, {primitive}, impl VecAlignment>) -> bool {{
            let mut i = 0;
            while i < N {{
                if self.as_array()[i] != other.as_array()[i] {{
                    return true;
                }}
                i += 1;
            }}
            false
        }}

        /// Returns `self.eq_mask(other)` and supports const contexts.
        pub const fn const_eq_mask(
            self,
            other: Vector<N, {primitive}, impl VecAlignment>,
        ) -> Vector<N, bool, A> {{
            let mut output = Vector::const_splat(false);
            let mut i = 0;
            while i < N {{
                output.as_array_mut()[i] = self.as_array()[i] == other.as_array()[i];
                i += 1;
            }}
            output
        }}

        /// Returns `self.ne_mask(other)` and supports const contexts.
        pub const fn const_ne_mask(
            self,
            other: Vector<N, {primitive}, impl VecAlignment>,
        ) -> Vector<N, bool, A> {{
            let mut output = Vector::const_splat(false);
            let mut i = 0;
            while i < N {{
                output.as_array_mut()[i] = self.as_array()[i] != other.as_array()[i];
                i += 1;
            }}
            output
        }}

        /// Returns `self.lt_mask(other)` and supports const contexts.
        pub const fn const_lt_mask(
            self,
            other: Vector<N, {primitive}, impl VecAlignment>,
        ) -> Vector<N, bool, A> {{
            let mut output = Vector::const_splat(false);
            let mut i = 0;
            while i < N {{
                output.as_array_mut()[i] = self.as_array()[i] < other.as_array()[i];
                i += 1;
            }}
            output
        }}

        /// Returns `self.gt_mask(other)` and supports const contexts.
        pub const fn const_gt_mask(
            self,
            other: Vector<N, {primitive}, impl VecAlignment>,
        ) -> Vector<N, bool, A> {{
            let mut output = Vector::const_splat(false);
            let mut i = 0;
            while i < N {{
                output.as_array_mut()[i] = self.as_array()[i] > other.as_array()[i];
                i += 1;
            }}
            output
        }}

        /// Returns `self.le_mask(other)` and supports const contexts.
        pub const fn const_le_mask(
            self,
            other: Vector<N, {primitive}, impl VecAlignment>,
        ) -> Vector<N, bool, A> {{
            let mut output = Vector::const_splat(false);
            let mut i = 0;
            while i < N {{
                output.as_array_mut()[i] = self.as_array()[i] <= other.as_array()[i];
                i += 1;
            }}
            output
        }}

        /// Returns `self.ge_mask(other)` and supports const contexts.
        pub const fn const_ge_mask(
            self,
            other: Vector<N, {primitive}, impl VecAlignment>,
        ) -> Vector<N, bool, A> {{
            let mut output = Vector::const_splat(false);
            let mut i = 0;
            while i < N {{
                output.as_array_mut()[i] = self.as_array()[i] >= other.as_array()[i];
                i += 1;
            }}
            output
        }}

        /// Returns `self.min(other)` and supports const contexts.
        #[inline(always)]
        pub const fn const_min(mut self, other: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
            let mut i = 0;
            while i < N {{
                if other.as_array()[i] < self.as_array()[i] {{
                    self.as_array_mut()[i] = other.as_array()[i];
                }}  
                i += 1;
            }}
            self
        }}

        /// Returns `self.max(other)` and supports const contexts.
        #[inline(always)]
        pub const fn const_max(mut self, other: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
            let mut i = 0;
            while i < N {{
                if other.as_array()[i] > self.as_array()[i] {{
                    self.as_array_mut()[i] = other.as_array()[i];
                }}
                i += 1;
            }}
            self
        }}

        /// Returns `self.clamp(min, max)` and supports const contexts.
        #[inline(always)]
        pub const fn const_clamp(self, min: Vector<N, {primitive}, impl VecAlignment>, max: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
            self.const_min(max).const_max(min)
        }}

        /// Returns `self.sum()` and supports const contexts.
        #[inline(always)]
        pub const fn const_sum(self) -> {primitive} {{
            let mut output = 0 as {primitive};
            let mut i = 0;
            while i < N {{
                output += self.as_array()[i];
                i += 1;
            }}
            output
        }}

        /// Returns `self.product()` and supports const contexts.
        #[inline(always)]
        pub const fn const_product(self) -> {primitive} {{
            let mut output = 1 as {primitive};
            let mut i = 0;
            while i < N {{
                output *= self.as_array()[i];
                i += 1;
            }}
            output
        }}

        /// Returns `self.mag_sq()` and supports const contexts.
        #[inline(always)]
        pub const fn const_mag_sq(self) -> {primitive} {{
            let mut output = 0 as {primitive};

            let mut i = 0;
            while i < N {{
                output += self.as_array()[i] * self.as_array()[i];
                i += 1;
            }}

            output
        }}

        /// Returns `self.distance_sq(other)` and supports const contexts.
        #[inline(always)]
        pub const fn const_distance_sq(self, other: Vector<N, {primitive}, impl VecAlignment>) -> {primitive} {{
            self.const_abs_diff(other).const_mag_sq()
        }}
    "#});
}
