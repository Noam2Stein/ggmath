mod iter;
mod module;
mod srcdir;
mod testdir;
mod util;

fn main() {
    srcdir::srcmod().export_as_root();
    testdir::testmod().export_as_root();
}
