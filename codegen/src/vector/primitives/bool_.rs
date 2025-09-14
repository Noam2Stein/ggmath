

use crate::constants::INT_PRIMITIVES;

pub fn push_fns(
    _primitive: &str,
    functions: &mut Vec<String>,
    _std_functions: &mut Vec<String>,
    _trait_impls: &mut Vec<String>,
    _use_crate_items: &mut Vec<String>,
) {
    for primitive2 in INT_PRIMITIVES {
        functions.push(formatdoc! {r#"
            /// Converts `self` to a vector of `{primitive2}` elements.
            #[inline(always)]
            pub fn as_{primitive2}(self) -> Vector<N, {primitive2}, A> {{
                self.map(|x| x as {primitive2})
            }}
        "#});
    }
}
