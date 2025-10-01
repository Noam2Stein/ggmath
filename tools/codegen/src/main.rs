mod backend;
mod iter;
mod srcgen;
mod testgen;
mod util;

fn main() {
    srcgen::srcmod().export_as_root();
    testgen::testmod().export_as_root();
}
