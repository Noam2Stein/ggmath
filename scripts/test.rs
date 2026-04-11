#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
colored = "3.1.1"
itertools = "0.14.0"
pretty-duration = "0.1.1"
---

use std::{
    env::args,
    process::{Command, Stdio},
    time::Instant,
};

use colored::Colorize;
use itertools::iproduct;
use pretty_duration::pretty_duration;

const THIRD_PARTY_CRATES: &str = "bytemuck fixed fixp mint rand serde wide";

fn main() {
    let mut commands = Vec::new();

    for (target, backend, release_mode, assertions, overflow_checks) in iproduct!(
        ["x86_64-unknown-linux-gnu", "riscv64gc-unknown-linux-gnu"],
        [Some("std"), Some("libm"), None],
        [false, true],
        [false, true],
        [false, true]
    ) {
        let third_party_crates = !release_mode;

        commands.push(cargo_command(
            "clippy",
            &[],
            Some(target),
            backend,
            release_mode,
            assertions,
            overflow_checks,
            third_party_crates,
        ));
        commands.push(cargo_command(
            "doc",
            &["--no-deps"],
            Some(target),
            backend,
            release_mode,
            assertions,
            overflow_checks,
            third_party_crates,
        ));
    }

    for (backend, release_mode, overflow_checks) in
        iproduct!(["std", "libm"], [false, true], [false, true])
    {
        let assertions = release_mode;
        let third_party_crates = backend == "std";

        commands.push(cargo_command(
            "test",
            &[],
            None,
            Some(backend),
            release_mode,
            assertions,
            overflow_checks,
            third_party_crates,
        ));
    }

    run_commands(commands);
}

fn cargo_command(
    cargo_command: &str,
    cargo_command_args: &[&str],
    target: Option<&str>,
    backend: Option<&str>,
    release_mode: bool,
    assertions: bool,
    overflow_checks: bool,
    third_party_crates: bool,
) -> Command {
    let mut command = Command::new("cargo");
    let mut rustflags = String::new();
    let mut features = String::new();

    command.arg(cargo_command);
    command.args(cargo_command_args);

    if let Some(backend) = backend {
        features += " ";
        features += backend;
    }

    if release_mode {
        command.arg("--release");
    }

    if assertions {
        features += " assertions";
    } else {
        features += " no-assertions";
    }

    if overflow_checks {
        rustflags += " -C overflow-checks=on";
    } else {
        rustflags += " -C overflow-checks=off";
    }

    if third_party_crates {
        features += " ";
        features += THIRD_PARTY_CRATES;
    }

    let mut allow_warnings = false;
    for arg in args().skip(1) {
        match arg.as_str() {
            "--allow-warnings" => allow_warnings = true,
            _ => panic!("unexpected argument"),
        }
    }

    if !allow_warnings {
        rustflags += " -D warnings";
    }

    if !rustflags.is_empty() {
        command.env("RUSTFLAGS", rustflags.trim());
    }

    if !features.is_empty() {
        command.arg("--features").arg(features.trim());
    }

    if let Some(target) = target {
        command.arg("--target").arg(target);
    }

    command
}

fn run_commands(commands: Vec<Command>) {
    let start_instant = Instant::now();

    let total_command_count = commands.len();
    let mut completed_command_count = 0;

    for mut command in commands {
        let command_str = format!("{}", std::fmt::from_fn(|f| format_command(&command, f)));

        println!();
        println!("Commands: {completed_command_count}/{total_command_count}");
        println!("{command_str}");
        println!();

        let command_output = command
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run command");

        if !command_output.status.success() {
            println!();
            println!("Commands: {completed_command_count}/{total_command_count}");
            println!("{command_str}");
            println!();
            println!("{}: warnings are denied by default", "note".bold());
            println!(
                "{}: to allow warnings add `--allow-warnings`",
                "help".bold()
            );
            println!();
            println!("{}: command failed", "error".red().bold());
            return;
        }

        completed_command_count += 1;
    }

    let time = pretty_duration(&Instant::now().duration_since(start_instant), None);

    println!();
    println!("Commands: {completed_command_count}/{total_command_count}");
    println!("Time: {time:?}");
    println!("{}", "GG! testing passed".green().bold());
}

fn format_command(command: &Command, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (key, value) in command.get_envs() {
        let key = key.to_str().expect("invalid utf8");
        let value = value
            .expect("do not remove environment variables")
            .to_str()
            .expect("invalid utf8");

        if value.contains(' ') {
            write!(f, "{key}=\"{value}\" ")?;
        } else {
            write!(f, "{key}={value} ")?;
        }
    }

    write!(f, "{}", command.get_program().display())?;

    for arg in command.get_args() {
        let arg = arg.to_str().expect("invalid utf8");

        if arg.contains(' ') {
            write!(f, " \"{arg}\"")?;
        } else {
            write!(f, " {arg}")?;
        }
    }

    Ok(())
}
