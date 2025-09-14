use indoc::formatdoc;

use crate::constants::NUM_PRIMITIVES;

pub fn push_fns(
    primitive: &str,
    functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _trait_impls: &mut Vec<String>,
) {
    functions.push(formatdoc! {r#"
        // The following items are generated for all primitive number types

        /// A vector of all minimum values.
        pub const MIN: Self = Self::const_splat({primitive}::MIN);
        /// A vector of all maximum values.
        pub const MAX: Self = Self::const_splat({primitive}::MAX);
    "#});

    for &primitive2 in NUM_PRIMITIVES {
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
    }
}
