use indoc::formatdoc;

pub fn push_fns(
    _primitive: &str,
    functions: &mut Vec<String>,
    const_functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _std_const_functions: &mut Vec<String>,
    _test_functions: &mut Vec<String>,
) {
    for primitive2 in [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
    ] {
        functions.push(formatdoc! {r#"
            /// Converts `self` to a vector of `{primitive2}` elements.
            #[inline(always)]
            pub fn as_{primitive2}(self) -> Vector<N, {primitive2}, A> {{
                self.map(|x| x as {primitive2})
            }}
        "#});

        const_functions.push(formatdoc! {r#"
            /// Version of `Vector::as_{primitive2}` that can be called from const contexts.
            /// This version may be less performant than the normal version.
            /// 
            /// When rust's const capabilities are expanded, this function will be removed.
            #[inline(always)]
            pub const fn const_as_{primitive2}(self) -> Vector<N, {primitive2}, A> {{
                let mut output = Vector::<N, {primitive2}, A>::GARBAGE;
                let mut i = 0;
                while i < N {{
                    output.as_array_mut()[i] = self.as_array()[i] as {primitive2};
                    i += 1;
                }}
                output
            }}
        "#});
    }
}
