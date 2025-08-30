use std::fmt::Write;

use const_format::formatcp;
use indoc::writedoc;

mod module;
use module::*;

mod primitives;
mod vector;

const ROOT_DIR: &str = formatcp!("{}/..", env!("CARGO_MANIFEST_DIR"));
const OUT_DIR: &str = formatcp!("{ROOT_DIR}/src/generated");

fn main() {
    let mut module = ModDir::root();

    vector::write_mod(module.submod_dir("vector"));
    primitives::write_mod(module.submod_dir("primitives"));

    writedoc!(
        module,
        r#"
        mod primitives;
        #[allow(unused_imports)]
        pub use primitives::*;
        "#
    )
    .unwrap();
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
