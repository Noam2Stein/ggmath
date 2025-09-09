use indoc::formatdoc;

pub fn push_fns(
    _primitive: &str,
    functions: &mut Vec<String>,
    const_functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _std_const_functions: &mut Vec<String>,
    test_functions: &mut Vec<String>,
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

    for a in ["VecAligned", "VecPacked"] {
        let a_lower = match a {
            "VecAligned" => "aligned",
            "VecPacked" => "packed",
            _ => panic!("Unhandled alignment: {}", a),
        };
        let a_postfix = match a {
            "VecAligned" => "",
            "VecPacked" => "p",
            _ => panic!("Unhandled alignment: {}", a),
        };

        let [vneg5, vneg4, vneg3, vneg2, vneg1, v0, v1, v2, v3, v4, v5] = match _primitive {
            "f32" | "f64" => [
                "-5.0", "-4.0", "-3.0", "-2.0", "-1.0", "0.0", "1.0", "2.0", "3.0", "4.0", "5.0",
            ],
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => {
                ["-5", "-4", "-3", "-2", "-1", "0", "1", "2", "3", "4", "5"]
            }
            _ => panic!("Invalid primitive: {}", _primitive),
        };

        test_functions.push(formatdoc! {r#"
            // These tests are generated for all signed number types

            #[test]
            fn test_signed_ops_{a_lower}() {{
                assert_eq!((-vec2{a_postfix}!({v5}, {v4})).to_array(), [{vneg5}, {vneg4}]);
                assert_eq!((-vec3{a_postfix}!({v0}, {v1}, {v3})).to_array(), [{v0}, {vneg1}, {vneg3}]);
                assert_eq!((-vec4{a_postfix}!({v5}, {v4}, {v3}, {v2})).to_array(), [{vneg5}, {vneg4}, {vneg3}, {vneg2}]);
            }}
        "#});
    }
}
