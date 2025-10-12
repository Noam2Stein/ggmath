mod constructor;
mod deref;
mod dir;
mod ops;
mod primitive_api;
mod swizzle;

pub fn generate() {
    constructor::generate();
    deref::generate();
    dir::generate();
    ops::generate();
    primitive_api::generate();
    swizzle::generate();
}
