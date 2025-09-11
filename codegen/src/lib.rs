use indoc::formatdoc;

mod r#gen;
mod gen_mod;
use r#gen::*;
use gen_mod::*;

mod float_ext;
mod primitive_aliases;
mod vector;

const LENGTHS: &[usize] = &[2, 3, 4];
const COMPONENTS: &[&str] = &["x", "y", "z", "w"];
const COMPONENT_ORDINALS: &[&str] = &["1st", "2nd", "3rd", "4th"];
const PRIMITIVES: &[&str] = &[
    "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
    "usize", "bool",
];
const NUM_PRIMITIVES: &[&str] = &[
    "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
    "usize",
];
const INT_PRIMITIVES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
];
const FLOAT_PRIMITIVES: &[&str] = &["f32", "f64"];

pub fn codegen() {
    GenModDir::new(
        "lib",
        formatdoc! {r#"
        mod float_ext;
        pub use float_ext::*;

        #[cfg(feature = "vector")]
        pub(crate) mod vector;

        #[cfg(feature = "primitive_aliases")]
        mod primitive_aliases;
        #[cfg(feature = "primitive_aliases")]
        pub use primitive_aliases::*;
        "#},
        "",
        vec![],
        vec![],
    )
    .write_as_root();
}

fn join_and(iter: impl Iterator<Item = String>) -> String {
    let mut vec = iter.collect::<Vec<_>>();
    let last = vec.pop();

    if let Some(last) = last {
        if vec.is_empty() {
            last
        } else {
            format!("{} and {last}", vec.join(", "))
        }
    } else {
        String::new()
    }
}

#[expect(unused)]
fn join_or(iter: impl Iterator<Item = String>) -> String {
    let mut vec = iter.collect::<Vec<_>>();
    let last = vec.pop();

    if let Some(last) = last {
        if vec.is_empty() {
            last
        } else {
            format!("{} or {last}", vec.join(", "))
        }
    } else {
        String::new()
    }
}
