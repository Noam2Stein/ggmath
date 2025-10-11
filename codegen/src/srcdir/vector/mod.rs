mod constructor;
mod deref;
mod dir;
mod ops;
mod primitives;
mod swizzle;

pub fn generate() {
    constructor::generate();
    deref::generate();
    dir::generate();
    ops::generate();
    primitives::generate();
    swizzle::generate();
}
