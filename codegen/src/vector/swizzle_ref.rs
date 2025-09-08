use indoc::formatdoc;

use crate::{join_and, module::*};

const LENGTHS: &[usize] = &[2, 3, 4];
const COMPONENTS: &[&str] = &["x", "y", "z", "w"];
const COMPONENT_ORDINALS: &[&str] = &["1st", "2nd", "3rd", "4th"];

pub fn write_mod(module: Mod) {
    let mut vector_impls = Vec::new();

    for &n in LENGTHS {
        let mut functions = Vec::new();

        for start in 0..n {
            for len in 1..=n - start {
                let components = (start..start + len)
                    .map(|i| COMPONENTS[i])
                    .collect::<String>();

                let component_ordinals =
                    join_and((start..start + len).map(|i| COMPONENT_ORDINALS[i].to_string()));

                let component_list =
                    join_and((start..start + len).map(|i| format!("`{}`", COMPONENTS[i])));

                if len == 1 {
                    functions.push(formatdoc! {r#"
                        /// Returns a reference to the {component_list} ({component_ordinals}) component of `self`.
                        #[inline(always)]
                        pub const fn {components}_ref(&self) -> &T {{
                            &self.as_array()[{start}]
                        }}
                    "#});
                } else {
                    functions.push(formatdoc! {r#"
                        /// Returns a vector reference to the {component_list} ({component_ordinals}) components of `self`.
                        #[inline(always)]
                        pub const fn {components}_ref(&self) -> &Vector<{len}, T, VecPacked> {{
                            Vector::from_array_ref(unsafe {{ &*(self.as_ptr().add({start}) as *const [T; {len}]) }})
                        }}
                    "#});
                }
            }
        }

        let functions = functions.join("\n").replace("\n", "\n\t");

        vector_impls.push(formatdoc! {r#"
            impl<T: Scalar, A: VecAlignment> Vector<{n}, T, A> {{
                {functions}
            }}
        "#});
    }

    let vector_impls = vector_impls.join("\n");

    module.finish(formatdoc! {r#"
        use crate::vector::{{Scalar, VecAlignment, VecPacked, Vector}};

        {vector_impls}
    "#});
}
