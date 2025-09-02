use colored::Colorize;

mod vector;

fn main() {
    let mut slow_fns = Vec::new();

    vector::bench_cmp(&mut slow_fns);

    println!();

    if slow_fns.is_empty() {
        println!(
            "{}",
            "GG! All functions are seen as fast enough.".green().bold()
        );
        println!(
            "{}",
            "Functions are compared against other crates."
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            "These functions are slower than in other crates:"
                .red()
                .bold()
        );
        println!();

        for slow_fn in slow_fns {
            println!("{}", slow_fn.red().bold());
        }
    }
}
