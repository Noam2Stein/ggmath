use std::mem::MaybeUninit;

use super::*;

primitive_aliases! { pub Isize => isize }

impl Scalar for isize {
    type Vec2Alignment = <isized as Scalar>::Vec2Alignment;
    type Vec3Alignment = <isized as Scalar>::Vec3Alignment;
    type Vec4Alignment = <isized as Scalar>::Vec4Alignment;

    const NEG_GARBAGE: Option<fn(MaybeUninit<Self>) -> MaybeUninit<Self>> = Some(|x| unsafe {
        let x = x.assume_init();

        let output = x.checked_neg().unwrap_unchecked();

        MaybeUninit::new(output)
    });

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

// isize methods are defined using `macro_loop` in i8.rs

#[cfg(target_pointer_width = "16")]
#[allow(non_camel_case_types)]
type isized = i16;

#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
type isized = i32;

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
type isized = i64;
