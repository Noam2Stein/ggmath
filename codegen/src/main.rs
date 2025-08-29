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
        mod vector;

        mod primitives;
        #[allow(unused_imports)]
        pub use primitives::*;
        "#
    )
    .unwrap();
}
