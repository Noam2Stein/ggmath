use indoc::formatdoc;

pub fn push_fns(
    _primitive: &str,
    functions: &mut Vec<String>,
    const_functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _std_const_functions: &mut Vec<String>,
) {
    functions.push(formatdoc! {r#"
        // The following items are generated for all signed number types

        /// Returns a vector containing the absolute value of each element of `self`.
        #[inline(always)]
        pub fn abs(self) -> Self {{
            self.map(|x| x.abs())
        }}
    "#});

    const_functions.push(formatdoc! {r#"
        // The following items are generated for all signed number types

        /// Returns `-self` and supports const contexts.
        #[inline(always)]
        pub const fn const_neg(mut self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = -self.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::abs` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        /// 
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_abs(mut self) -> Self {{
            let mut i = 0;

            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].abs();
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::signum` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        /// 
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_signum(mut self) -> Self {{
            let mut i = 0;

            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].signum();
                i += 1;
            }}

            self
        }}
    "#});
}
