use super::*;

pub fn write_mod(mut module: Mod) {
    writedoc!(
        module,
        r#"
        /// Constructs a quaternion from scalars/vectors, like in shaders.
        /// This works like the `vec4!` macro, which expects a length of `4`.
        ///
        /// ### Example
        ///
        /// ```
        /// use ggmath::*;
        ///
        /// let q = quat!(1.0, vec2!(2.0, 3.0), 4.0);
        /// ```
        #[macro_export]
        macro_rules! quat {{
            ($($tt:tt)*) => {{
                $crate::Quat::from_vec($crate::vec4!($($tt)*))
            }};
        }}

        /// Constructs a packed quaternion from scalars/vectors, like in shaders.
        /// This works like the `vec4p!` macro, which expects a length of `4`.
        ///
        /// ### Example
        ///
        /// ```
        /// use ggmath::*;
        ///
        /// let q = quatp!(1.0, vec2!(2.0, 3.0), 4.0);
        /// ```
        #[macro_export]
        macro_rules! quatp {{
            ($($tt:tt)*) => {{
                $crate::QuatP::from_vec($crate::vec4p!($($tt)*))
            }};
        }}

        /// The equivalent of `vec4g!` macro, but for quaternions.
        #[macro_export]
        macro_rules! quatg {{
            ($($tt:tt)*) => {{
                $crate::Quaternion::from_vec($crate::vec4g!($($tt)*))
            }};
        }}
        "#
    )
    .unwrap();
}
