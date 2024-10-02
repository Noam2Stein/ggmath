use super::*;

impl<T: ElementDefaultImpl> ElementVecWith for T {
    fn vec2_with(
        mut vec: Self::InnerVec2,
        x: usize,
        value: Self,
    ) -> Result<Self::InnerVec2, &'static str> {
        match vec.get_mut(x) {
            Some(some) => *some = value,
            None => return Err("x is out of vec2 bounds"),
        };
        Ok(vec)
    }
    fn vec2_with2(
        mut vec: Self::InnerVec2,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Result<Self::InnerVec2, &'static str> {
        match vec.get_mut(x) {
            Some(some) => *some = unsafe { T::vec2_cget::<0>(value) },
            None => return Err("x is out of vec2 bounds"),
        };
        match vec.get_mut(y) {
            Some(some) => *some = unsafe { T::vec2_cget::<1>(value) },
            None => return Err("y is out of vec2 bounds"),
        };
        Ok(vec)
    }

    fn vec3_with(
        mut vec: Self::InnerVec3,
        x: usize,
        value: Self,
    ) -> Result<Self::InnerVec3, &'static str> {
        if x < 3 {
            *unsafe { vec.get_unchecked_mut(x) } = value;
        } else {
            return Err("x is out of vec3 bounds");
        };
        Ok(vec)
    }
    fn vec3_with2(
        mut vec: Self::InnerVec3,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Result<Self::InnerVec3, &'static str> {
        if x < 3 {
            *unsafe { vec.get_unchecked_mut(x) } = unsafe { T::vec2_cget::<0>(value) };
        } else {
            return Err("x is out of vec3 bounds");
        };
        if y < 3 {
            *unsafe { vec.get_unchecked_mut(y) } = unsafe { T::vec2_cget::<1>(value) };
        } else {
            return Err("y is out of vec3 bounds");
        };
        Ok(vec)
    }
    fn vec3_with3(
        mut vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
        value: Self::InnerVec3,
    ) -> Result<Self::InnerVec3, &'static str> {
        if x < 3 {
            *unsafe { vec.get_unchecked_mut(x) } = unsafe { T::vec3_cget::<0>(value) };
        } else {
            return Err("x is out of vec3 bounds");
        };
        if y < 3 {
            *unsafe { vec.get_unchecked_mut(y) } = unsafe { T::vec3_cget::<1>(value) };
        } else {
            return Err("y is out of vec3 bounds");
        };
        if z < 3 {
            *unsafe { vec.get_unchecked_mut(z) } = unsafe { T::vec3_cget::<2>(value) };
        } else {
            return Err("z is out of vec3 bounds");
        };
        Ok(vec)
    }

    fn vec4_with(
        mut vec: Self::InnerVec4,
        x: usize,
        value: Self,
    ) -> Result<Self::InnerVec4, &'static str> {
        match vec.get_mut(x) {
            Some(some) => *some = value,
            None => return Err("x is out of vec4 bounds"),
        };
        Ok(vec)
    }
    fn vec4_with2(
        mut vec: Self::InnerVec4,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Result<Self::InnerVec4, &'static str> {
        match vec.get_mut(x) {
            Some(some) => *some = unsafe { T::vec2_cget::<0>(value) },
            None => return Err("x is out of vec4 bounds"),
        };
        match vec.get_mut(y) {
            Some(some) => *some = unsafe { T::vec2_cget::<1>(value) },
            None => return Err("y is out of vec4 bounds"),
        };
        Ok(vec)
    }
    fn vec4_with3(
        mut vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        value: Self::InnerVec3,
    ) -> Result<Self::InnerVec4, &'static str> {
        match vec.get_mut(x) {
            Some(some) => *some = unsafe { T::vec3_cget::<0>(value) },
            None => return Err("x is out of vec4 bounds"),
        };
        match vec.get_mut(y) {
            Some(some) => *some = unsafe { T::vec3_cget::<1>(value) },
            None => return Err("y is out of vec4 bounds"),
        };
        match vec.get_mut(z) {
            Some(some) => *some = unsafe { T::vec3_cget::<2>(value) },
            None => return Err("z is out of vec4 bounds"),
        };
        Ok(vec)
    }
    fn vec4_with4(
        mut vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        value: Self::InnerVec4,
    ) -> Result<Self::InnerVec4, &'static str> {
        match vec.get_mut(x) {
            Some(some) => *some = unsafe { T::vec4_cget::<0>(value) },
            None => return Err("x is out of vec4 bounds"),
        };
        match vec.get_mut(y) {
            Some(some) => *some = unsafe { T::vec4_cget::<1>(value) },
            None => return Err("y is out of vec4 bounds"),
        };
        match vec.get_mut(z) {
            Some(some) => *some = unsafe { T::vec4_cget::<2>(value) },
            None => return Err("z is out of vec4 bounds"),
        };
        match vec.get_mut(w) {
            Some(some) => *some = unsafe { T::vec4_cget::<3>(value) },
            None => return Err("w is out of vec4 bounds"),
        };
        Ok(vec)
    }

    unsafe fn vec2_with_unchecked(
        mut vec: Self::InnerVec2,
        x: usize,
        value: Self,
    ) -> Self::InnerVec2 {
        *vec.get_unchecked_mut(x) = value;
        vec
    }
    unsafe fn vec2_with2_unchecked(
        mut vec: Self::InnerVec2,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Self::InnerVec2 {
        *vec.get_unchecked_mut(x) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(y) = *value.get_unchecked(1);
        vec
    }

    unsafe fn vec3_with_unchecked(
        mut vec: Self::InnerVec3,
        x: usize,
        value: Self,
    ) -> Self::InnerVec3 {
        *vec.get_unchecked_mut(x) = value;
        vec
    }
    unsafe fn vec3_with2_unchecked(
        mut vec: Self::InnerVec3,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Self::InnerVec3 {
        *vec.get_unchecked_mut(x) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(y) = *value.get_unchecked(1);
        vec
    }
    unsafe fn vec3_with3_unchecked(
        mut vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
        value: Self::InnerVec3,
    ) -> Self::InnerVec3 {
        *vec.get_unchecked_mut(x) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(y) = *value.get_unchecked(1);
        *vec.get_unchecked_mut(z) = *value.get_unchecked(2);
        vec
    }

    unsafe fn vec4_with_unchecked(
        mut vec: Self::InnerVec4,
        x: usize,
        value: Self,
    ) -> Self::InnerVec4 {
        *vec.get_unchecked_mut(x) = value;
        vec
    }
    unsafe fn vec4_with2_unchecked(
        mut vec: Self::InnerVec4,
        x: usize,
        y: usize,
        value: Self::InnerVec2,
    ) -> Self::InnerVec4 {
        *vec.get_unchecked_mut(x) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(y) = *value.get_unchecked(1);
        vec
    }
    unsafe fn vec4_with3_unchecked(
        mut vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        value: Self::InnerVec3,
    ) -> Self::InnerVec4 {
        *vec.get_unchecked_mut(x) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(y) = *value.get_unchecked(1);
        *vec.get_unchecked_mut(z) = *value.get_unchecked(2);
        vec
    }
    unsafe fn vec4_with4_unchecked(
        mut vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        value: Self::InnerVec4,
    ) -> Self::InnerVec4 {
        *vec.get_unchecked_mut(x) = *value.get_unchecked(0);
        *vec.get_unchecked_mut(y) = *value.get_unchecked(1);
        *vec.get_unchecked_mut(z) = *value.get_unchecked(2);
        *vec.get_unchecked_mut(w) = *value.get_unchecked(3);
        vec
    }
}
