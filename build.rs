use std::process::Command;

fn main() {
    codegen::codegen();

    Command::new("cargo").arg("fmt").status().unwrap();
}
