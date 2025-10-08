mod constructor;
mod deref;
mod dir;
mod ops;
mod swizzle;

pub fn generate() {
    constructor::generate();
    deref::generate();
    dir::generate();
    ops::generate();
    swizzle::generate();
}
