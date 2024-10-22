use std::mem::transmute;

ggmath_proc_macros::vec_api!(
    Swizzle:

    fn get(self, index: usize) -> Option<T> {
        if index >= N {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }
    fn get_2(self, indicies: [usize; 2]) -> Option<Vector2<T, A>> {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else {
            Some(unsafe { self.get_2_unchecked(indicies) })
        }
    }
    fn get_3(self, indicies: [usize; 3]) -> Option<Vector3<T, A>> {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else {
            Some(unsafe { self.get_3_unchecked(indicies) })
        }
    }
    fn get_4(self, indicies: [usize; 4]) -> Option<Vector4<T, A>> {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else {
            Some(unsafe { self.get_4_unchecked(indicies) })
        }
    }
    unsafe fn get_unchecked(self, index: usize) -> T;
    unsafe fn get_2_unchecked(self, indicies: [usize; 2]) -> Vector2<T, A>;
    unsafe fn get_3_unchecked(self, indicies: [usize; 3]) -> Vector3<T, A>;
    unsafe fn get_4_unchecked(self, indicies: [usize; 4]) -> Vector4<T, A>;

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index + 1 > N {
            None
        }
        else {
            Some(unsafe { self.get_mut_unchecked(index) })
        }
    }
    fn get_mut_2(&mut self, index: usize) -> Option<&mut Vec2P<T>> {
        if index + 2 > N {
            None
        }
        else {
            Some(unsafe { self.get_mut_2_unchecked(index) })
        }
    }
    fn get_mut_3(&mut self, index: usize) -> Option<&mut Vec3P<T>> {
        if index + 3 > N {
            None
        }
        else {
            Some(unsafe { self.get_mut_3_unchecked(index) })
        }
    }
    fn get_mut_4(&mut self, index: usize) -> Option<&mut Vec4P<T>> {
        if index + 4 > N {
            None
        }
        else {
            Some(unsafe { self.get_mut_4_unchecked(index) })
        }
    }
    unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        self.as_array_mut().get_unchecked_mut(index)
    }
    unsafe fn get_mut_2_unchecked(&mut self, index: usize) -> &mut Vec2P<T> {
        transmute(self.get_mut_unchecked(index))
    }
    unsafe fn get_mut_3_unchecked(&mut self, index: usize) -> &mut Vec3P<T> {
        transmute(self.get_mut_unchecked(index))
    }
    unsafe fn get_mut_4_unchecked(&mut self, index: usize) -> &mut Vec4P<T> {
        transmute(self.get_mut_unchecked(index))
    }

    fn get_mut_1_1(&mut self, indicies: [usize; 2]) -> Option<(&mut T, &mut T)> {
        if indicies[0] == indicies[1] {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[1])?,
                )
            )
        }
    }
    fn get_mut_1_2(&mut self, indicies: [usize; 2]) -> Option<(&mut T, &mut Vec2P<T>)> {
        if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut_2(indicies[1])?,
                )
            )
        }
    }
    fn get_mut_1_3(&mut self, indicies: [usize; 2]) -> Option<(&mut T, &mut Vec3P<T>)> {
        if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 || indicies[0] == indicies[1] + 2 {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut_3(indicies[1])?,
                )
            )
        }
    }
    fn get_mut_2_1(&mut self, indicies: [usize; 2]) -> Option<(&mut Vec2P<T>, &mut T)> {
        if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut_2(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[1])?,
                )
            )
        }
    }
    fn get_mut_2_2(&mut self, indicies: [usize; 2]) -> Option<(&mut Vec2P<T>, &mut Vec2P<T>)> {
        if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 || indicies[0] + 1 == indicies[1] {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut_2(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut_2(indicies[1])?,
                )
            )
        }
    }
    fn get_mut_3_1(&mut self, indicies: [usize; 2]) -> Option<(&mut Vec3P<T>, &mut T)> {
        if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] || indicies[0] + 2 == indicies[1] {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut_3(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[1])?,
                )
            )
        }
    }
    fn get_mut_1_1_1(&mut self, indicies: [usize; 3]) -> Option<(&mut T, &mut T, &mut T)> {
        if indicies[0] == indicies[1] {
            None
        } else if indicies[0] == indicies[2] {
            None
        } else if indicies[1] == indicies[2] {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[1])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[2])?,
                )
            )
        }
    }
    fn get_mut_1_1_2(&mut self, indicies: [usize; 3]) -> Option<(&mut T, &mut T, &mut Vec2P<T>)> {
        if indicies[0] == indicies[1] {
            None
        } else if indicies[0] == indicies[2] || indicies[0] == indicies[2] + 1 {
            None
        } else if indicies[1] == indicies[2] || indicies[1] == indicies[2] + 1 {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[1])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut_2(indicies[2])?,
                )
            )
        }
    }
    fn get_mut_1_2_1(&mut self, indicies: [usize; 3]) -> Option<(&mut T, &mut Vec2P<T>, &mut T)> {
        if indicies[0] == indicies[1] || indicies[0] == indicies[1] + 1 {
            None
        } else if indicies[0] == indicies[2] {
            None
        } else if indicies[1] == indicies[2] || indicies[1] + 1 == indicies[2] {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut_2(indicies[1])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[2])?,
                )
            )
        }
    }
    fn get_mut_2_1_1(&mut self, indicies: [usize; 3]) -> Option<(&mut Vec2P<T>, &mut T, &mut T)> {
        if indicies[0] == indicies[1] || indicies[0] + 1 == indicies[1] {
            None
        } else if indicies[0] == indicies[2] || indicies[0] + 1 == indicies[2] {
            None
        } else if indicies[1] == indicies[2] {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut_2(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[1])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[2])?,
                )
            )
        }
    }
    fn get_mut_1_1_1_1(&mut self, indicies: [usize; 4]) -> Option<(&mut T, &mut T, &mut T, &mut T)> {
        if indicies[0] == indicies[1] {
            None
        } else if indicies[0] == indicies[2] {
            None
        } else if indicies[0] == indicies[3] {
            None
        } else if indicies[1] == indicies[2] {
            None
        } else if indicies[1] == indicies[3] {
            None
        } else if indicies[2] == indicies[3] {
            None
        } else {
            Some(
                (
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[0])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[1])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[2])?,
                    unsafe { transmute::<&mut Self, &mut Self>(self) }.get_mut(indicies[3])?,
                )
            )
        }
    }

    unsafe fn with(self, index: usize, value: T) -> Option<Self> {
        if index >= N {
            None
        } else {
            Some(unsafe { self.with_unchecked(index, value) })
        }
    }
    unsafe fn with_2(self, indicies: [usize; 2], values: Vector2<T, A>) -> Option<Self> {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else if indicies[0] == indicies[1] {
            None
        } else {
            Some(unsafe { self.with_2_unchecked(indicies, values) })
        }
    }
    unsafe fn with_3(self, indicies: [usize; 3], values: Vector3<T, A>) -> Option<Self> {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else if indicies[0] == indicies[1] {
            None
        } else if indicies[0] == indicies[2] {
            None
        } else if indicies[1] == indicies[2] {
            None
        } else {
            Some(unsafe { self.with_3_unchecked(indicies, values) })
        }
    }
    unsafe fn with_4(self, indicies: [usize; 4], values: Vector4<T, A>) -> Option<Self> {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else if indicies[0] == indicies[1] {
            None
        } else if indicies[0] == indicies[2] {
            None
        } else if indicies[0] == indicies[3] {
            None
        } else if indicies[1] == indicies[2] {
            None
        } else if indicies[1] == indicies[3] {
            None
        } else if indicies[2] == indicies[3] {
            None
        } else {
            Some(unsafe { self.with_4_unchecked(indicies, values) })
        }
    }
    unsafe fn with_unchecked(self, index: usize, value: T) -> Self;
    unsafe fn with_2_unchecked(self, indicies: [usize; 2], values: Vector2<T, A>) -> Self;
    unsafe fn with_3_unchecked(self, indicies: [usize; 3], values: Vector3<T, A>) -> Self;
    unsafe fn with_4_unchecked(self, indicies: [usize; 4], values: Vector4<T, A>) -> Self;

    fn set(&mut self, index: usize, value: T) -> Result<(), ()> {
        if index >= N {
            Err(())
        } else {
            unsafe { self.set_unchecked(index, value) }
            Ok(())
        }
    }
    fn set_2(&mut self, indicies: [usize; 2], values: Vector2<T, A>) -> Result<(), ()> {
        if indicies.into_iter().any(|index| index >= N) {
            Err(())
        } else if indicies[0] == indicies[1] {
            Err(())
        } else {
            unsafe { self.set_2_unchecked(indicies, values) }
            Ok(())
        }
    }
    fn set_3(&mut self, indicies: [usize; 3], values: Vector3<T, A>) -> Result<(), ()> {
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
    fn set_4(&mut self, indicies: [usize; 4], values: Vector4<T, A>) -> Result<(), ()> {
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
    unsafe fn set_unchecked(&mut self, index: usize, value: T) {
        *self = self.with_unchecked(index, value)
    }
    unsafe fn set_2_unchecked(&mut self, indicies: [usize; 2], values: Vector2<T, A>) {
        *self = self.with_2_unchecked(indicies, values)
    }
    unsafe fn set_3_unchecked(&mut self, indicies: [usize; 3], values: Vector3<T, A>) {
        *self = self.with_3_unchecked(indicies, values)
    }
    unsafe fn set_4_unchecked(&mut self, indicies: [usize; 4], values: Vector4<T, A>) {
        *self = self.with_4_unchecked(indicies, values)
    }
);
