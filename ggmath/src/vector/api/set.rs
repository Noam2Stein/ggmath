use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn set(&mut self, index: usize, value: T) -> Result<(), ()> {
        if index >= N {
            Err(())
        } else {
            unsafe { self.set_unchecked(index, value) }
            Ok(())
        }
    }
    #[inline(always)]
    fn set_1_1(
        &mut self,
        indicies: [usize; 2],
        values: Vector<2, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        if indicies.into_iter().any(|index| index >= N) {
            Err(())
        } else if indicies[0] == indicies[1] {
            Err(())
        } else {
            unsafe { self.set_2_unchecked(indicies, values) }
            Ok(())
        }
    }
    #[inline(always)]
    fn set_1_1_1(
        &mut self,
        indicies: [usize; 3],
        values: Vector<3, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        if indicies.into_iter().any(|index| index >= N) {
            Err(())
        } else if indicies[0] == indicies[1] {
            Err(())
        } else if indicies[0] == indicies[2] {
            Err(())
        } else if indicies[1] == indicies[2] {
            Err(())
        } else {
            unsafe { self.set_3_unchecked(indicies, values) }
            Ok(())
        }
    }
    #[inline(always)]
    fn set_1_1_1_1(
        &mut self,
        indicies: [usize; 4],
        values: Vector<4, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        if indicies.into_iter().any(|index| index >= N) {
            Err(())
        } else if indicies[0] == indicies[1] {
            Err(())
        } else if indicies[0] == indicies[2] {
            Err(())
        } else if indicies[0] == indicies[3] {
            Err(())
        } else if indicies[1] == indicies[2] {
            Err(())
        } else if indicies[1] == indicies[3] {
            Err(())
        } else if indicies[2] == indicies[3] {
            Err(())
        } else {
            unsafe { self.set_4_unchecked(indicies, values) }
            Ok(())
        }
    }

    #[inline(always)]
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        *self = self.with_unchecked(index, value)
    }
    #[inline(always)]
    unsafe fn set_1_1_unchecked(
        &mut self,
        indicies: [usize; 2],
        values: Vector<2, T, impl VecAlignment>,
    ) {
        *self = self.with_2_unchecked(indicies, values)
    }
    #[inline(always)]
    unsafe fn set_1_1_1_unchecked(
        &mut self,
        indicies: [usize; 3],
        values: Vector<3, T, impl VecAlignment>,
    ) {
        *self = self.with_3_unchecked(indicies, values)
    }
    #[inline(always)]
    unsafe fn set_1_1_1_1_unchecked(
        &mut self,
        indicies: [usize; 4],
        values: Vector<4, T, impl VecAlignment>,
    ) {
        *self = self.with_4_unchecked(indicies, values)
    }
}
