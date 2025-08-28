use indoc::formatdoc;

use super::*;

pub fn write_mod(mut module: ModDir) {
    let type_aliases = LENGTHS.map(|columns| {
        LENGTHS.map(move |rows| {
            formatdoc! {r#"
                /// Type alias to [`Matrix<{columns}, {rows}, T, VecAligned, ColMajor>`].
                pub type Mat{columns}x{rows}C<T> = Matrix<{columns}, {rows}, T, VecAligned, ColMajor>;

                /// Type alias to [`Matrix<{columns}, {rows}, T, VecAligned, RowMajor>`].
                pub type Mat{columns}x{rows}R<T> = Matrix<{columns}, {rows}, T, VecAligned, RowMajor>;

                /// Type alias to [`Matrix<{columns}, {rows}, T, VecPacked, ColMajor>`].
                pub type Mat{columns}x{rows}CP<T> = Matrix<{columns}, {rows}, T, VecPacked, ColMajor>;

                /// Type alias to [`Matrix<{columns}, {rows}, T, VecPacked, RowMajor>`].
                pub type Mat{columns}x{rows}RP<T> = Matrix<{columns}, {rows}, T, VecPacked, RowMajor>;
            "#}
        })
    }).flatten().collect::<Vec<_>>()
        .join("\n");

    writedoc!(
        module,
        r#"
        //! Module for the matrix type.

        use super::*;

        mod convert;
        mod index;
        mod construct;
        mod splat;
        mod ops;
        mod swizzle;
        pub use construct::*;
        pub use splat::*;

        /// The matrix type.
        ///
        /// This type has alot of generics, but in most cases you can use its type aliases instead.
        /// For example, [`Mat2C<T>`].
        ///
        /// This type is generic over columns, rows, scalar type, alignment and major axis.
        /// The first 4 match the generics of [`Vector`].
        ///
        /// The major axis is a marker type like `VecAlignment` types, which is either `ColMajor` or `RowMajor`.
        /// This only affects the memory representation of the matrix,
        /// and does not affect the outer API.
        ///
        /// `ggmath` matrix size specification is ordered columns then rows.
        /// `2x3` means 2 columns and 3 rows.
        #[derive_where(Clone, Copy)]
        pub struct Matrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
        where
            Usize<C>: VecLen,
            Usize<R>: VecLen,
        {{
            inner: M::InnerMatrix<C, R, T, A>,
        }}

        {type_aliases}
        "#,
    )
    .unwrap();
}
