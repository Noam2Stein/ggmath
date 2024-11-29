use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn set_n<const N_VALUE: usize>(
        &mut self,
        index: usize,
        value: VectorOrT<N_VALUE, T, impl VecAlignment>,
    ) -> Result<(), ()>
    where
        ScalarCount<N_VALUE>: VecLenOr1,
    {
        match self.with_n(index, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }

    #[inline(always)]
    pub unsafe fn set_n_unchecked<const N_VALUE: usize, AValue: VecAlignment>(
        &mut self,
        index: usize,
        value: VectorOrT<N_VALUE, T, AValue>,
    ) where
        ScalarCount<N_VALUE>: VecLenOr1,
    {
        *self = self.with_n_unchecked(index, value)
    }

    #[inline(always)]
    pub fn set(&mut self, index: usize, value: T) -> Result<(), ()> {
        match self.with(index, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_2(
        &mut self,
        index: usize,
        value: Vector<2, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_2(index, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_3(
        &mut self,
        index: usize,
        value: Vector<3, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_3(index, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_4(
        &mut self,
        index: usize,
        value: Vector<4, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_4(index, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }

    #[inline(always)]
    pub fn set_1_1(
        &mut self,
        indicies: [usize; 2],
        value: Vector<2, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_1_1(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_1_2(
        &mut self,
        indicies: [usize; 2],
        value: Vector<3, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_1_2(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_1_3(
        &mut self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_1_3(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_2_1(
        &mut self,
        indicies: [usize; 2],
        value: Vector<3, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_2_1(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_2_2(
        &mut self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_2_2(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_3_1(
        &mut self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_3_1(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }

    #[inline(always)]
    pub fn set_1_1_1(
        &mut self,
        indicies: [usize; 3],
        value: Vector<3, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_1_1_1(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_1_1_2(
        &mut self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_1_1_2(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_1_2_1(
        &mut self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_1_2_1(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }
    #[inline(always)]
    pub fn set_2_1_1(
        &mut self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_2_1_1(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }

    #[inline(always)]
    pub fn set_1_1_1_1(
        &mut self,
        indicies: [usize; 4],
        value: Vector<4, T, impl VecAlignment>,
    ) -> Result<(), ()> {
        match self.with_1_1_1_1(indicies, value) {
            Some(self_value) => {
                *self = self_value;
                Ok(())
            }
            None => Err(()),
        }
    }

    #[inline(always)]
    pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        *self = self.with_unchecked(index, value)
    }
    #[inline(always)]
    pub unsafe fn set_2_unchecked(&mut self, index: usize, value: Vector<2, T, impl VecAlignment>) {
        *self = self.with_2_unchecked(index, value)
    }
    #[inline(always)]
    pub unsafe fn set_3_unchecked(&mut self, index: usize, value: Vector<3, T, impl VecAlignment>) {
        *self = self.with_3_unchecked(index, value)
    }
    #[inline(always)]
    pub unsafe fn set_4_unchecked(&mut self, index: usize, value: Vector<4, T, impl VecAlignment>) {
        *self = self.with_4_unchecked(index, value)
    }

    #[inline(always)]
    pub unsafe fn set_1_1_unchecked(
        &mut self,
        indicies: [usize; 2],
        value: Vector<2, T, impl VecAlignment>,
    ) {
        *self = self.with_1_1_unchecked(indicies, value)
    }
    #[inline(always)]
    pub unsafe fn set_1_2_unchecked(
        &mut self,
        indicies: [usize; 2],
        value: Vector<3, T, impl VecAlignment>,
    ) {
        *self = self.with_1_2_unchecked(indicies, value)
    }
    #[inline(always)]
    pub unsafe fn set_1_3_unchecked(
        &mut self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) {
        *self = self.with_1_3_unchecked(indicies, value)
    }
    #[inline(always)]
    pub unsafe fn set_2_1_unchecked(
        &mut self,
        indicies: [usize; 2],
        value: Vector<3, T, impl VecAlignment>,
    ) {
        *self = self.with_2_1_unchecked(indicies, value)
    }
    #[inline(always)]
    pub unsafe fn set_2_2_unchecked(
        &mut self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) {
        *self = self.with_2_2_unchecked(indicies, value)
    }
    #[inline(always)]
    pub unsafe fn set_3_1_unchecked(
        &mut self,
        indicies: [usize; 2],
        value: Vector<4, T, impl VecAlignment>,
    ) {
        *self = self.with_3_1_unchecked(indicies, value)
    }

    #[inline(always)]
    pub unsafe fn set_1_1_1_unchecked(
        &mut self,
        indicies: [usize; 3],
        value: Vector<3, T, impl VecAlignment>,
    ) {
        *self = self.with_1_1_1_unchecked(indicies, value)
    }
    #[inline(always)]
    pub unsafe fn set_1_1_2_unchecked(
        &mut self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) {
        *self = self.with_1_1_2_unchecked(indicies, value)
    }
    #[inline(always)]
    pub unsafe fn set_1_2_1_unchecked(
        &mut self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) {
        *self = self.with_1_2_1_unchecked(indicies, value)
    }
    #[inline(always)]
    pub unsafe fn set_2_1_1_unchecked(
        &mut self,
        indicies: [usize; 3],
        value: Vector<4, T, impl VecAlignment>,
    ) {
        *self = self.with_2_1_1_unchecked(indicies, value)
    }

    #[inline(always)]
    pub unsafe fn set_1_1_1_1_unchecked(
        &mut self,
        indicies: [usize; 4],
        value: Vector<4, T, impl VecAlignment>,
    ) {
        *self = self.with_1_1_1_1_unchecked(indicies, value)
    }
}
