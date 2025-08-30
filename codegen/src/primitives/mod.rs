use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::module::*;

pub fn write_mod(mut module: ModDir) {
    let mut primitive_mods = Vec::new();

    for primitive in [
        "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
        "u128", "usize", "bool",
    ] {
        write_primitive_mod(module.submod(primitive), primitive);

        primitive_mods.push(formatdoc! {r#"
            mod {primitive};
            #[allow(unused_imports)]
            pub use {primitive}::*;
        "#});
    }

    let primitive_mods = primitive_mods.join("\n");

    writedoc!(
        module,
        r#"
        {primitive_mods}
    "#
    )
    .unwrap();
}

fn write_primitive_mod(mut module: Mod, primitive: &str) {
    let prefix = match primitive {
        "f32" => "F",
        "f64" => "D",
        "i8" => "I8",
        "i16" => "I16",
        "i32" => "I",
        "i64" => "I64",
        "i128" => "I128",
        "isize" => "Isize",
        "u8" => "U8",
        "u16" => "U16",
        "u32" => "U",
        "u64" => "U64",
        "u128" => "U128",
        "usize" => "Usize",
        "bool" => "B",
        _ => panic!("unhandled primitive in prefix: {primitive}"),
    };

    let mut functions = Vec::new();

    for &unary_op in {
        let unary_ops: &[&str] = match primitive {
            "f32" | "f64" => &["neg"],
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => &["not"],
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => &["neg", "not"],
            "bool" => &["not"],
            _ => panic!("unhandled primitive in unary operations: {primitive}"),
        };

        unary_ops
    } {
        let doc_op = match unary_op {
            "neg" => "Negation",
            "not" => "Bitwise NOT",
            _ => panic!("unhandled unary operation in doc: {unary_op}"),
        };

        let op_token = match unary_op {
            "neg" => "-",
            "not" => "!",
            _ => panic!("unhandled unary operation in op token: {unary_op}"),
        };

        functions.push(formatdoc! {r#"
            /// Applies {doc_op} to each element of the vector.
            /// 
            /// This function exists to allow vector {doc_op} to be used in const contexts.
            #[inline(always)]
            pub const fn {unary_op}(mut self) -> Self {{
                let mut i = 0;

                while i < N {{
                    self.as_array_mut()[i] = {op_token}self.as_array()[i];
                    i += 1;
                }}

                self
            }}
        "#});
    }

    for &bin_op in {
        let bin_ops: &[&str] = match primitive {
            "f32" | "f64" => &["add", "sub", "mul", "div", "rem"],
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => &[
                "add", "sub", "mul", "div", "rem", "and", "or", "xor", "shl", "shr",
            ],
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => &[
                "add", "sub", "mul", "div", "rem", "and", "or", "xor", "shl", "shr",
            ],
            "bool" => &["and", "or", "xor"],
            _ => panic!("unhandled primitive in binary operations: {primitive}"),
        };

        bin_ops
    } {
        let doc_op = match bin_op {
            "add" => "Addition",
            "sub" => "Subtraction",
            "mul" => "Multiplication",
            "div" => "Division",
            "rem" => "Remainder",
            "and" => "Bitwise AND",
            "or" => "Bitwise OR",
            "xor" => "Bitwise XOR",
            "shl" => "Left Shift",
            "shr" => "Right Shift",
            _ => panic!("unhandled binary operation in doc: {bin_op}"),
        };

        let op_token = match bin_op {
            "add" => "+",
            "sub" => "-",
            "mul" => "*",
            "div" => "/",
            "rem" => "%",
            "and" => "&",
            "or" => "|",
            "xor" => "^",
            "shl" => "<<",
            "shr" => ">>",
            _ => panic!("unhandled binary operation in op token: {bin_op}"),
        };

        functions.push(formatdoc! {r#"
            /// Applies {doc_op} to each element of the vector.
            /// 
            /// This function exists to allow vector {doc_op} to be used in const contexts.
            #[inline(always)]
            pub const fn {bin_op}(mut self, other: Vector<N, {primitive}, impl VecAlignment>) -> Self {{
                let mut i = 0;

                while i < N {{
                    self.as_array_mut()[i] = self.as_array()[i] {op_token} other.as_array()[i];
                    i += 1;
                }}

                self
            }}
        "#});
    }

    let functions = functions.join("\n").replace("\n", "\n\t");

    writedoc!(
        module,
        r#"
        use crate::{{vector::{{VecAlignment, Vector, VecLen}}, Usize}};

        /// A module with `{primitive}` type aliases.
        #[cfg(feature = "primitive_aliases")]
        pub mod {primitive}_aliases {{
            use crate::vector_aliases;

            vector_aliases!(pub {prefix} => {primitive});
        }}

        impl<const N: usize, A: VecAlignment> Vector<N, {primitive}, A>
        where
            Usize<N>: VecLen,
        {{
            {functions}
        }}
    "#
    )
    .unwrap();

    let primitive_is_number = primitive != "bool";
    let primitive_is_signed = match primitive {
        "f32" | "f64" => true,
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => true,
        _ => false,
    };

    if primitive_is_number {
        writeln!(module).unwrap();
        writedoc!(
            module,
            r#"
            impl crate::vector::ScalarZero for {primitive} {{
                const ZERO: {primitive} = 0 as Self;
            }}

            impl crate::vector::ScalarOne for {primitive} {{
                const ONE: {primitive} = 1 as Self;
            }}
            "#
        )
        .unwrap();
    }

    if primitive_is_signed {
        writeln!(module).unwrap();
        writedoc!(
            module,
            r#"
            impl crate::vector::ScalarNegOne for {primitive} {{
                const NEG_ONE: {primitive} = -1 as Self;
            }}
            "#
        )
        .unwrap();
    }
}
