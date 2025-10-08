mod constructor;
mod dir;
mod ops;
mod swizzle;

pub fn generate() {
    constructor::generate();
    dir::generate();
    ops::generate();
    swizzle::generate();
}
