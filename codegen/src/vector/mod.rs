use std::fmt::Write;

use indoc::writedoc;

use crate::module::*;

mod dir;
mod ops;
mod primitives;
mod swizzle;
mod transmute;

pub fn write_mod(mut module: ModDir) {
    swizzle::write_mod(module.submod("swizzle"));
    primitives::write_mod(module.submod_dir("primitives"));
    dir::write_mod(module.submod("dir"));
    transmute::write_mod(module.submod("transmute"));
    ops::write_mod(module.submod("ops"));

    writedoc!(
        module,
        r#"
        mod swizzle;
        mod primitives;
        mod transmute;
        mod typematch;
        mod ops;

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
