mod backend;
mod code;
mod iter;

fn main() {
    code::srcmod().write_as_root();
}
