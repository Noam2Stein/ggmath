use ggmath_testing::test_scalar::test_scalar;

fn main() {
    test_scalar::<f32>();
    test_scalar::<f64>();

    test_scalar::<u8>();
    test_scalar::<u16>();
    test_scalar::<u32>();
    test_scalar::<u64>();
    test_scalar::<u128>();
    test_scalar::<usize>();

    test_scalar::<i8>();
    test_scalar::<i16>();
    test_scalar::<i32>();
    test_scalar::<i64>();
    test_scalar::<i128>();
    test_scalar::<isize>();

    test_scalar::<bool>();

    println!(
        "{}{}all tests passed! GG{}",
        "\x1b[1m", "\x1b[32m", "\x1b[0m"
    );
}
