use std::fmt::Write;

use const_format::formatcp;
use indoc::writedoc;

mod module;
use module::*;

mod vector;

const ROOT_DIR: &str = formatcp!("{}/..", env!("CARGO_MANIFEST_DIR"));
const OUT_DIR: &str = formatcp!("{ROOT_DIR}/src/generated");

fn main() {
    let mut module = ModDir::root();

    vector::write_mod(module.submod_dir("vector"));

    writedoc!(
        module,
        r#"
        mod vector;
        "#
    )
    .unwrap();
}
