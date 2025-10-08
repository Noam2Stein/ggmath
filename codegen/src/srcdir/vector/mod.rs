mod constructor;
mod dir;
mod ops;

pub fn generate() {
    constructor::generate();
    dir::generate();
    ops::generate();
}
