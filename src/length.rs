/// Marker type representing the length of a math type.
///
/// For simplicity, math types such as [`Vector`](crate::Vector) do not support
/// arbitrary lengths. Supporting types like `Vec10<T>` would quickly lead to
/// excessive API bloat. Instead, all math types must have a length of either 2,
/// 3, or 4.
///
/// This restriction is enforced in the type system by implementing the
/// [`SupportedLength`] marker trait for `Length<2>`, `Length<3>`, and
/// `Length<4>`.
///
/// # Usage
///
/// ```
/// use ggmath::{Length, SupportedLength};
///
/// struct MathType<const N: usize>
/// where
///     Length<N>: SupportedLength,
/// {
///     // ...
/// }
/// ```
pub struct Length<const N: usize>;

/// Marker trait restricting the length of math types.
///
/// For simplicity, math types such as [`Vector`](crate::Vector) do not support
/// arbitrary lengths. Supporting types like `Vec10<T>` would quickly lead to
/// excessive API bloat. Instead, all math types must have a length of either 2,
/// 3, or 4.
///
/// This restriction is enforced in the type system using the [`Length<N>`]
/// marker type. `SupportedLength` is implemented only for `Length<2>`,
/// `Length<3>`, and `Length<4>`.
///
/// # Usage
///
/// ```
/// use ggmath::{Length, SupportedLength};
///
/// struct MathType<const N: usize>
/// where
///     Length<N>: SupportedLength,
/// {
///     // ...
/// }
/// ```
#[expect(private_bounds)]
pub trait SupportedLength: Sealed {
    #[doc(hidden)]
    type Select<T2: Copy, T3: Copy, T4: Copy>: Copy;
}

impl SupportedLength for Length<2> {
    type Select<T2: Copy, T3: Copy, T4: Copy> = T2;
}

impl SupportedLength for Length<3> {
    type Select<T2: Copy, T3: Copy, T4: Copy> = T3;
}

impl SupportedLength for Length<4> {
    type Select<T2: Copy, T3: Copy, T4: Copy> = T4;
}

trait Sealed {}

impl Sealed for Length<2> {}
impl Sealed for Length<3> {}
impl Sealed for Length<4> {}
