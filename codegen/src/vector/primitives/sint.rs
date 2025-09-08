use indoc::formatdoc;

pub fn push_fns(
    _primitive: &str,
    functions: &mut Vec<String>,
    const_functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _std_const_functions: &mut Vec<String>,
) {
    functions.push(formatdoc! {r#"
        // The following items are generated for all signed int types

        /// Returns `-self` with saturating arithmetic.
        #[inline(always)]
        pub fn saturating_neg(self) -> Self {{
            Vector::from_fn(|i| self[i].saturating_neg())
        }}

        /// Returns a vector containing the signum of each element of `self`.
        /// Signum for each element is:
        /// - `1` if the element is positive
        /// - `-1` if the element is negative
        /// - `0` if the element is zero
        #[inline(always)]
        pub fn signum(self) -> Self {{
            self.map(|x| x.signum())
        }}
    "#});

    const_functions.push(formatdoc! {r#"
        // The following items are generated for all signed int types

        /// Version of `Vector::saturating_neg` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_saturating_neg(mut self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].saturating_neg();
                i += 1;
            }}

            self
        }}
    "#});
}
