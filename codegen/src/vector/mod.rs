use indoc::formatdoc;

use crate::gen_mod::*;

mod dir;
mod ops;
mod primitives;
mod swizzle;

pub fn write_mod(module: GenModDir) {
    swizzle::write_mod(module.submod("swizzle"));
    primitives::write_mod(module.submod_dir("primitives"));
    dir::write_mod(module.submod("dir"));
    ops::write_mod(module.submod("ops"));

    module.finish(formatdoc! {r#"
        mod swizzle;
        mod primitives;
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
    "#});
}
