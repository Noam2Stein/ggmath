use indoc::formatdoc;

pub fn push_fns(
    primitive: &str,
    _functions: &mut Vec<String>,
    const_functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _std_const_functions: &mut Vec<String>,
    test_functions: &mut Vec<String>,
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

    let [value0, value1, value2, value3] = match primitive {
        "f32" | "f64" => ["1.0", "2.0", "3.0", "4.0"],
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => ["1", "2", "3", "4"],
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => ["1", "2", "3", "4"],
        "bool" => ["true", "false", "true", "false"],
        _ => panic!("Invalid primitive: {}", primitive),
    };

    for a in ["VecAligned", "VecPacked"] {
        let a_lower = match a {
            "VecAligned" => "aligned",
            "VecPacked" => "packed",
            _ => panic!("Unhandled alignment: {}", a),
        };

        let a_is_aligned = match a {
            "VecAligned" => "true",
            "VecPacked" => "false",
            _ => panic!("Unhandled alignment: {}", a),
        };

        let a_postfix = match a {
            "VecAligned" => "",
            "VecPacked" => "p",
            _ => panic!("Unhandled alignment: {}", a),
        };

        let test_bin_op = match primitive {
            "f32" | "f64" => "+",
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "+",
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "+",
            "bool" => "&",
            _ => panic!("Invalid primitive: {}", primitive),
        };

        test_functions.push(formatdoc! {r#"
            // These tests are generated for all primitive types

            #[test]
            fn test_array_{a_lower}() {{
                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).to_array(),
                    [{value0}, {value1}]
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).to_array(),
                    [{value2}, {value3}, {value0}]
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).to_array(),
                    [{value1}, {value2}, {value3}, {value0}]
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).as_array(),
                    &[{value0}, {value1}]
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).as_array(),
                    &[{value2}, {value3}, {value0}]
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).as_array(),
                    &[{value1}, {value2}, {value3}, {value0}]
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).as_array_mut(),
                    &mut [{value0}, {value1}]
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).as_array_mut(),
                    &mut [{value2}, {value3}, {value0}]
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).as_array_mut(),
                    &mut [{value1}, {value2}, {value3}, {value0}]
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).as_ptr(),
                    [{value0}, {value1}].as_ptr()
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).as_ptr(),
                    [{value2}, {value3}, {value0}].as_ptr()
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).as_ptr(),
                    [{value1}, {value2}, {value3}, {value0}].as_ptr()
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).as_mut_ptr(),
                    [{value0}, {value1}].as_mut_ptr()
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).as_mut_ptr(),
                    [{value2}, {value3}, {value0}].as_mut_ptr()
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).as_mut_ptr(),
                    [{value1}, {value2}, {value3}, {value0}].as_mut_ptr()
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_fn(|i| [{value0}, {value1}][i]).to_array(),
                    [{value0}, {value1}]
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_fn(|i| [{value2}, {value3}, {value0}][i]).to_array(),
                    [{value2}, {value3}, {value0}]
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_fn(|i| [{value1}, {value2}, {value3}, {value0}][i]).to_array(),
                    [{value1}, {value2}, {value3}, {value0}]
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).map(|x| {{
                        let idx = [{value0}, {value1}].into_iter().position(|y| y == x).unwrap();
                        [{value0}, {value1}][idx]
                    }}).to_array(),
                    [{value0}, {value1}]
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).map(|x| {{
                        let idx = [{value2}, {value3}, {value0}].into_iter().position(|y| y == x).unwrap();
                        [{value2}, {value3}, {value0}][idx]
                    }}).to_array(),
                    [{value2}, {value3}, {value0}]
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).map(|x| {{
                        let idx = [{value1}, {value2}, {value3}, {value0}].into_iter().position(|y| y == x).unwrap();
                        [{value1}, {value2}, {value3}, {value0}][idx]
                    }}).to_array(),
                    [{value1}, {value2}, {value3}, {value0}]
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).fold(|x, y| x {test_bin_op} y),
                    {value0} {test_bin_op} {value1}
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).fold(|x, y| x {test_bin_op} y),
                    {value2} {test_bin_op} {value3} {test_bin_op} {value0}
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).fold(|x, y| x {test_bin_op} y),
                    {value1} {test_bin_op} {value2} {test_bin_op} {value3} {test_bin_op} {value0}
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).all(|x| x == {value0}),
                    false
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).all(|x| x == {value0}),
                    false
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).all(|x| x == {value0}),
                    false
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).any(|x| x == {value0}),
                    true
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).any(|x| x == {value0}),
                    true
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).any(|x| x == {value0}),
                    true
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).count(|x| x == {value0}),
                    1
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).count(|x| x == {value0}),
                    1
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).count(|x| x == {value0}),
                    1
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).len(),
                    2
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).len(),
                    3
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).len(),
                    4
                );
            }}

            #[test]
            fn test_splat_{a_lower}() {{
                assert_eq!(
                    Vector::<2, {primitive}, {a}>::splat({value0}).to_array(),
                    [{value0}; 2]
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::splat({value2}).to_array(),
                    [{value2}; 3]
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::splat({value1}).to_array(),
                    [{value1}; 4]
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::const_splat({value0}).to_array(),
                    [{value0}; 2]
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::const_splat({value2}).to_array(),
                    [{value2}; 3]
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::const_splat({value1}).to_array(),
                    [{value1}; 4]
                );
            }}
            
            #[test]
            fn test_storage_{a_lower}() {{
                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).align(),
                    Vector::<2, {primitive}, VecAligned>::from_array([{value0}, {value1}]),
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).align(),
                    Vector::<3, {primitive}, VecAligned>::from_array([{value2}, {value3}, {value0}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).align(),
                    Vector::<4, {primitive}, VecAligned>::from_array([{value1}, {value2}, {value3}, {value0}]),
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).pack(),
                    Vector::<2, {primitive}, VecPacked>::from_array([{value0}, {value1}]),
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).pack(),
                    Vector::<3, {primitive}, VecPacked>::from_array([{value2}, {value3}, {value0}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).pack(),
                    Vector::<4, {primitive}, VecPacked>::from_array([{value1}, {value2}, {value3}, {value0}]),
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).to_storage::<VecAligned>(),
                    Vector::<2, {primitive}, VecAligned>::from_array([{value0}, {value1}]),
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).to_storage::<VecAligned>(),
                    Vector::<3, {primitive}, VecAligned>::from_array([{value2}, {value3}, {value0}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).to_storage::<VecAligned>(),
                    Vector::<4, {primitive}, VecAligned>::from_array([{value1}, {value2}, {value3}, {value0}]),
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).to_storage::<VecPacked>(),
                    Vector::<2, {primitive}, VecPacked>::from_array([{value0}, {value1}]),
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).to_storage::<VecPacked>(),
                    Vector::<3, {primitive}, VecPacked>::from_array([{value2}, {value3}, {value0}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).to_storage::<VecPacked>(),
                    Vector::<4, {primitive}, VecPacked>::from_array([{value1}, {value2}, {value3}, {value0}]),
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).is_aligned(),
                    {a_is_aligned}
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).is_aligned(),
                    {a_is_aligned}
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).is_aligned(),
                    {a_is_aligned}
                );
            }}

            #[test]
            fn test_swizzle_{a_lower}() {{
                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).x(),
                    {value0}
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).y(),
                    {value3}
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).z(),
                    {value3}
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).yy(),
                    Vector::<2, {primitive}, {a}>::from_array([{value1}, {value1}]),
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).zy(),
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value3}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).xw(),
                    Vector::<2, {primitive}, {a}>::from_array([{value1}, {value0}]),
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).xyy(),
                    Vector::<3, {primitive}, {a}>::from_array([{value0}, {value1}, {value1}]),
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).yzy(),
                    Vector::<3, {primitive}, {a}>::from_array([{value3}, {value0}, {value3}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).wxy(),
                    Vector::<3, {primitive}, {a}>::from_array([{value0}, {value1}, {value2}]),
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).xxyy(),
                    Vector::<4, {primitive}, {a}>::from_array([{value0}, {value0}, {value1}, {value1}]),
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).yzyz(),
                    Vector::<4, {primitive}, {a}>::from_array([{value3}, {value0}, {value3}, {value0}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).wxyw(),
                    Vector::<4, {primitive}, {a}>::from_array([{value0}, {value1}, {value2}, {value0}]),
                );
            }}

            #[test]
            fn test_swizzle_ref_{a_lower}() {{
                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).x_ref(),
                    &{value0}
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).y_ref(),
                    &{value3}
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).z_ref(),
                    &{value3}
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).xy_ref(),
                    &Vector::<2, {primitive}, VecPacked>::from_array([{value0}, {value1}]),
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value0}, {value1}, {value2}]).yz_ref(),
                    &Vector::<2, {primitive}, VecPacked>::from_array([{value1}, {value2}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value0}, {value1}, {value2}, {value3}]).zw_ref(),
                    &Vector::<2, {primitive}, VecPacked>::from_array([{value2}, {value3}]),
                );

                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value0}, {value1}, {value2}]).xyz_ref(),
                    &Vector::<3, {primitive}, VecPacked>::from_array([{value0}, {value1}, {value2}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value0}, {value1}, {value2}, {value3}]).yzw_ref(),
                    &Vector::<3, {primitive}, VecPacked>::from_array([{value1}, {value2}, {value3}]),
                );

                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value0}, {value1}, {value2}, {value3}]).xyzw_ref(),
                    &Vector::<4, {primitive}, VecPacked>::from_array([{value0}, {value1}, {value2}, {value3}]),
                );
            }}

            #[test]
            fn test_swizzle_mut_{a_lower}() {{
                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).x_mut(),
                    &mut {value0}
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).y_mut(),
                    &mut {value3}
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).z_mut(),
                    &mut {value3}
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).x_y_mut(),
                    (&mut {value0}, &mut {value1})
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).xy_z_mut(),
                    (&mut Vector::<2, {primitive}, VecPacked>::from_array([{value2}, {value3}]), &mut {value0})
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).xy_zw_mut(),
                    (
                        &mut Vector::<2, {primitive}, VecPacked>::from_array([{value1}, {value2}]),
                        &mut Vector::<2, {primitive}, VecPacked>::from_array([{value3}, {value0}]),
                    ),
                );
            }}

            #[test]
            fn test_swizzle_with_{a_lower}() {{
                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).with_x({value2}),
                    Vector::<2, {primitive}, {a}>::from_array([{value2}, {value1}]),
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).with_y({value1}),
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value1}, {value0}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).with_z({value2}),
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value2}, {value0}]),
                );

                assert_eq!(
                    Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]).with_xy(vec2!({value2}, {value3})),
                    Vector::<2, {primitive}, {a}>::from_array([{value2}, {value3}]),
                );
                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]).with_zy(vec2!({value1}, {value2})),
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value2}, {value1}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).with_xw(vec2!({value2}, {value3})),
                    Vector::<4, {primitive}, {a}>::from_array([{value2}, {value1}, {value2}, {value3}]),
                );

                assert_eq!(
                    Vector::<3, {primitive}, {a}>::from_array([{value0}, {value1}, {value2}]).with_xzy(vec3!({value2}, {value3}, {value0})),
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value0}, {value3}]),
                );
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).with_ywx(vec3!({value2}, {value3}, {value0})),
                    Vector::<4, {primitive}, {a}>::from_array([{value0}, {value2}, {value3}, {value3}]),
                );
                
                assert_eq!(
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]).with_xywz(vec4!({value2}, {value3}, {value0}, {value1})),
                    Vector::<4, {primitive}, {a}>::from_array([{value2}, {value3}, {value1}, {value0}]),
                );
            }}

            #[test]
            fn test_swizzle_set_{a_lower}() {{
                assert_eq!(
                    {{
                        let mut vector = Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]);
                        vector.set_x({value2});
                        vector
                    }},
                    Vector::<2, {primitive}, {a}>::from_array([{value2}, {value1}]),
                );
                assert_eq!(
                    {{
                        let mut vector = Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]);
                        vector.set_y({value1});
                        vector
                    }},
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value1}, {value0}]),
                );
                assert_eq!(
                    {{
                        let mut vector = Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]);
                        vector.set_z({value2});
                        vector
                    }},
                    Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value2}, {value0}]),
                );

                assert_eq!(
                    {{
                        let mut vector = Vector::<2, {primitive}, {a}>::from_array([{value0}, {value1}]);
                        vector.set_xy(vec2!({value2}, {value3}));
                        vector
                    }},
                    Vector::<2, {primitive}, {a}>::from_array([{value2}, {value3}]),
                );
                assert_eq!(
                    {{
                        let mut vector = Vector::<3, {primitive}, {a}>::from_array([{value2}, {value3}, {value0}]);
                        vector.set_zy(vec2!({value1}, {value2}));
                        vector
                    }},
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value2}, {value1}]),
                );
                assert_eq!(
                    {{
                        let mut vector = Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]);
                        vector.set_xw(vec2!({value2}, {value3}));
                        vector
                    }},
                    Vector::<4, {primitive}, {a}>::from_array([{value2}, {value1}, {value2}, {value3}]),
                );

                assert_eq!(
                    {{
                        let mut vector = Vector::<3, {primitive}, {a}>::from_array([{value0}, {value1}, {value2}]);
                        vector.set_xzy(vec3!({value2}, {value3}, {value0}));
                        vector
                    }},
                    Vector::<3, {primitive}, {a}>::from_array([{value2}, {value0}, {value3}]),
                );
                assert_eq!(
                    {{
                        let mut vector = Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]);
                        vector.set_ywx(vec3!({value2}, {value3}, {value0}));
                        vector
                    }},
                    Vector::<4, {primitive}, {a}>::from_array([{value0}, {value2}, {value3}, {value3}]),
                );
                
                assert_eq!(
                    {{
                        let mut vector = Vector::<4, {primitive}, {a}>::from_array([{value1}, {value2}, {value3}, {value0}]);
                        vector.set_xywz(vec4!({value2}, {value3}, {value0}, {value1}));
                        vector
                    }},
                    Vector::<4, {primitive}, {a}>::from_array([{value2}, {value3}, {value1}, {value0}]),
                );
            }}

            #[test]
            fn test_constructor_{a_lower}() {{
                assert_eq!(vec2{a_postfix}!({value0}, {value1}).to_array(), [{value0}, {value1}]);
                assert_eq!(vec2{a_postfix}!(vec2{a_postfix}!({value0}, {value1})).to_array(), [{value0}, {value1}]);

                assert_eq!(vec3{a_postfix}!({value0}, {value1}, {value2}).to_array(), [{value0}, {value1}, {value2}]);
                assert_eq!(vec3{a_postfix}!({value0}, vec2{a_postfix}!({value1}, {value2})).to_array(), [{value0}, {value1}, {value2}]);
                assert_eq!(vec3{a_postfix}!(vec2{a_postfix}!({value0}, {value1}), {value2}).to_array(), [{value0}, {value1}, {value2}]);
                assert_eq!(vec3{a_postfix}!(vec3{a_postfix}!({value0}, {value1}, {value2})).to_array(), [{value0}, {value1}, {value2}]);

                assert_eq!(vec4{a_postfix}!({value0}, {value1}, {value2}, {value3}).to_array(), [{value0}, {value1}, {value2}, {value3}]);
                assert_eq!(vec4{a_postfix}!({value0}, {value1}, vec2{a_postfix}!({value2}, {value3})).to_array(), [{value0}, {value1}, {value2}, {value3}]);
                assert_eq!(vec4{a_postfix}!({value0}, vec2{a_postfix}!({value1}, {value2}), {value3}).to_array(), [{value0}, {value1}, {value2}, {value3}]);
                assert_eq!(vec4{a_postfix}!({value0}, vec3{a_postfix}!({value1}, {value2}, {value3})).to_array(), [{value0}, {value1}, {value2}, {value3}]);
                assert_eq!(vec4{a_postfix}!(vec2{a_postfix}!({value0}, {value1}), {value2}, {value3}).to_array(), [{value0}, {value1}, {value2}, {value3}]);
                assert_eq!(vec4{a_postfix}!(vec2{a_postfix}!({value0}, {value1}), vec2{a_postfix}!({value2}, {value3})).to_array(), [{value0}, {value1}, {value2}, {value3}]);
                assert_eq!(vec4{a_postfix}!(vec3{a_postfix}!({value0}, {value1}, {value2}), {value3}).to_array(), [{value0}, {value1}, {value2}, {value3}]);
                assert_eq!(vec4{a_postfix}!(vec4{a_postfix}!({value0}, {value1}, {value2}, {value3})).to_array(), [{value0}, {value1}, {value2}, {value3}]);
            }}
        "#});
    }
}
