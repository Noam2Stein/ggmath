use std::time::Instant;

mod matrix;
mod rectangle;
mod scalars;
mod vector;
use matrix::test_matrix;
use rectangle::test_rectangle;
use scalars::test_scalars;
use vector::test_vector;

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
    test_vector();
    test_matrix();
    test_rectangle();
    test_scalars();
}
