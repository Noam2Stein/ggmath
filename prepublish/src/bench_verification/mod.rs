use std::{collections::HashMap, path::Path};

use colored::Colorize;
use serde::Deserialize;

use crate::CRITERION_DIR;

pub fn verify_bench_results() {
    if !Path::new(CRITERION_DIR).exists() {
        panic!("{}", "Criterion directory not found".red().bold());
    }

    let mut slow_fns = Vec::new();

    for entry in std::fs::read_dir(CRITERION_DIR)
        .unwrap()
        .map(|entry| entry.unwrap())
    {
        let bench_name = entry.file_name().to_str().unwrap().to_string();

        let subdirs = std::fs::read_dir(entry.path())
            .unwrap()
            .map(|entry| entry.unwrap())
            .map(|entry| {
                (
                    entry.file_name().to_str().unwrap().to_string(),
                    entry.path(),
                )
            })
            .collect::<HashMap<_, _>>();

        let ggmath_subdir = match subdirs.get("ggmath") {
            Some(subdir) => subdir,
            None => continue,
        };

        let ggmath_estimates = serde_json::from_str::<Estimates>(
            &std::fs::read_to_string(ggmath_subdir.join("new/estimates.json")).unwrap(),
        )
        .unwrap();

        let opposing_name = if subdirs.contains_key("glam") {
            "glam"
        } else {
            "wide"
        };

        let opposing_estimates = if subdirs.contains_key("glam") {
            serde_json::from_str::<Estimates>(
                &std::fs::read_to_string(subdirs.get("glam").unwrap().join("new/estimates.json"))
                    .unwrap(),
            )
            .unwrap()
        } else {
            serde_json::from_str::<Estimates>(
                &std::fs::read_to_string(subdirs.get("wide").unwrap().join("new/estimates.json"))
                    .unwrap(),
            )
            .unwrap()
        };

        if ggmath_estimates.mean.point_estimate * 0.9 > opposing_estimates.mean.point_estimate {
            slow_fns.push(SlowFn {
                name: bench_name,
                opposing_name: opposing_name,
                ggmath_estimate: ggmath_estimates.mean.point_estimate,
                opposing_estimate: opposing_estimates.mean.point_estimate,
            });
        }
    }

    if !slow_fns.is_empty() {
        panic!(
            "{}",
            format!(
                "ggmath is slower than the opposing implementation in:\n{}",
                slow_fns
                    .iter()
                    .map(|slow_fn| format!(
                        "{}: {} vs {} (against {})",
                        slow_fn.name,
                        slow_fn.ggmath_estimate,
                        slow_fn.opposing_estimate,
                        slow_fn.opposing_name
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
            .red()
            .bold()
        );
    }
}

#[derive(Debug, Deserialize)]
struct Estimate {
    point_estimate: f64,
}

#[derive(Debug, Deserialize)]
struct Estimates {
    mean: Estimate,
}

#[derive(Debug)]
struct SlowFn {
    name: String,
    opposing_name: &'static str,
    ggmath_estimate: f64,
    opposing_estimate: f64,
}
