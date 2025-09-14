use indoc::formatdoc;

use crate::constants::{COMPONENTS, LENGTHS};

pub fn push_fns(
    primitive: &str,
    functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    trait_impls: &mut Vec<String>,
    _use_crate_items: &mut Vec<String>,
) {
    functions.push(formatdoc! {r#"
        // The following items are generated for all primitive int types

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

    let zero_vector_consts = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                const VEC{n}_ZERO: Vec{n}<{primitive}> = Vec{n}::const_from_array([0; {n}]);
            "#}
        })
        .collect::<Vec<_>>()
        .join("");

    let one_vector_consts = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                const VEC{n}_ONE: Vec{n}<{primitive}> = Vec{n}::const_from_array([1; {n}]);
            "#}
        })
        .collect::<Vec<_>>()
        .join("");

    let axis_vector_consts = LENGTHS
        .iter()
        .map(|&n| {
            (0..n).map(|i| {
                let component = COMPONENTS[i];

                let array_items = (0..n).map(|i2| {
                    if i2 == i {
                        "1"
                    } else {
                        "0"
                    }
                }).collect::<Vec<_>>().join(", ");

                formatdoc! {r#"
                    const VEC{n}_{component}: Vec{n}<{primitive}> = Vec{n}::const_from_array([{array_items}]);
                "#}
            }).collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n");

    trait_impls.push(formatdoc! {r#"
        impl ScalarZero for {primitive} {{
            const ZERO: Self = 0;

            {zero_vector_consts}
        }}

        impl ScalarOne for {primitive} {{
            const ONE: Self = 1;

            {one_vector_consts}

            {axis_vector_consts}
        }}
    "#});
}
