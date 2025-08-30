use std::fmt::Write;

use indoc::writedoc;

use crate::module::*;

mod dir;
mod swizzle;

pub fn write_mod(mut module: ModDir) {
    swizzle::write_mod(module.submod("swizzle"));
    dir::write_mod(module.submod("dir"));

    writedoc!(
        module,
        r#"
        mod swizzle;

        #[cfg(any(
            feature = "right",
            feature = "backwards",
            feature = "down",
            feature = "forwards",
            feature = "left",
            feature = "up"
        ))]
        mod dir;
        #[cfg(any(
            feature = "right",
            feature = "backwards",
            feature = "down",
            feature = "forwards",
            feature = "left",
            feature = "up"
        ))]
        pub use dir::*;
        "#
    )
    .unwrap();
}
