use indoc::indoc;

use super::*;

pub fn write_mod(mut module: Mod, scalar_trait: &mut ScalarTrait) {
    writedoc!(
        module,
        "
        use super::*;
        
        impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn default() -> Self {{
                T::vec_default()
            }}
        }}
        "
    )
    .unwrap();

    scalar_trait.push_overridable_fn(
        "default",
        indoc! {"
            #[inline(always)]
            fn vec_default<const N: usize, A: VecAlignment>() -> Vector<N, Self, A> {
                Vector::splat(Self::default())
            }
        "},
    );
}
