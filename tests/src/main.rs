use std::time::Instant;

use matrix::test_matrix;
use scalars::test_scalars;

mod matrix;
mod scalars;

fn main() {
    let start_time = Instant::now();

    test();

    let end_time = Instant::now();
    let duration = (end_time - start_time).as_secs_f64();

    println!(
        "{}{}all tests passed! GG{}. time: {duration} secs",
        "\x1b[1m", "\x1b[32m", "\x1b[0m"
    );
}

fn test() {
    test_scalars();

    test_matrix::<u32>();
}
