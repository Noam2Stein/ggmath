mod matrix;
//mod rectangle;
mod scalars;
mod vector;
use ggmath::testing::{FormatTestingResult, TestResult};
use matrix::*;
//use rectangle::*;
use scalars::*;
use vector::*;

fn main() {
    println!("{}", test().fmt_test_result())
}

fn test() -> TestResult {
    test_vector()?;
    test_scalars()?;
    test_matrix()?;
    //test_rectangle()?;

    Ok(())
}
