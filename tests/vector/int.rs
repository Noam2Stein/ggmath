use crate::{assert_debug_panic, assert_debug_panic_val, assert_panic, test};

test! {
    int_api for T, S in [i8, i16, i32, i64, i128, isize], [Simd, NonSimd]:

    assert_eq!(-vec2t!(1, 2), vec2t!(-1, -2), "vec2 neg");
    assert_eq!(-vec3t!(1, 2, 3), vec3t!(-1, -2, -3), "vec3 neg");
    assert_eq!(-vec4t!(1, 2, 3, 4), vec4t!(-1, -2, -3, -4), "vec4 neg");

    assert_debug_panic_val!(-vec2t!(1, T::MIN), vec2t!(-1, -T::MIN), "vec2 neg overflow");
    assert_debug_panic_val!(-vec3t!(1, T::MIN, 1), vec3t!(-1, -T::MIN, -1), "vec3 neg overflow");
    assert_debug_panic_val!(-vec4t!(1, T::MIN, 1, 1), vec4t!(-1, -T::MIN, -1, -1), "vec4 neg overflow");

    assert_eq!(!vec2t!(1, 2), vec2t!(!1, !2), "vec2 bitwise not");
    assert_eq!(!vec3t!(1, 2, 3), vec3t!(!1, !2, !3), "vec3 bitwise not");
    assert_eq!(!vec4t!(1, 2, 3, 4), vec4t!(!1, !2, !3, !4), "vec4 bitwise not");

    assert_eq!(vec2t!(1, 2) + vec2t!(3, 4), vec2t!(4, 6), "vec2 add");
    assert_eq!(vec3t!(1, 2, 3) + vec3t!(4, 5, 6), vec3t!(5, 7, 9), "vec3 add");
    assert_eq!(vec4t!(1, 2, 3, 4) + vec4t!(5, 6, 7, 8), vec4t!(6, 8, 10, 12), "vec4 add");

    assert_debug_panic_val!(vec2t!(1, T::MAX) + vec2t!(3, 1), vec2t!(4, T::MIN), "vec2 overflowing add");
    assert_debug_panic_val!(vec3t!(1, T::MAX, 1) + vec3t!(3, 1, 1), vec3t!(4, T::MIN, 2), "vec3 overflowing add");
    assert_debug_panic_val!(vec4t!(1, T::MAX, 1, 1) + vec4t!(3, 1, 1, 1), vec4t!(4, T::MIN, 2, 2), "vec4 overflowing add");

    assert_debug_panic_val!(vec2t!(1, T::MAX) + vec2t!(3, 3), vec2t!(4, T::MIN + 2), "vec2 overflowing add");
    assert_debug_panic_val!(vec3t!(1, T::MAX, 1) + vec3t!(3, 3, 1), vec3t!(4, T::MIN + 2, 2), "vec3 overflowing add");
    assert_debug_panic_val!(vec4t!(1, T::MAX, 1, 1) + vec4t!(3, 3, 1, 1), vec4t!(4, T::MIN + 2, 2, 2), "vec4 overflowing add");

    assert_debug_panic_val!(vec2t!(1, T::MIN) + vec2t!(3, -1), vec2t!(4, T::MAX), "vec2 overflowing add");
    assert_debug_panic_val!(vec3t!(1, T::MIN, 1) + vec3t!(3, -1, 1), vec3t!(4, T::MAX, 2), "vec3 overflowing add");
    assert_debug_panic_val!(vec4t!(1, T::MIN, 1, 1) + vec4t!(3, -1, 1, 1), vec4t!(4, T::MAX, 2, 2), "vec4 overflowing add");

    assert_debug_panic_val!(vec2t!(1, T::MIN) + vec2t!(3, -3), vec2t!(4, T::MAX - 2), "vec2 overflowing add");
    assert_debug_panic_val!(vec3t!(1, T::MIN, 1) + vec3t!(3, -3, 1), vec3t!(4, T::MAX - 2, 2), "vec3 overflowing add");
    assert_debug_panic_val!(vec4t!(1, T::MIN, 1, 1) + vec4t!(3, -3, 1, 1), vec4t!(4, T::MAX - 2, 2, 2), "vec4 overflowing add");

    assert_eq!(vec2t!(3, 4) - vec2t!(1, 3), vec2t!(2, 1), "vec2 sub");
    assert_eq!(vec3t!(3, 4, 5) - vec3t!(1, 3, 2), vec3t!(2, 1, 3), "vec3 sub");
    assert_eq!(vec4t!(3, 4, 5, 6) - vec4t!(1, 3, 2, 1), vec4t!(2, 1, 3, 5), "vec4 sub");

    assert_debug_panic_val!(vec2t!(21, T::MIN) - vec2t!(3, 1), vec2t!(18, T::MAX), "overflowing sub");
    assert_debug_panic_val!(vec3t!(21, T::MIN, 1) - vec3t!(3, 1, 1), vec3t!(18, T::MAX, 0), "overflowing sub");
    assert_debug_panic_val!(vec4t!(21, T::MIN, 1, 1) - vec4t!(3, 1, 1, 1), vec4t!(18, T::MAX, 0, 0), "overflowing sub");

    assert_debug_panic_val!(vec2t!(21, T::MIN) - vec2t!(3, 3), vec2t!(18, T::MAX - 2), "overflowing sub");
    assert_debug_panic_val!(vec3t!(21, T::MIN, 1) - vec3t!(3, 3, 1), vec3t!(18, T::MAX - 2, 0), "overflowing sub");
    assert_debug_panic_val!(vec4t!(21, T::MIN, 1, 1) - vec4t!(3, 3, 1, 1), vec4t!(18, T::MAX - 2, 0, 0), "overflowing sub");

    assert_debug_panic_val!(vec2t!(21, T::MAX) - vec2t!(3, -1), vec2t!(18, T::MIN), "overflowing sub");
    assert_debug_panic_val!(vec3t!(21, T::MAX, 1) - vec3t!(3, -1, 1), vec3t!(18, T::MIN, 0), "overflowing sub");
    assert_debug_panic_val!(vec4t!(21, T::MAX, 1, 1) - vec4t!(3, -1, 1, 1), vec4t!(18, T::MIN, 0, 0), "overflowing sub");

    assert_debug_panic_val!(vec2t!(21, T::MAX) - vec2t!(3, -3), vec2t!(18, T::MIN + 2), "overflowing sub");
    assert_debug_panic_val!(vec3t!(21, T::MAX, 1) - vec3t!(3, -3, 1), vec3t!(18, T::MIN + 2, 0), "overflowing sub");
    assert_debug_panic_val!(vec4t!(21, T::MAX, 1, 1) - vec4t!(3, -3, 1, 1), vec4t!(18, T::MIN + 2, 0, 0), "overflowing sub");

    assert_eq!(vec2t!(1, 2) * vec2t!(3, 4), vec2t!(3, 8), "vec2 mul");
    assert_eq!(vec3t!(1, 2, 3) * vec3t!(4, 5, 6), vec3t!(4, 10, 18), "vec3 mul");
    assert_eq!(vec4t!(1, 2, 3, 4) * vec4t!(5, 6, 7, 8), vec4t!(5, 12, 21, 32), "vec4 mul");

    assert_debug_panic_val!(vec2t!(21, T::MAX) * vec2t!(2), vec2t!(42, T::MAX * 2), "overflowing mul");
    assert_debug_panic_val!(vec3t!(21, T::MAX, 1) * vec3t!(2), vec3t!(42, T::MAX * 2, 2), "overflowing mul");
    assert_debug_panic_val!(vec4t!(21, T::MAX, 1, 1) * vec4t!(2), vec4t!(42, T::MAX * 2, 2, 2), "overflowing mul");

    assert_debug_panic_val!(vec2t!(21, T::MAX) * vec2t!(-2), vec2t!(-42, T::MAX * -2), "overflowing mul");
    assert_debug_panic_val!(vec3t!(21, T::MAX, 1) * vec3t!(-2), vec3t!(-42, T::MAX * -2, -2), "overflowing mul");
    assert_debug_panic_val!(vec4t!(21, T::MAX, 1, 1) * vec4t!(-2), vec4t!(-42, T::MAX * -2, -2, -2), "overflowing mul");

    assert_debug_panic_val!(vec2t!(21, T::MIN) * vec2t!(-1), vec2t!(-21, -T::MIN), "overflowing mul");
    assert_debug_panic_val!(vec3t!(21, T::MIN, 1) * vec3t!(-1), vec3t!(-21, -T::MIN, -1), "overflowing mul");
    assert_debug_panic_val!(vec4t!(21, T::MIN, 1, 1) * vec4t!(-1), vec4t!(-21, -T::MIN, -1, -1), "overflowing mul");

    assert_eq!(vec2t!(31, 41) / vec2t!(5, 12), vec2t!(6, 3), "vec2 div");
    assert_eq!(vec3t!(31, 41, 51) / vec3t!(5, 12, 13), vec3t!(6, 3, 3), "vec3 div");
    assert_eq!(vec4t!(31, 41, 51, 61) / vec4t!(5, 12, 13, 14), vec4t!(6, 3, 3, 4), "vec4 div");

    assert_eq!(vec2t!(1, 2) / vec2t!(3, 4), vec2t!(0), "vec2 div");
    assert_eq!(vec3t!(1, 2, 3) / vec3t!(4, 5, 6), vec3t!(0), "vec3 div");
    assert_eq!(vec4t!(1, 2, 3, 4) / vec4t!(5, 6, 7, 8), vec4t!(0), "vec4 div");

    assert_panic!(vec2t!(1, 2) / vec2t!(3, 0), "vec2 div by zero");
    assert_panic!(vec3t!(1, 2, 3) / vec3t!(4, 0, 6), "vec3 div by zero");
    assert_panic!(vec4t!(1, 2, 3, 4) / vec4t!(5, 0, 7, 8), "vec4 div by zero");

    assert_panic!(vec2t!(1, T::MIN) / vec2t!(1, -1), "vec2 overflowing div");
    assert_panic!(vec3t!(1, T::MIN, 1) / vec3t!(1, -1, 1), "vec3 overflowing div");
    assert_panic!(vec4t!(1, T::MIN, 1, 1) / vec4t!(1, -1, 1, 1), "vec4 overflowing div");

    assert_eq!(vec2t!(31, 41) % vec2t!(5, 12), vec2t!(1, 5), "vec2 rem");
    assert_eq!(vec3t!(31, 41, 51) % vec3t!(5, 12, 13), vec3t!(1, 5, 12), "vec3 rem");
    assert_eq!(vec4t!(31, 41, 51, 61) % vec4t!(5, 12, 13, 14), vec4t!(1, 5, 12, 5), "vec4 rem");

    assert_eq!(vec2t!(1, 2) % vec2t!(3, 4), vec2t!(1, 2), "vec2 rem");
    assert_eq!(vec3t!(1, 2, 3) % vec3t!(4, 5, 1), vec3t!(1, 2, 0), "vec3 rem");
    assert_eq!(vec4t!(1, 2, 3, 4) % vec4t!(5, 6, 1, 1), vec4t!(1, 2, 0, 0), "vec4 rem");

    assert_panic!(vec2t!(1, 2) % vec2t!(3, 0), "vec2 rem by zero");
    assert_panic!(vec3t!(1, 2, 3) % vec3t!(4, 0, 6), "vec3 rem by zero");
    assert_panic!(vec4t!(1, 2, 3, 4) % vec4t!(5, 0, 7, 8), "vec4 rem by zero");
}
