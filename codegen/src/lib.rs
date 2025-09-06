use core::mem::forget;

use const_format::formatcp;

mod module;
use module::*;

mod primitive_aliases;
mod vector;

const ROOT_DIR: &str = formatcp!("{}/..", env!("CARGO_MANIFEST_DIR"));
const OUT_DIR: &str = formatcp!("{ROOT_DIR}/src/generated");

pub fn codegen() {
    std::fs::remove_dir_all(OUT_DIR).unwrap();

    let module = ModDir::root();

    vector::write_mod(module.submod_dir("vector"));
    primitive_aliases::write_mod(module.submod_dir("primitive_aliases"));

    forget(module);
}

fn join_and(iter: impl Iterator<Item = String>) -> String {
    let mut vec = iter.collect::<Vec<_>>();
    let last = vec.pop();

    if let Some(last) = last {
        if vec.is_empty() {
            last
        } else {
            format!("{} and {last}", vec.join(", "))
        }
    } else {
        String::new()
    }
}

#[expect(unused)]
fn join_or(iter: impl Iterator<Item = String>) -> String {
    let mut vec = iter.collect::<Vec<_>>();
    let last = vec.pop();

    if let Some(last) = last {
        if vec.is_empty() {
            last
        } else {
            format!("{} or {last}", vec.join(", "))
        }
    } else {
        String::new()
    }
}
