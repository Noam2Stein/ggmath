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
        // The following items are generated for all int types

        /// Returns `-self` or `None` if there is an overflow.
        #[inline(always)]
        pub fn checked_neg(self) -> Option<Self> {{
            self.map(|x| x.checked_neg()).flatten()
        }}

        /// Returns `self + other` or `None` if there is an overflow.
        #[inline(always)]
        pub fn checked_add(self, other: Self) -> Option<Self> {{
            Vector::from_fn(|i| self[i].checked_add(other[i])).flatten()
        }}

        /// Returns `self - other` or `None` if there is an overflow.
        #[inline(always)]
        pub fn checked_sub(self, other: Self) -> Option<Self> {{
            Vector::from_fn(|i| self[i].checked_sub(other[i])).flatten()
        }}

        /// Returns `self * other` or `None` if there is an overflow.
        #[inline(always)]
        pub fn checked_mul(self, other: Self) -> Option<Self> {{
            Vector::from_fn(|i| self[i].checked_mul(other[i])).flatten()
        }}

        /// Returns `self / other` or `None` if there is an overflow.
        #[inline(always)]
        pub fn checked_div(self, other: Self) -> Option<Self> {{
            Vector::from_fn(|i| self[i].checked_div(other[i])).flatten()
        }}

        /// Returns `self % other` or `None` if there is an overflow.
        #[inline(always)]
        pub fn checked_rem(self, other: Self) -> Option<Self> {{
            Vector::from_fn(|i| self[i].checked_rem(other[i])).flatten()
        }}

        /// Returns `-self` with wrapping arithmetic.
        #[inline(always)]
        pub fn wrapping_neg(self) -> Self {{
            self.map(|x| x.wrapping_neg())
        }}

        /// Returns `self + other` with wrapping arithmetic.
        #[inline(always)]
        pub fn wrapping_add(self, other: Self) -> Self {{
            Vector::from_fn(|i| self[i].wrapping_add(other[i]))
        }}

        /// Returns `self - other` with wrapping arithmetic.
        #[inline(always)]
        pub fn wrapping_sub(self, other: Self) -> Self {{
            Vector::from_fn(|i| self[i].wrapping_sub(other[i]))
        }}

        /// Returns `self * other` with wrapping arithmetic.
        #[inline(always)]
        pub fn wrapping_mul(self, other: Self) -> Self {{
            Vector::from_fn(|i| self[i].wrapping_mul(other[i]))
        }}

        /// Returns `self / other` with wrapping arithmetic.
        #[inline(always)]
        pub fn wrapping_div(self, other: Self) -> Self {{
            Vector::from_fn(|i| self[i].wrapping_div(other[i]))
        }}

        /// Returns `self % other` with wrapping arithmetic.
        #[inline(always)]
        pub fn wrapping_rem(self, other: Self) -> Self {{
            Vector::from_fn(|i| self[i].wrapping_rem(other[i]))
        }}

        /// Returns `self + other` with saturating arithmetic.
        #[inline(always)]
        pub fn saturating_add(self, other: Self) -> Self {{
            Vector::from_fn(|i| self[i].saturating_add(other[i]))
        }}

        /// Returns `self - other` with saturating arithmetic.
        #[inline(always)]
        pub fn saturating_sub(self, other: Self) -> Self {{
            Vector::from_fn(|i| self[i].saturating_sub(other[i]))
        }}

        /// Returns `self * other` with saturating arithmetic.
        #[inline(always)]
        pub fn saturating_mul(self, other: Self) -> Self {{
            Vector::from_fn(|i| self[i].saturating_mul(other[i]))
        }}

        /// Returns `self / other` with saturating arithmetic.
        #[inline(always)]
        pub fn saturating_div(self, other: Self) -> Self {{
            Vector::from_fn(|i| self[i].saturating_div(other[i]))
        }}
    "#});

    const_functions.push(formatdoc! {r#"
        // The following items are generated for all int types

        /// Returns `!self` and supports const contexts.
        #[inline(always)]
        pub const fn const_not(mut self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = !self.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Returns `self << other` and supports const contexts.
        #[inline(always)]
        pub const fn const_shl(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i] << other.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Returns `self >> other` and supports const contexts.
        #[inline(always)]
        pub const fn const_shr(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i] >> other.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Returns `self & other` and supports const contexts.
        #[inline(always)]
        pub const fn const_bitand(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i] & other.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Returns `self | other` and supports const contexts.
        #[inline(always)]
        pub const fn const_bitor(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i] | other.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Returns `self ^ other` and supports const contexts.
        #[inline(always)]
        pub const fn const_bitxor(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i] ^ other.as_array()[i];
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::checked_neg` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_checked_neg(mut self) -> Option<Self> {{
            let mut i = 0;
            while i < N {{
                match self.as_array()[i].checked_neg() {{
                    Some(value) => self.as_array_mut()[i] = value,
                    None => return None,
                }};
                
                i += 1;
            }}

            Some(self)
        }}

        /// Version of `Vector::checked_add` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_checked_add(mut self, other: Self) -> Option<Self> {{
            let mut i = 0;
            while i < N {{
                match self.as_array()[i].checked_add(other.as_array()[i]) {{
                    Some(value) => self.as_array_mut()[i] = value,
                    None => return None,
                }};
                
                i += 1;
            }}

            Some(self)
        }}

        /// Version of `Vector::checked_sub` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_checked_sub(mut self, other: Self) -> Option<Self> {{
            let mut i = 0;
            while i < N {{
                match self.as_array()[i].checked_sub(other.as_array()[i]) {{
                    Some(value) => self.as_array_mut()[i] = value,
                    None => return None,
                }};

                i += 1;
            }}

            Some(self)
        }}

        /// Version of `Vector::checked_mul` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_checked_mul(mut self, other: Self) -> Option<Self> {{
            let mut i = 0;
            while i < N {{
                match self.as_array()[i].checked_mul(other.as_array()[i]) {{
                    Some(value) => self.as_array_mut()[i] = value,
                    None => return None,
                }};

                i += 1;
            }}

            Some(self)
        }}

        /// Version of `Vector::checked_div` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_checked_div(mut self, other: Self) -> Option<Self> {{
            let mut i = 0;
            while i < N {{
                match self.as_array()[i].checked_div(other.as_array()[i]) {{
                    Some(value) => self.as_array_mut()[i] = value,
                    None => return None,
                }};

                i += 1;
            }}

            Some(self)
        }}

        /// Version of `Vector::checked_rem` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_checked_rem(mut self, other: Self) -> Option<Self> {{
            let mut i = 0;
            while i < N {{
                match self.as_array()[i].checked_rem(other.as_array()[i]) {{
                    Some(value) => self.as_array_mut()[i] = value,
                    None => return None,
                }};

                i += 1;
            }}

            Some(self)
        }}

        /// Version of `Vector::wrapping_neg` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_wrapping_neg(mut self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].wrapping_neg();
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::wrapping_add` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_wrapping_add(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].wrapping_add(other.as_array()[i]);
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::wrapping_sub` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_wrapping_sub(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].wrapping_sub(other.as_array()[i]);
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::wrapping_mul` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_wrapping_mul(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].wrapping_mul(other.as_array()[i]);
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::wrapping_div` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_wrapping_div(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].wrapping_div(other.as_array()[i]);
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::wrapping_rem` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_wrapping_rem(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].wrapping_rem(other.as_array()[i]);
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::saturating_add` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_saturating_add(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].saturating_add(other.as_array()[i]);
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::saturating_sub` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_saturating_sub(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].saturating_sub(other.as_array()[i]);
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::saturating_mul` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_saturating_mul(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].saturating_mul(other.as_array()[i]);
                i += 1;
            }}

            self
        }}

        /// Version of `Vector::saturating_div` that can be called from const contexts.
        /// This version may be less performant than the normal version.
        ///
        /// When rust's const capabilities are expanded, this function will be removed.
        #[inline(always)]
        pub const fn const_saturating_div(mut self, other: Self) -> Self {{
            let mut i = 0;
            while i < N {{
                self.as_array_mut()[i] = self.as_array()[i].saturating_div(other.as_array()[i]);
                i += 1;
            }}

            self
        }}

        /// Returns `self.abs_diff(other)` and supports const contexts.
        #[inline(always)]
        pub const fn const_abs_diff(self, other: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
            self.const_max(other).const_sub(self.const_min(other))
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

        test_functions.push(formatdoc! {r#"
            // These tests are generated for all int types

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn test_add2_overflow_{a_lower}() {{
                assert_eq!(
                    (vec2{a_postfix}!({primitive}::MAX, 1) + vec2{a_postfix}!(1, 3)).to_array(),
                    [{primitive}::MIN, 4]
                );
            }}

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn test_add3_overflow_{a_lower}() {{
                assert_eq!(
                    (vec3{a_postfix}!({primitive}::MAX, 1, 3) + vec3{a_postfix}!(1, 3, 5)).to_array(),
                    [{primitive}::MIN, 4, 8]
                );
            }}

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn test_add4_overflow_{a_lower}() {{
                assert_eq!(
                    (vec4{a_postfix}!({primitive}::MAX, 1, 3, 5) + vec4{a_postfix}!(1, 3, 5, 7)).to_array(),
                    [{primitive}::MIN, 4, 8, 12]
                );
            }}

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn test_sub2_overflow_{a_lower}() {{
                assert_eq!(
                    (vec2{a_postfix}!({primitive}::MIN, 5) - vec2{a_postfix}!(1, 3)).to_array(),
                    [{primitive}::MAX, 2]
                );
            }}

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn test_sub3_overflow_{a_lower}() {{
                assert_eq!(
                    (vec3{a_postfix}!({primitive}::MIN, 5, 7) - vec3{a_postfix}!(1, 3, 5)).to_array(),
                    [{primitive}::MAX, 2, 2]
                );
            }}

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn test_sub4_overflow_{a_lower}() {{
                assert_eq!(
                    (vec4{a_postfix}!({primitive}::MIN, 5, 7, 9) - vec4{a_postfix}!(1, 3, 5, 7)).to_array(),
                    [{primitive}::MAX, 2, 2, 2]
                );
            }}

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn test_mul2_overflow_{a_lower}() {{
                assert_eq!(
                    (vec2{a_postfix}!({primitive}::MAX, 5) * vec2{a_postfix}!(2, 3)).to_array(),
                    [{primitive}::MAX * 2, 15]
                );
            }}

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn test_mul3_overflow_{a_lower}() {{
                assert_eq!(
                    (vec3{a_postfix}!({primitive}::MAX, 5, 7) * vec3{a_postfix}!(2, 3, 4)).to_array(),
                    [{primitive}::MAX * 2, 15, 28]
                );
            }}

            #[test]
            #[cfg_attr(debug_assertions, should_panic)]
            fn test_mul4_overflow_{a_lower}() {{
                assert_eq!(
                    (vec4{a_postfix}!({primitive}::MAX, 5, 7, 9) * vec4{a_postfix}!(2, 3, 4, 5)).to_array(),
                    [{primitive}::MAX * 2, 15, 28, 45]
                );
            }}

            #[test]
            #[should_panic]
            fn test_div2_by_zero_{a_lower}() {{
                println!("{{}}", vec2{a_postfix}!(3, 5) / vec2{a_postfix}!(0, 3));
            }}

            #[test]
            #[should_panic]
            fn test_div3_by_zero_{a_lower}() {{
                println!("{{}}", vec3{a_postfix}!(3, 5, 7) / vec3{a_postfix}!(0, 3, 5));
            }}

            #[test]
            #[should_panic]
            fn test_div4_by_zero_{a_lower}() {{
                println!("{{}}", vec4{a_postfix}!(3, 5, 7, 9) / vec4{a_postfix}!(0, 3, 5, 7));
            }}

            #[test]
            #[should_panic]
            fn test_rem2_by_zero_{a_lower}() {{
                println!("{{}}", vec2{a_postfix}!(3, 5) % vec2{a_postfix}!(0, 3));
            }}

            #[test]
            #[should_panic]
            fn test_rem3_by_zero_{a_lower}() {{
                println!("{{}}", vec3{a_postfix}!(3, 5, 7) % vec3{a_postfix}!(0, 3, 5));
            }}
            
            #[test]
            #[should_panic]
            fn test_rem4_by_zero_{a_lower}() {{
                println!("{{}}", vec4{a_postfix}!(3, 5, 7, 9) % vec4{a_postfix}!(0, 3, 5, 7));
            }}
        "#});
    }
}
