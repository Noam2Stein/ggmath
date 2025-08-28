use std::fmt::Write;

use indoc::writedoc;

use crate::module::*;

mod swizzle;

pub fn write_mod(mut module: ModDir) {
    swizzle::write_mod(module.submod("swizzle"));

    writedoc!(
        module,
        r#"
        mod swizzle;
        "#
    )
    .unwrap();
}
