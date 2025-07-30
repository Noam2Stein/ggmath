use std::mem::MaybeUninit;

use super::*;

primitive_aliases! { pub U16 => u16 }

impl Scalar for u16 {
    type Vec2Alignment = Align<4>;
    type Vec3Alignment = Align<8>;
    type Vec4Alignment = Align<8>;

    const NOT_GARBAGE: Option<fn(MaybeUninit<Self>) -> MaybeUninit<Self>> = Some(|x| unsafe {
        let x = x.assume_init();

        let output = !x;

        MaybeUninit::new(output)
    });

    const ADD_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x.checked_add(y).unwrap_unchecked();

            MaybeUninit::new(output)
        });

    const SUB_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x.checked_sub(y).unwrap_unchecked();

            MaybeUninit::new(output)
        });

    const MUL_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x.checked_mul(y).unwrap_unchecked();

            MaybeUninit::new(output)
        });

    const REM_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x.checked_rem(y).unwrap_unchecked();

            MaybeUninit::new(output)
        });

    const BITAND_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x & y;

            MaybeUninit::new(output)
        });

    const BITOR_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x | y;

            MaybeUninit::new(output)
        });

    const BITXOR_GARBAGE: Option<fn(MaybeUninit<Self>, MaybeUninit<Self>) -> MaybeUninit<Self>> =
        Some(|x, y| unsafe {
            let x = x.assume_init();
            let y = y.assume_init();

            let output = x ^ y;

            MaybeUninit::new(output)
        });
}

// u16 methods are defined using `macro_loop` in other files
