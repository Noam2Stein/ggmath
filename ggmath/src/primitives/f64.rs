use std::mem::MaybeUninit;

use super::*;

primitive_aliases! { pub D => f64 }

impl Scalar for f64 {
    type Vec2Alignment = Align<16>;
    type Vec3Alignment = Align<32>;
    type Vec4Alignment = Align<32>;

    const NEG_GARBAGE: Option<fn(MaybeUninit<Self>) -> MaybeUninit<Self>> = Some(|x| unsafe {
        let x = x.assume_init();

        let output = -x;

        MaybeUninit::new(output)
    });

    const ADD_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x + y;

            MaybeUninit::new(output)
        });

    const SUB_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x - y;

            MaybeUninit::new(output)
        });

    const MUL_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x * y;

            MaybeUninit::new(output)
        });

    const DIV_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x / y;

            MaybeUninit::new(output)
        });

    const REM_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x % y;

            MaybeUninit::new(output)
        });
}

// f64 methods are defined using `macro_loop` in other files
