use indoc::formatdoc;

use crate::constants::{COMPONENTS, LENGTHS};

pub fn push_fns(
    primitive: &str,
    functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    trait_impls: &mut Vec<String>,
    use_crate_items: &mut Vec<String>,
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

    let neg_one_vector_consts = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                const VEC{n}_NEG_ONE: Vec{n}<{primitive}> = Vec{n}::const_from_array([-1; {n}]);
            "#}
        })
        .collect::<Vec<_>>()
        .join("");

    let neg_axis_vector_consts = LENGTHS
        .iter()
        .map(|&n| {
            (0..n).map(|i| {
                let component = COMPONENTS[i];

                let array_items = (0..n).map(|i2| {
                    if i2 == i {
                        "-1"
                    } else {
                        "0"
                    }
                }).collect::<Vec<_>>().join(", ");

                formatdoc! {r#"
                    const VEC{n}_NEG_{component}: Vec{n}<{primitive}> = Vec{n}::const_from_array([{array_items}]);
                "#}
            }).collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n");

    trait_impls.push(formatdoc! {r#"
        impl ScalarNegOne for {primitive} {{
            const NEG_ONE: Self = -1;

            {neg_one_vector_consts}

            {neg_axis_vector_consts}
        }}
    "#});

    use_crate_items.push("ScalarNegOne".to_string());
}
