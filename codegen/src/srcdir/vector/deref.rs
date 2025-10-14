use genco::quote;

use crate::util::TokensExt;

pub fn generate() {
    quote! {
        $(let common_lengths = [2, 3, 4])
        $(let axes_lowercase = ["x", "y", "z", "w"])
        $(let axes_ordinals = ["1st", "2nd", "3rd", "4th"])

        use std::{
            mem::transmute,
            ops::{Deref, DerefMut},
        };

        use crate::{Construct, Scalar, Simdness, Vector};

        $(
            for n in common_lengths join($['\n']) =>

            #[repr(C)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
            pub struct Vec$(n)Elements<T: Construct> {
                $(
                    for i in 0..n join($['\r']) =>

                    $(format!("/// The {} element of the vector.", axes_ordinals[i]))
                    pub $(axes_lowercase[i]): T,
                )
            }
        )

        $(
            for n in common_lengths join($['\n']) =>

            impl<T: Scalar<$n>, S: Simdness> Deref for Vector<$n, T, S> {
                type Target = Vec$(n)Elements<T>;

                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                    $(format!("// SAFETY: Vector<{n}, T, S> is guaranteed to begin with {n} consecutive T elements"))
                    unsafe { transmute::<&Vector<$n, T, S>, &Vec$(n)Elements<T>>(self) }
                }
            }
        )

        $(
            for n in common_lengths join($['\n']) =>

            impl<T: Scalar<$n>, S: Simdness> DerefMut for Vector<$n, T, S> {
                #[inline(always)]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    $(format!("// SAFETY: Vector<{n}, T, S> is guaranteed to begin with {n} consecutive T elements"))
                    unsafe { transmute::<&mut Vector<$n, T, S>, &mut Vec$(n)Elements<T>>(self) }
                }
            }
        )
    }
    .write_in_src("vector/deref.rs");
}
