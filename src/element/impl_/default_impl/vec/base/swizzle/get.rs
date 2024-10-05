use super::*;

impl<T: ElementDefaultImpl> ElementVecGet for T {
    fn vec2_get(vec: Self::InnerVec2, x: usize) -> Result<Self, &'static str> {
        Ok(match vec.get(x) {
            Some(some) => *some,
            None => return Err("x is out of vec2 bounds"),
        })
    }
    fn vec2_get2(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
    ) -> Result<Self::InnerVec2, &'static str> {
        Ok([
            match vec.get(x) {
                Some(some) => *some,
                None => return Err("x is out of vec2 bounds"),
            },
            match vec.get(y) {
                Some(some) => *some,
                None => return Err("y is out of vec2 bounds"),
            },
        ])
    }
    fn vec2_get3(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
        z: usize,
    ) -> Result<Self::InnerVec3, &'static str> {
        Ok([
            match vec.get(x) {
                Some(some) => *some,
                None => return Err("x is out of vec2 bounds"),
            },
            match vec.get(y) {
                Some(some) => *some,
                None => return Err("y is out of vec2 bounds"),
            },
            match vec.get(z) {
                Some(some) => *some,
                None => return Err("z is out of vec2 bounds"),
            },
            T::default(),
        ])
    }
    fn vec2_get4(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Result<Self::InnerVec4, &'static str> {
        Ok([
            match vec.get(x) {
                Some(some) => *some,
                None => return Err("x is out of vec2 bounds"),
            },
            match vec.get(y) {
                Some(some) => *some,
                None => return Err("y is out of vec2 bounds"),
            },
            match vec.get(z) {
                Some(some) => *some,
                None => return Err("z is out of vec2 bounds"),
            },
            match vec.get(w) {
                Some(some) => *some,
                None => return Err("w is out of vec2 bounds"),
            },
        ])
    }

    fn vec3_get(vec: Self::InnerVec3, x: usize) -> Result<Self, &'static str> {
        Ok(if x < 3 {
            unsafe { *vec.get_unchecked(x) }
        } else {
            return Err("x is out of vec3 bounds");
        })
    }
    fn vec3_get2(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
    ) -> Result<Self::InnerVec2, &'static str> {
        Ok([
            if x < 3 {
                unsafe { *vec.get_unchecked(x) }
            } else {
                return Err("x is out of vec3 bounds");
            },
            if y < 3 {
                unsafe { *vec.get_unchecked(y) }
            } else {
                return Err("y is out of vec3 bounds");
            },
        ])
    }
    fn vec3_get3(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
    ) -> Result<Self::InnerVec3, &'static str> {
        Ok([
            if x < 3 {
                unsafe { *vec.get_unchecked(x) }
            } else {
                return Err("x is out of vec3 bounds");
            },
            if y < 3 {
                unsafe { *vec.get_unchecked(y) }
            } else {
                return Err("y is out of vec3 bounds");
            },
            if z < 3 {
                unsafe { *vec.get_unchecked(z) }
            } else {
                return Err("z is out of vec3 bounds");
            },
            T::default(),
        ])
    }
    fn vec3_get4(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Result<Self::InnerVec4, &'static str> {
        Ok([
            if x < 3 {
                unsafe { *vec.get_unchecked(x) }
            } else {
                return Err("x is out of vec3 bounds");
            },
            if y < 3 {
                unsafe { *vec.get_unchecked(y) }
            } else {
                return Err("y is out of vec3 bounds");
            },
            if z < 3 {
                unsafe { *vec.get_unchecked(z) }
            } else {
                return Err("z is out of vec3 bounds");
            },
            if w < 3 {
                unsafe { *vec.get_unchecked(w) }
            } else {
                return Err("w is out of vec3 bounds");
            },
        ])
    }

    fn vec4_get(vec: Self::InnerVec4, x: usize) -> Result<Self, &'static str> {
        Ok(match vec.get(x) {
            Some(some) => *some,
            None => return Err("x is out of vec4 bounds"),
        })
    }
    fn vec4_get2(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
    ) -> Result<Self::InnerVec2, &'static str> {
        Ok([
            match vec.get(x) {
                Some(some) => *some,
                None => return Err("x is out of vec4 bounds"),
            },
            match vec.get(y) {
                Some(some) => *some,
                None => return Err("y is out of vec4 bounds"),
            },
        ])
    }
    fn vec4_get3(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
    ) -> Result<Self::InnerVec3, &'static str> {
        Ok([
            match vec.get(x) {
                Some(some) => *some,
                None => return Err("x is out of vec4 bounds"),
            },
            match vec.get(y) {
                Some(some) => *some,
                None => return Err("y is out of vec4 bounds"),
            },
            match vec.get(z) {
                Some(some) => *some,
                None => return Err("z is out of vec4 bounds"),
            },
            T::default(),
        ])
    }
    fn vec4_get4(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Result<Self::InnerVec4, &'static str> {
        Ok([
            match vec.get(x) {
                Some(some) => *some,
                None => return Err("x is out of vec4 bounds"),
            },
            match vec.get(y) {
                Some(some) => *some,
                None => return Err("y is out of vec4 bounds"),
            },
            match vec.get(z) {
                Some(some) => *some,
                None => return Err("z is out of vec4 bounds"),
            },
            match vec.get(w) {
                Some(some) => *some,
                None => return Err("w is out of vec4 bounds"),
            },
        ])
    }

    unsafe fn vec2_get_unchecked(vec: Self::InnerVec2, x: usize) -> Self {
        *vec.get_unchecked(x)
    }
    unsafe fn vec2_get2_unchecked(vec: Self::InnerVec2, x: usize, y: usize) -> Self::InnerVec2 {
        [*vec.get_unchecked(x), *vec.get_unchecked(y)]
    }
    unsafe fn vec2_get3_unchecked(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
        z: usize,
    ) -> Self::InnerVec3 {
        [
            *vec.get_unchecked(x),
            *vec.get_unchecked(y),
            *vec.get_unchecked(z),
            T::default(),
        ]
    }
    unsafe fn vec2_get4_unchecked(
        vec: Self::InnerVec2,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Self::InnerVec4 {
        [
            *vec.get_unchecked(x),
            *vec.get_unchecked(y),
            *vec.get_unchecked(z),
            *vec.get_unchecked(w),
        ]
    }

    unsafe fn vec3_get_unchecked(vec: Self::InnerVec3, x: usize) -> Self {
        *vec.get_unchecked(x)
    }
    unsafe fn vec3_get2_unchecked(vec: Self::InnerVec3, x: usize, y: usize) -> Self::InnerVec2 {
        [*vec.get_unchecked(x), *vec.get_unchecked(y)]
    }
    unsafe fn vec3_get3_unchecked(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
    ) -> Self::InnerVec3 {
        [
            *vec.get_unchecked(x),
            *vec.get_unchecked(y),
            *vec.get_unchecked(z),
            T::default(),
        ]
    }
    unsafe fn vec3_get4_unchecked(
        vec: Self::InnerVec3,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Self::InnerVec4 {
        [
            *vec.get_unchecked(x),
            *vec.get_unchecked(y),
            *vec.get_unchecked(z),
            *vec.get_unchecked(w),
        ]
    }

    unsafe fn vec4_get_unchecked(vec: Self::InnerVec4, x: usize) -> Self {
        *vec.get_unchecked(x)
    }
    unsafe fn vec4_get2_unchecked(vec: Self::InnerVec4, x: usize, y: usize) -> Self::InnerVec2 {
        [*vec.get_unchecked(x), *vec.get_unchecked(y)]
    }
    unsafe fn vec4_get3_unchecked(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
    ) -> Self::InnerVec3 {
        [
            *vec.get_unchecked(x),
            *vec.get_unchecked(y),
            *vec.get_unchecked(z),
            T::default(),
        ]
    }
    unsafe fn vec4_get4_unchecked(
        vec: Self::InnerVec4,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    ) -> Self::InnerVec4 {
        [
            *vec.get_unchecked(x),
            *vec.get_unchecked(y),
            *vec.get_unchecked(z),
            *vec.get_unchecked(w),
        ]
    }
}
