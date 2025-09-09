use indoc::formatdoc;

pub fn push_fns(
    primitive: &str,
    functions: &mut Vec<String>,
    const_functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _std_const_functions: &mut Vec<String>,
    test_functions: &mut Vec<String>,
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
            #[cfg(debug_assertions)]
            assert!(min.const_le_mask(max).const_all_true(), "min must be less than or equal to max");

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

        /// Returns `self.dot(other)` and supports const contexts.
        #[inline(always)]
        pub const fn const_dot(self, other: Vector<N, {primitive}, impl VecAlignment>) -> {primitive} {{
            let mut output = 0 as {primitive};
            let mut i = 0;
            while i < N {{
                output += self.as_array()[i] * other.as_array()[i];
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

    for primitive2 in [
        "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
        "u128", "usize",
    ] {
        if primitive2 == primitive {
            continue;
        }

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

        let v0 = match primitive {
            "f32" | "f64" => "0.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "0",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "0",
            _ => panic!("Invalid primitive: {}", primitive),
        };
        let v1 = match primitive {
            "f32" | "f64" => "1.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "1",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "1",
            _ => panic!("Invalid primitive: {}", primitive),
        };
        let v2 = match primitive {
            "f32" | "f64" => "2.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "2",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "2",
            _ => panic!("Invalid primitive: {}", primitive),
        };
        let v3 = match primitive {
            "f32" | "f64" => "3.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "3",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "3",
            _ => panic!("Invalid primitive: {}", primitive),
        };
        let v4 = match primitive {
            "f32" | "f64" => "4.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "4",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "4",
            _ => panic!("Invalid primitive: {}", primitive),
        };
        let v5 = match primitive {
            "f32" | "f64" => "5.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "5",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "5",
            _ => panic!("Invalid primitive: {}", primitive),
        };
        let v6 = match primitive {
            "f32" | "f64" => "6.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "6",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "6",
            _ => panic!("Invalid primitive: {}", primitive),
        };
        let v7 = match primitive {
            "f32" | "f64" => "7.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "7",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "7",
            _ => panic!("Invalid primitive: {}", primitive),
        };
        let v8 = match primitive {
            "f32" | "f64" => "8.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "8",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "8",
            _ => panic!("Invalid primitive: {}", primitive),
        };
        let v9 = match primitive {
            "f32" | "f64" => "9.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "9",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "9",
            _ => panic!("Invalid primitive: {}", primitive),
        };
        let v10 = match primitive {
            "f32" | "f64" => "10.0",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "10",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "10",
            _ => panic!("Invalid primitive: {}", primitive),
        };

        test_functions.push(formatdoc! {r#"
            // These tests are generated for all primitive number types

            #[test]
            fn test_ops_{a_lower}() {{
                assert_eq!((vec2{a_postfix}!({v0}, {v1}) + vec2{a_postfix}!({v2}, {v3})).to_array(), [{v2}, {v4}]);
                assert_eq!((vec2{a_postfix}!({v5}, {v3}) - vec2{a_postfix}!({v2}, {v2})).to_array(), [{v3}, {v1}]);
                assert_eq!((vec2{a_postfix}!({v5}, {v3}) * vec2{a_postfix}!({v2}, {v3})).to_array(), [{v10}, {v9}]);
                assert_eq!((vec2{a_postfix}!({v5}, {v3}) / vec2{a_postfix}!({v2}, {v3})).to_array(), [{v2}, {v1}]);
                assert_eq!((vec2{a_postfix}!({v5}, {v3}) % vec2{a_postfix}!({v2}, {v3})).to_array(), [{v1}, {v0}]);
                assert_eq!((vec2{a_postfix}!({v5}, {v3}) * {v2}).to_array(), [{v10}, {v6}]);
                assert_eq!((vec2{a_postfix}!({v5}, {v3}) / {v2}).to_array(), [{v2}, {v1}]);
                assert_eq!((vec2{a_postfix}!({v5}, {v3}) % {v2}).to_array(), [{v1}, {v1}]);

                assert_eq!((vec3{a_postfix}!({v0}, {v1}, {v2}) + vec3{a_postfix}!({v3}, {v4}, {v5})).to_array(), [{v3}, {v5}, {v7}]);
                assert_eq!((vec3{a_postfix}!({v5}, {v3}, {v2}) - vec3{a_postfix}!({v2}, {v2}, {v2})).to_array(), [{v3}, {v1}, {v0}]);
                assert_eq!((vec3{a_postfix}!({v5}, {v3}, {v2}) * vec3{a_postfix}!({v2}, {v3}, {v4})).to_array(), [{v10}, {v9}, {v8}]);
                assert_eq!((vec3{a_postfix}!({v5}, {v3}, {v2}) / vec3{a_postfix}!({v2}, {v3}, {v4})).to_array(), [{v2}, {v1}, {v0}]);
                assert_eq!((vec3{a_postfix}!({v5}, {v3}, {v2}) % vec3{a_postfix}!({v2}, {v3}, {v4})).to_array(), [{v1}, {v0}, {v2}]);
                assert_eq!((vec3{a_postfix}!({v5}, {v3}, {v2}) * {v2}).to_array(), [{v10}, {v6}, {v4}]);
                assert_eq!((vec3{a_postfix}!({v5}, {v3}, {v2}) / {v2}).to_array(), [{v2}, {v1}, {v1}]);
                assert_eq!((vec3{a_postfix}!({v5}, {v3}, {v2}) % {v2}).to_array(), [{v1}, {v1}, {v0}]);

                assert_eq!((vec4{a_postfix}!({v0}, {v1}, {v2}, {v3}) + vec4{a_postfix}!({v4}, {v5}, {v6}, {v7})).to_array(), [{v4}, {v6}, {v8}, {v10}]);
                assert_eq!((vec4{a_postfix}!({v5}, {v3}, {v2}, {v1}) - vec4{a_postfix}!({v2}, {v2}, {v2}, {v2})).to_array(), [{v3}, {v1}, {v0}, {v1}]);
                assert_eq!((vec4{a_postfix}!({v5}, {v3}, {v2}, {v1}) * vec4{a_postfix}!({v2}, {v3}, {v4}, {v5})).to_array(), [{v10}, {v9}, {v8}, {v5}]);
                assert_eq!((vec4{a_postfix}!({v5}, {v3}, {v2}, {v1}) / vec4{a_postfix}!({v2}, {v3}, {v4}, {v5})).to_array(), [{v2}, {v1}, {v0}, {v1}]);
                assert_eq!((vec4{a_postfix}!({v5}, {v3}, {v2}, {v1}) % vec4{a_postfix}!({v2}, {v3}, {v4}, {v5})).to_array(), [{v1}, {v0}, {v2}, {v1}]);
                assert_eq!((vec4{a_postfix}!({v5}, {v3}, {v2}, {v1}) * {v2}).to_array(), [{v10}, {v6}, {v4}, {v2}]);
                assert_eq!((vec4{a_postfix}!({v5}, {v3}, {v2}, {v1}) / {v2}).to_array(), [{v2}, {v1}, {v1}, {v1}]);
                assert_eq!((vec4{a_postfix}!({v5}, {v3}, {v2}, {v1}) % {v2}).to_array(), [{v1}, {v1}, {v0}, {v1}]);
            }}
        "#});
    }
}
