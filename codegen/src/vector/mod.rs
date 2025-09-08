use indoc::formatdoc;

use crate::module::*;

mod dir;
mod ops;
mod primitives;
mod swizzle;
mod swizzle_mut;
mod swizzle_ref;
mod swizzle_set;
mod swizzle_with;

pub fn write_mod(module: ModDir) {
    swizzle::write_mod(module.submod("swizzle"));
    swizzle_ref::write_mod(module.submod("swizzle_ref"));
    swizzle_mut::write_mod(module.submod("swizzle_mut"));
    swizzle_with::write_mod(module.submod("swizzle_with"));
    swizzle_set::write_mod(module.submod("swizzle_set"));
    primitives::write_mod(module.submod_dir("primitives"));
    dir::write_mod(module.submod("dir"));
    ops::write_mod(module.submod("ops"));

    module.finish(formatdoc! {r#"
        mod swizzle;
        mod swizzle_ref;
        mod swizzle_mut;
        mod swizzle_with;
        mod swizzle_set;
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
