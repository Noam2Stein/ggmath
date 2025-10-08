mod iter;
mod util;

mod srcdir;
mod testsdir;

fn main() {
    srcdir::generate();
    testsdir::generate();
}
