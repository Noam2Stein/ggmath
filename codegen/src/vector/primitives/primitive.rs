use indoc::formatdoc;

pub fn push_fns(
    primitive: &str,
    _functions: &mut Vec<String>,
    const_functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _std_const_functions: &mut Vec<String>,
) {
    const_functions.push(formatdoc! {r#"
        // The following items are generated for all primitive types

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
    "#});
}
