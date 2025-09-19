use genco::quote;

use crate::{
    constants::LENGTHS,
    module::{SrcFile, TokensExt},
};

pub fn src_mod() -> SrcFile {
    quote! {
        use crate::{Scalar, Vector, $(for &n in LENGTHS join(, )=> Vec$(n)), VecAlignment, VecLen, Usize};

        impl<T: Scalar> Scalar for Option<T> {
            $(for &n in LENGTHS join($['\r']) => type InnerAlignedVec$(n) = [Option<T>; $n];)

            $(
                for &n in LENGTHS join($['\n']) =>

                #[inline(always)]
                fn vec$(n)_from_array(array: [Option<T>; $n]) -> Vec$(n)<Option<T>> {
                    Vector(array)
                }

                #[inline(always)]
                fn vec$(n)_as_array(vec: Vec$(n)<Option<T>>) -> [Option<T>; $n] {
                    vec.0
                }
            )
        }
        
        impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, Option<T>, A>
        where
            Usize<N>: VecLen,
        {
            $("/// Returns a vector of bools with `true` for each element that is `Some`, otherwise `false`.")
            #[inline(always)]
            pub fn some_mask(self) -> Vector<N, bool, A> {
                self.map(|x| x.is_some())
            }

            $("/// Returns a vector of bools with `true` for each element that is `None`, otherwise `false`.")
            #[inline(always)]
            pub fn none_mask(self) -> Vector<N, bool, A> {
                self.map(|x| x.is_none())
            }
            
            $("/// Returns `Some(self)` if all elements are `Some`, otherwise returns `None`.")
            #[inline(always)]
            pub fn flatten(self) -> Option<Vector<N, T, A>> {
                if self.some_mask().all_true() {
                    Some(self.map(|x| x.unwrap()))
                } else {
                    None
                }
            }
        }
    }
    .to_src_file("option")
}
