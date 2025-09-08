use std::{
    path::Path,
    process::{Command, Stdio},
};

use colored::Colorize;
use const_format::concatcp;

mod bench_verification;

const WORKSPACE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const CRITERION_DIR: &str = concatcp!(WORKSPACE_DIR, "/target/criterion");

fn main() {
    if Path::new(CRITERION_DIR).exists() {
        std::fs::remove_dir_all(CRITERION_DIR).unwrap();
    }

    let commands = collect_commands();

    for (i, command) in commands.iter().enumerate() {
        println!();
        println!(
            "Running command {}/{}: {}",
            i + 1,
            commands.len(),
            command.join(" ")
        );
        println!();

        run(&command[0], &command[1..]);
    }

    bench_verification::verify_bench_results();

    println!();
    println!("{}", "GG! ggmath is ready to cargo publish!".green().bold());
}

fn collect_commands() -> Vec<Vec<&'static str>> {
    let mut commands = Vec::new();

    commands.push(vec!["cargo", "build", "--no-default-features"]);
    for features in [
        // empty features
        vec!["aliases"],
        vec!["right"],
        vec!["left", "right", "up"],
        vec!["std"],
        // vector
        vec!["vector"],
        vec!["vector", "std"],
        vec!["vector", "aliases"],
        vec!["vector", "aliases", "std"],
        vec!["vector", "aliases", "primitive_aliases"],
        vec!["vector", "aliases", "primitive_aliases", "std"],
        vec!["vector", "right", "up", "forwards"],
        vec!["vector", "left", "down", "backwards"],
        vec![
            "vector",
            "right",
            "left",
            "up",
            "down",
            "forwards",
            "backwards",
            "std",
        ],
        // matrix
        vec!["matrix"],
        vec!["matrix", "std"],
        vec!["matrix", "aliases"],
        vec!["matrix", "aliases", "std"],
        vec!["matrix", "aliases", "primitive_aliases"],
        vec!["matrix", "aliases", "primitive_aliases", "std"],
        vec![
            "matrix",
            "right",
            "left",
            "up",
            "down",
            "forwards",
            "backwards",
            "std",
        ],
        // quaternion
        vec!["quaternion"],
        vec!["quaternion", "std"],
        vec!["quaternion", "aliases"],
        vec!["quaternion", "aliases", "std"],
        vec!["quaternion", "aliases", "primitive_aliases"],
        vec!["quaternion", "aliases", "primitive_aliases", "std"],
        vec![
            "quaternion",
            "right",
            "left",
            "up",
            "down",
            "forwards",
            "backwards",
            "std",
        ],
        // aabb
        vec!["aabb"],
        vec!["aabb", "std"],
        vec!["aabb", "aliases"],
        vec!["aabb", "aliases", "std"],
        vec!["aabb", "aliases", "primitive_aliases"],
        vec!["aabb", "aliases", "primitive_aliases", "std"],
        vec![
            "aabb",
            "right",
            "left",
            "up",
            "down",
            "forwards",
            "backwards",
            "std",
        ],
        // all
        vec!["vector", "matrix", "quaternion", "aabb"],
        vec!["vector", "matrix", "quaternion", "aabb", "std"],
        vec!["vector", "matrix", "quaternion", "aabb", "aliases"],
        vec!["vector", "matrix", "quaternion", "aabb", "aliases", "std"],
        vec![
            "vector",
            "matrix",
            "quaternion",
            "aabb",
            "aliases",
            "primitive_aliases",
        ],
        vec![
            "vector",
            "matrix",
            "quaternion",
            "aabb",
            "aliases",
            "primitive_aliases",
            "std",
        ],
        vec![
            "vector",
            "matrix",
            "quaternion",
            "aabb",
            "right",
            "left",
            "up",
            "down",
            "forwards",
            "backwards",
            "std",
        ],
    ] {
        commands.push(vec![
            "cargo",
            "build",
            "--no-default-features",
            "--features",
            features.join(" ").leak(),
        ]);

        commands.push(vec![
            "cargo",
            "build",
            "--no-default-features",
            "--features",
            features.join(" ").leak(),
            "--release",
        ]);
    }

    commands.push(vec!["cargo", "test"]);
    commands.push(vec!["cargo", "test", "--release"]);

    commands.push(vec!["cargo", "fmt", "--all"]);

    commands.push(vec!["cargo", "bench"]);

    commands
}

fn run(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .current_dir(WORKSPACE_DIR)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect(&"Failed to execute process".red().bold().to_string())
        .wait()
        .expect(&"Failed to wait on process".red().bold().to_string());

    if !status.success() {
        eprintln!("{}", "Command failed!".red().bold());
        std::process::exit(1);
    }
}
