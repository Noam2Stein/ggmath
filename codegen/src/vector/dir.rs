

use crate::{
    constants::{COMPONENTS, LENGTHS},
    module::*,
};

pub fn mod_() -> ModFile {
    let mut mods = Vec::new();

    for (axis_idx, &axis) in ["x", "y", "z"].iter().enumerate() {
        let (dir_a_lower, dir_a_upper, dir_a_camel) = match axis {
            "x" => ("right", "RIGHT", "Right"),
            "y" => ("up", "UP", "Up"),
            "z" => ("forwards", "FORWARD", "Forward"),
            _ => unreachable!(),
        };
        let (dir_b_lower, dir_b_upper, dir_b_camel) = match axis {
            "x" => ("left", "LEFT", "Left"),
            "y" => ("down", "DOWN", "Down"),
            "z" => ("backwards", "BACKWARD", "Backward"),
            _ => unreachable!(),
        };

        let mut vector_impls_a = Vec::new();
        let mut vector_impls_b = Vec::new();

        for n in 2.max(axis_idx + 1)..=4 {
            let axis_uppercase = COMPONENTS[axis_idx].to_uppercase();

            vector_impls_a.push(formatdoc! {r#"
                impl<T: ScalarZero + ScalarOne, A: VecAlignment> Positive{dir_a_camel} for Vector<{n}, T, A> {{
                    const {dir_a_upper}: Self = Self::{axis_uppercase};
                }}

                impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> Negative{dir_b_camel} for Vector<{n}, T, A> {{
                    const {dir_b_upper}: Self = Self::NEG_{axis_uppercase};
                }}
            "#});

            vector_impls_b.push(formatdoc! {r#"
                impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> Negative{dir_a_camel} for Vector<{n}, T, A> {{
                    const {dir_a_upper}: Self = Self::NEG_{axis_uppercase};
                }}

                impl<T: ScalarZero + ScalarOne, A: VecAlignment> Positive{dir_b_camel} for Vector<{n}, T, A> {{
                    const {dir_b_upper}: Self = Self::{axis_uppercase};
                }}
            "#});
        }

        let vector_impls_a = vector_impls_a.join("\n").replace("\n", "\n\t");
        let vector_impls_b = vector_impls_b.join("\n").replace("\n", "\n\t");

        mods.push(formatdoc! {
            r#"
            /// A module for traits with `{dir_a_upper}` and `{dir_b_upper}` constants,
            /// where {dir_a_lower} is the positive direction.
            #[cfg(feature = "{dir_a_lower}")]
            pub mod {dir_a_lower} {{
                use crate::{{
                    Construct,
                    ScalarZero,
                    ScalarOne,
                    ScalarNegOne,
                    VecAlignment,
                    Vector,
                }};

                /// A trait for a `{dir_a_upper}` constant where {dir_a_lower} is the positive direction.
                pub trait Positive{dir_a_camel}: Construct {{
                    /// A value that points {dir_a_lower} with magnitude `1` where {dir_a_lower} is the positive direction.
                    const {dir_a_upper}: Self;
                }}

                /// A trait for a `{dir_b_upper}` constant where {dir_a_lower} is the positive direction.
                pub trait Negative{dir_b_camel}: Construct {{
                    /// A value that points {dir_b_lower} with magnitude `1` where {dir_a_lower} is the positive direction.
                    const {dir_b_upper}: Self;
                }}

                impl<T: ScalarOne> Positive{dir_a_camel} for T {{
                    const {dir_a_upper}: Self = Self::ONE;
                }}
                
                impl<T: ScalarNegOne> Negative{dir_b_camel} for T {{
                    const {dir_b_upper}: Self = Self::NEG_ONE;
                }}

                {vector_impls_a}
            }}

            /// A module for vectors with `{dir_a_upper}` and `{dir_b_upper}` constants,
            /// where {dir_b_lower} is the positive direction.
            #[cfg(feature = "{dir_b_lower}")]
            pub mod {dir_b_lower} {{
                use crate::{{
                    Construct,
                    ScalarZero,
                    ScalarOne,
                    ScalarNegOne,
                    VecAlignment,
                    Vector,
                }};

                /// A trait for a `{dir_a_upper}` constant where {dir_b_lower} is the positive direction.
                pub trait Negative{dir_a_camel}: Construct {{
                    /// A value that points {dir_a_lower} with magnitude `1` where {dir_b_lower} is the positive direction.
                    const {dir_a_upper}: Self;
                }}

                /// A trait for a `{dir_b_upper}` constant where {dir_b_lower} is the positive direction.
                pub trait Positive{dir_b_camel}: Construct {{
                    /// A value that points {dir_b_lower} with magnitude `1` where {dir_b_lower} is the positive direction.
                    const {dir_b_upper}: Self;
                }}

                impl<T: ScalarOne> Positive{dir_b_camel} for T {{
                    const {dir_b_upper}: Self = Self::ONE;
                }}

                impl<T: ScalarNegOne> Negative{dir_a_camel} for T {{
                    const {dir_a_upper}: Self = Self::NEG_ONE;
                }}

                {vector_impls_b}
            }}
        "#});
    }

    let mods = mods.join("\n");

    let scalar_vector_zero_consts = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                /// A vec{n} of all `0`s.
                /// 
                /// This is only required because `Vector::from_array` doesn't support const contexts.
                /// When Rust's const context capabilities increase, this will be removed.
                const VEC{n}_ZERO: Vec{n}<Self>;
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    let scalar_vector_one_consts = LENGTHS
        .iter()
        .map(|&n| {
            let axis_consts = (0..n).map(|i| {
                let axis = COMPONENTS[i].to_uppercase();
                formatdoc! {r#"
                    /// A vec{n} that points to the positive `{axis}` direction with magnitude `1`.
                    const VEC{n}_{axis}: Vec{n}<Self>;
                "#}
            }).collect::<Vec<_>>().join("\n");

            formatdoc! {r#"
                /// A vec{n} of all `1`s.
                /// 
                /// This is only required because `Vector::from_array` doesn't support const contexts.
                /// When Rust's const context capabilities increase, this will be removed.
                const VEC{n}_ONE: Vec{n}<Self>;

                {axis_consts}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    let scalar_vector_neg_one_consts = LENGTHS
        .iter()
        .map(|&n| {
            let axis_consts = (0..n).map(|i| {
                let axis = COMPONENTS[i].to_uppercase();
                formatdoc! {r#"
                    /// A vec{n} that points to the negative `{axis}` direction with magnitude `1`.
                    const VEC{n}_NEG_{axis}: Vec{n}<Self>;
                "#}
            }).collect::<Vec<_>>().join("\n");

            formatdoc! {r#"
                /// A vec{n} of all `-1`s.
                /// 
                /// This is only required because `Vector::from_array` doesn't support const contexts.
                /// When Rust's const context capabilities increase, this will be removed.
                const VEC{n}_NEG_ONE: Vec{n}<Self>;

                {axis_consts}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    let zero_vector_match_arms = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                {n} => transmute_copy::<Vector<{n}, T, VecAligned>, Vector<N, T, A>>(T::VEC{n}_ZERO),
            "#}
        })
        .collect::<Vec<_>>()
        .join("");

    let one_vector_match_arms = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                {n} => transmute_copy::<Vector<{n}, T, VecAligned>, Vector<N, T, A>>(T::VEC{n}_ONE),
            "#}
        })
        .collect::<Vec<_>>()
        .join("");

    let neg_one_vector_match_arms = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                {n} => transmute_copy::<Vector<{n}, T, VecAligned>, Vector<N, T, A>>(T::VEC{n}_NEG_ONE),
            "#}
        })
        .collect::<Vec<_>>()
        .join("");

    let axis_vector_impls = LENGTHS
        .iter()
        .map(|&n| {
            let consts = 

            (0..n).map(|i| {
                let axis_uppercase = COMPONENTS[i].to_uppercase();

                formatdoc! {r#"
                    /// A vector that points to the positive `{axis_uppercase}` direction with magnitude `1`.
                    pub const {axis_uppercase}: Self = {{
                        unsafe {{
                            if A::IS_ALIGNED {{
                                return transmute_copy::<Vector<{n}, T, VecAligned>, Vector<{n}, T, A>>(T::VEC{n}_{axis_uppercase});
                            }} else {{
                                return transmute_copy::<Vector<{n}, T, VecPacked>, Vector<{n}, T, A>>(Vector([T::{axis_uppercase}; {n}]));
                            }}
                        }}

                        unreachable!("unusual vector type");
                    }};
                "#}
            }).collect::<Vec<_>>().join("\n");

            formatdoc! {r#"
                impl<T: ScalarOne, A: VecAlignment> Vector<{n}, T, A> {{
                    {consts}
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    let neg_axis_vector_impls = LENGTHS
        .iter()
        .map(|&n| {
            let consts = 

            (0..n).map(|i| {
                let axis_uppercase = COMPONENTS[i].to_uppercase();

                formatdoc! {r#"
                    /// A vector that points to the negative `{axis_uppercase}` direction with magnitude `1`.
                    pub const {axis_uppercase}: Self = {{
                        unsafe {{
                            if A::IS_ALIGNED {{
                                return transmute_copy::<Vector<{n}, T, VecAligned>, Vector<{n}, T, A>>(T::VEC{n}_NEG_{axis_uppercase});
                            }} else {{
                                return transmute_copy::<Vector<{n}, T, VecPacked>, Vector<{n}, T, A>>(Vector([T::NEG_{axis_uppercase}; {n}]));
                            }}
                        }}

                        unreachable!("unusual vector type");
                    }};
                "#}
            }).collect::<Vec<_>>().join("");

            formatdoc! {r#"
                impl<T: ScalarNegOne, A: VecAlignment> Vector<{n}, T, A> {{
                    {consts}
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    let use_vecs = LENGTHS
        .iter()
        .map(|&n| format!("Vec{n}"))
        .collect::<Vec<_>>()
        .join(", ");

    ModFile::new(
        "dir",
        formatdoc! {r#"
            use core::mem::transmute_copy;

            use crate::{{
                Usize,
                vector::{{Scalar, VecAlignment, VecAligned, VecPacked, VecLen, Vector, {use_vecs}}},
            }};

            /// A trait for scalar types that have a `0` value.
            ///
            /// This trait along with `ScalarOne` and `ScalarNegOne`
            /// automatically enables direction constants like `RIGHT` if positive-direction features are enabled.
            pub trait ScalarZero: Scalar {{
                /// The zero value of the scalar type.
                const ZERO: Self;

                {scalar_vector_zero_consts}
            }}

            /// A trait for scalar types that have a `1` value.
            ///
            /// This trait along with `ScalarZero` and `ScalarNegOne`
            /// automatically enables direction constants like `RIGHT` if positive-direction features are enabled.
            pub trait ScalarOne: ScalarZero {{
                /// The one value of the scalar type.
                const ONE: Self;

                {scalar_vector_one_consts}
            }}

            /// A trait for scalar types that have a `-1` value.
            ///
            /// This trait along with `ScalarZero` and `ScalarOne`
            /// automatically enables direction constants like `RIGHT` if positive-direction features are enabled.
            pub trait ScalarNegOne: ScalarZero {{
                /// The negative one value of the scalar type.
                const NEG_ONE: Self;

                {scalar_vector_neg_one_consts}
            }}

            impl<const N: usize, T: ScalarZero, A: VecAlignment> Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                /// A vector of all `0`s.
                pub const ZERO: Self = {{
                    unsafe {{
                        if A::IS_ALIGNED {{
                            match N {{
                                {zero_vector_match_arms}
                            }}
                        }} else {{
                            return transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(Vector([T::ZERO; N]));
                        }}
                    }}

                    unreachable!("unusual vector type");
                }};
            }}

            impl<const N: usize, T: ScalarOne, A: VecAlignment> Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                /// A vector of all `1`s.
                pub const ONE: Self = {{
                    unsafe {{
                        if A::IS_ALIGNED {{
                            match N {{
                                {one_vector_match_arms}
                            }}
                        }} else {{
                            return transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(Vector([T::ONE; N]));
                        }}
                    }}

                    unreachable!("unusual vector type");
                }};
            }}

            impl<const N: usize, T: ScalarNegOne, A: VecAlignment> Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                /// A vector of all `-1`s.
                pub const NEG_ONE: Self = {{
                    unsafe {{
                        if A::IS_ALIGNED {{
                            match N {{
                                {neg_one_vector_match_arms}
                            }}
                        }} else {{
                            return transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(Vector([T::NEG_ONE; N]));
                        }}
                    }}

                    unreachable!("unusual vector type");
                }};
            }}

            {axis_vector_impls}
            
            {neg_axis_vector_impls}

            {mods}
        "#},
    )
}
