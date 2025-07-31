//! This tool displays what ggmath functions are slower than their glam counterparts.
//! This makes it easier to identify functions that need to be optimized.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use colored::*;
use serde_json::Value;
use walkdir::WalkDir;

#[derive(Debug)]
struct BenchmarkResult {
    mean_estimate_ns: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "ggmath benchmark analyser".bold().underline());
    println!();

    // Check paths
    let ggmath_dir = Path::new("../ggmath");
    if !ggmath_dir.exists() {
        return Err("ggmath directory not found".into());
    }

    let target_dir = Path::new("../ggmath/target");
    let full_target_path = target_dir
        .canonicalize()
        .unwrap_or_else(|_| target_dir.to_path_buf());
    println!("target: {}", full_target_path.display());

    println!("\nRunning benchmarks...");

    // Run cargo bench in the ggmath directory
    let output = Command::new("cargo")
        .args(&["bench"])
        .current_dir("../ggmath")
        .output()?;

    if !output.status.success() {
        eprintln!("❌ Failed to run cargo bench:");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("Benchmark execution failed".into());
    }

    // Parse criterion results from target/criterion
    let criterion_dir = Path::new("../ggmath/target/criterion");

    if !criterion_dir.exists() {
        return Err(
            "Criterion results directory not found. Make sure benchmarks ran successfully.".into(),
        );
    }

    let mut results = HashMap::<String, HashMap<String, BenchmarkResult>>::new();

    // Walk through criterion directory to find benchmark results

    for entry in WalkDir::new(criterion_dir) {
        let entry = entry?;
        if entry.file_name() == "estimates.json" {
            let path = entry.path();

            if let Some(benchmark_name) = extract_benchmark_name(path) {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(json) = serde_json::from_str::<Value>(&content) {
                        if let Some(mean) = json.get("mean").and_then(|m| m.get("point_estimate")) {
                            if let Some(mean_ns) = mean.as_f64() {
                                let parts: Vec<&str> = benchmark_name.split('/').collect();
                                if parts.len() == 2 {
                                    let group_name = parts[0].to_string();
                                    let function_name = parts[1].to_string();

                                    results
                                        .entry(group_name)
                                        .or_insert_with(HashMap::new)
                                        .insert(
                                            function_name,
                                            BenchmarkResult {
                                                mean_estimate_ns: mean_ns,
                                            },
                                        );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut all_comparisons = Vec::new();
    let mut slower_functions = Vec::new();

    // Compare ggmath vs glam for each benchmark group
    // Ignore tiny differences that are likely measurement noise
    const NOISE_THRESHOLD: f64 = 1.05; // 5% difference minimum
    const MIN_TIME_DIFF_NS: f64 = 0.05; // 0.05ns minimum absolute difference

    for (group_name, group_results) in &results {
        if let (Some(ggmath_result), Some(glam_result)) =
            (group_results.get("ggmath"), group_results.get("glam"))
        {
            let ggmath_time = ggmath_result.mean_estimate_ns;
            let glam_time = glam_result.mean_estimate_ns;
            let time_diff = (ggmath_time - glam_time).abs();

            // Determine if this is a meaningful difference
            let is_meaningfully_slower = if time_diff >= MIN_TIME_DIFF_NS && ggmath_time > glam_time
            {
                let slowdown_factor = ggmath_time / glam_time;
                slowdown_factor >= NOISE_THRESHOLD
            } else {
                false
            };

            // Store all comparisons for first pass
            all_comparisons.push((
                group_name.clone(),
                ggmath_time,
                glam_time,
                is_meaningfully_slower,
            ));

            // Store slower functions for second pass
            if is_meaningfully_slower {
                let slowdown_factor = ggmath_time / glam_time;
                slower_functions.push((
                    group_name.clone(),
                    slowdown_factor,
                    ggmath_time,
                    glam_time,
                ));
            }
        }
    }

    // Sort all comparisons by function name for consistent output
    all_comparisons.sort_by(|a, b| a.0.cmp(&b.0));

    // First pass: Show all function pairs with pass/fail coloring
    for (function_name, ggmath_time, glam_time, is_slower) in &all_comparisons {
        if *is_slower {
            println!(
                "{}",
                format!(
                    "{}: {:.2} ns vs {:.2} ns",
                    function_name, ggmath_time, glam_time
                )
                .red()
                .bold()
            );
        } else {
            println!(
                "{}",
                format!(
                    "{}: {:.2} ns vs {:.2} ns",
                    function_name, ggmath_time, glam_time
                )
                .green()
                .bold()
            );
        }
    }

    println!();

    // Second pass: Summary section
    if slower_functions.is_empty() {
        println!(
            "{}",
            "GG! All functions passed performance checks!"
                .green()
                .bold()
        );
    } else {
        println!("{}", "GG... Functions that need optimization:".red().bold());
        println!();

        // Sort by slowdown factor (worst first)
        slower_functions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (function_name, slowdown_factor, ggmath_time, glam_time) in slower_functions {
            println!(
                "{}",
                format!(
                    "{}: ggmath is {:.2}x slower ({:.2} ns vs {:.2} ns)",
                    function_name, slowdown_factor, ggmath_time, glam_time
                )
                .red()
                .bold()
            );
        }
    }

    Ok(())
}

fn extract_benchmark_name(path: &Path) -> Option<String> {
    // Extract benchmark name from path like: target/criterion/group_name/function_name/type/estimates.json
    // We only want 'base' estimates, not 'change' or 'new'
    let path_str = path.to_string_lossy();
    let parts: Vec<&str> = path_str.split(['/', '\\']).collect();

    // Find the criterion directory index
    let criterion_index = parts.iter().position(|&p| p == "criterion")?;

    // Make sure we have group/function/type/estimates.json after criterion
    if parts.len() >= criterion_index + 5 && parts[parts.len() - 1] == "estimates.json" {
        let group_name = parts[criterion_index + 1];
        let function_name = parts[criterion_index + 2];
        let estimate_type = parts[criterion_index + 3];

        // Only use 'base' estimates for consistency
        if estimate_type == "base" {
            Some(format!("{}/{}", group_name, function_name))
        } else {
            None
        }
    } else {
        None
    }
}
