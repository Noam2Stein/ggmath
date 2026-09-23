use core::mem::MaybeUninit;

use crate::{Alignment, Dim, Element, TwoThreeOrFour, Vector, utils::transmute_generic};

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Dim<N>: TwoThreeOrFour,
    T: Element + Copy,
{
    pub(crate) fn option_from_fn<F>(mut f: F) -> Option<Self>
    where
        F: FnMut(usize) -> Option<T>,
    {
        let mut array = [MaybeUninit::uninit(); N];

        for (i, element_mut) in array.iter_mut().enumerate() {
            *element_mut = MaybeUninit::new(f(i)?);
        }

        // SAFETY: If this part of the function is reached, all elements of the
        // array have been initialized.
        let array = unsafe { transmute_generic::<[MaybeUninit<T>; N], [T; N]>(array) };

        Some(Self::from_array(array))
    }
}
