use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::module::*;

pub fn write_mod(mut module: Mod) {
    let mut transmute_fns = Vec::new();

    for ownership in ["move", "ref", "mut"] {
        let ownership_postfix = match ownership {
            "move" => "",
            "ref" => "_ref",
            "mut" => "_mut",
            _ => unreachable!(),
        };

        let ownership_type_prefix = match ownership {
            "move" => "",
            "ref" => "&",
            "mut" => "&mut ",
            _ => unreachable!(),
        };

        transmute_fns.push(formatdoc! {r#"
            /// Transmutes `self` to a vector of a different length.
            ///
            /// Only use this if you know that `N2` is the same as `N`.
            /// If this is not the case, it is undefined behavior.
            #[inline(always)]
            pub const unsafe fn transmute_len{ownership_postfix}<const N2: usize>({ownership_type_prefix}self) -> {ownership_type_prefix}Vector<N2, T, A>
            where
                Usize<N2>: VecLen,
            {{
                unsafe {{ std::mem::transmute_copy::<{ownership_type_prefix}Vector<N, T, A>, {ownership_type_prefix}Vector<N2, T, A>>(&self) }}
            }}

            /// Transmutes `self` to a vector of a different scalar type.
            ///
            /// Only use this if you are sure that `T2` has the same size and alignment as `T`,
            /// and that `T` is transmutable to `T2`.
            /// If this is not the case, it is undefined behavior.
            #[inline(always)]
            pub const unsafe fn transmute_scalar{ownership_postfix}<T2: Scalar>({ownership_type_prefix}self) -> {ownership_type_prefix}Vector<N, T2, A> {{
                unsafe {{ std::mem::transmute_copy::<{ownership_type_prefix}Vector<N, T, A>, {ownership_type_prefix}Vector<N, T2, A>>(&self) }}
            }}

            /// Transmutes `self` to a vector of a different alignment.
            ///
            /// Only use this if you know that `A2` is the same as `A`.
            /// If this is not the case, it is undefined behavior.
            #[inline(always)]
            pub const unsafe fn transmute_alignment{ownership_postfix}<A2: VecAlignment>({ownership_type_prefix}self) -> {ownership_type_prefix}Vector<N, T, A2> {{
                unsafe {{ std::mem::transmute_copy::<{ownership_type_prefix}Vector<N, T, A>, {ownership_type_prefix}Vector<N, T, A2>>(&self) }}
            }}
        "#});

        for n in 2..=4 {
            transmute_fns.push(formatdoc! {r#"
                /// Transmutes `self` to `{ownership_postfix}Vector<{n}, T, VecAligned>`.
                /// 
                /// Only use this if you know that `A` is `VecAligned` and `N` is `{n}`.
                /// If this is not the case, it is undefined behavior.
                #[inline(always)]
                pub const unsafe fn transmute_vec{n}{ownership_postfix}({ownership_type_prefix}self) -> {ownership_type_prefix}Vector<{n}, T, VecAligned> {{
                    unsafe {{ std::mem::transmute_copy::<{ownership_type_prefix}Vector<N, T, A>, {ownership_type_prefix}Vector<{n}, T, VecAligned>>(&self) }}
                }}

                /// Transmutes `self` to `{ownership_postfix}Vector<{n}, T, VecPacked>`.
                /// 
                /// Only use this if you know that `A` is `VecPacked`, and `N` is `{n}`.
                /// If this is not the case, it is undefined behavior.
                #[inline(always)]
                pub const unsafe fn transmute_vec{n}p{ownership_postfix}({ownership_type_prefix}self) -> {ownership_type_prefix}Vector<{n}, T, VecPacked> {{
                    unsafe {{ std::mem::transmute_copy::<{ownership_type_prefix}Vector<N, T, A>, {ownership_type_prefix}Vector<{n}, T, VecPacked>>(&self) }}
                }}
            "#});
        }
    }

    let transmute_fns = transmute_fns.join("\n").replace("\n", "\n\t");

    writedoc!(
        module,
        r#"
        use crate::{{Scalar, Usize, VecAligned, VecAlignment, VecLen, VecPacked, Vector}};

        impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            {transmute_fns}
        }}
        "#
    )
    .unwrap();
}
