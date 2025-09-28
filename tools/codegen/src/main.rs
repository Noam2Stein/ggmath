mod backend;
mod code;
mod iter;

fn main() {
    code::srcmod().export_as_root();
    code::testmod().export_as_root();
}
