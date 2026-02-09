use std::{
    fmt::{Debug, Display},
    panic::{RefUnwindSafe, UnwindSafe},
};

use ggmath::{Alignment, Matrix, Scalar, Vector};

use crate::assert_panic;

pub fn test<T, A: Alignment>(x: T, y: T, z: T, w: T)
where
    T: Scalar + Debug + Display + PartialEq + PartialOrd + Default + UnwindSafe + RefUnwindSafe,
{
    macro_rules! vec2 {
        ($($arg:expr),*$(,)?) => {
            ggmath::Vector::<2, T, A>::from(($($arg,)*))
        };
    }

    macro_rules! vec3 {
        ($($arg:expr),*$(,)?) => {
            ggmath::Vector::<3, T, A>::from(($($arg,)*))
        };
    }

    macro_rules! vec4 {
        ($($arg:expr),*$(,)?) => {
            ggmath::Vector::<4, T, A>::from(($($arg,)*))
        };
    }

    macro_rules! mat2 {
        ($($arg:expr),*$(,)?) => {
            ggmath::Matrix::<2, T, A>::from(($($arg,)*))
        };
    }

    macro_rules! mat3 {
        ($($arg:expr),*$(,)?) => {
            ggmath::Matrix::<3, T, A>::from(($($arg,)*))
        };
    }

    macro_rules! mat4 {
        ($($arg:expr),*$(,)?) => {
            ggmath::Matrix::<4, T, A>::from(($($arg,)*))
        };
    }

    assert!(y > x && w > z);

    assert!(size_of::<Matrix<2, T, A>>() == size_of::<Vector<2, T, A>>() * 2);
    assert!(
        size_of::<Matrix<3, T, A>>() == size_of::<Vector<3, T, A>>() * 3
            || size_of::<Matrix<3, T, A>>() == size_of::<Vector<3, T, A>>() * 4
    );
    assert!(size_of::<Matrix<4, T, A>>() == size_of::<Vector<4, T, A>>() * 4);

    assert!(
        align_of::<Matrix<2, T, A>>() == align_of::<T>()
            || align_of::<Matrix<2, T, A>>() == align_of::<Vector<2, T, A>>()
            || align_of::<Matrix<2, T, A>>() == size_of::<Vector<2, T, A>>() * 2
    );
    assert!(
        align_of::<Matrix<3, T, A>>() == align_of::<T>()
            || align_of::<Matrix<3, T, A>>() == align_of::<Vector<3, T, A>>()
            || align_of::<Matrix<3, T, A>>() == size_of::<Vector<3, T, A>>() * 4
    );
    assert!(
        align_of::<Matrix<4, T, A>>() == align_of::<T>()
            || align_of::<Matrix<4, T, A>>() == align_of::<Vector<4, T, A>>()
            || align_of::<Matrix<4, T, A>>() == size_of::<Vector<4, T, A>>() * 4
    );

    assert_eq!(
        Matrix::<2, T, A>::from_col_array(&[vec2!(x, y), vec2!(z, x)]).to_col_array(),
        [vec2!(x, y), vec2!(z, x)]
    );
    assert_eq!(
        Matrix::<3, T, A>::from_col_array(&[vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)])
            .to_col_array(),
        [vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)]
    );
    assert_eq!(
        Matrix::<4, T, A>::from_col_array(&[
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ])
        .to_col_array(),
        [
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ]
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, x)).to_col_array(),
        [vec2!(x, y), vec2!(z, x)]
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).to_col_array(),
        [vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)]
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .to_col_array(),
        [
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ]
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, x))
            .align()
            .to_alignment::<A>()
            .to_col_array(),
        [vec2!(x, y), vec2!(z, x)]
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y))
            .align()
            .to_alignment::<A>()
            .to_col_array(),
        [vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)]
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .align()
        .to_alignment::<A>()
        .to_col_array(),
        [
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ]
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, x))
            .unalign()
            .to_alignment::<A>()
            .to_col_array(),
        [vec2!(x, y), vec2!(z, x)]
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y))
            .unalign()
            .to_alignment::<A>()
            .to_col_array(),
        [vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)]
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .unalign()
        .to_alignment::<A>()
        .to_col_array(),
        [
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ]
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, x)).as_col_array_ref(),
        &[vec2!(x, y), vec2!(z, x)]
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).as_col_array_ref(),
        &[vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)]
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .as_col_array_ref(),
        &[
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ]
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, x)).as_col_array_mut(),
        &mut [vec2!(x, y), vec2!(z, x)]
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).as_col_array_mut(),
        &mut [vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)]
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .as_col_array_mut(),
        &mut [
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ]
    );

    assert_eq!(mat2!(vec2!(x, y), vec2!(z, x)).x_axis, vec2!(x, y));
    assert_eq!(mat2!(vec2!(x, y), vec2!(z, x)).y_axis, vec2!(z, x));
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).x_axis,
        vec3!(x, y, z)
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).y_axis,
        vec3!(y, z, x)
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).z_axis,
        vec3!(z, x, y)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .x_axis,
        vec4!(x, y, z, x)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .y_axis,
        vec4!(y, z, x, y)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .z_axis,
        vec4!(z, x, y, z)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .w_axis,
        vec4!(y, x, z, x)
    );

    assert_eq!(
        &mut mat2!(vec2!(x, y), vec2!(z, x)).x_axis,
        &mut vec2!(x, y)
    );
    assert_eq!(
        &mut mat2!(vec2!(x, y), vec2!(z, x)).y_axis,
        &mut vec2!(z, x)
    );
    assert_eq!(
        &mut mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).x_axis,
        &mut vec3!(x, y, z)
    );
    assert_eq!(
        &mut mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).y_axis,
        &mut vec3!(y, z, x)
    );
    assert_eq!(
        &mut mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).z_axis,
        &mut vec3!(z, x, y)
    );
    assert_eq!(
        &mut mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .x_axis,
        &mut vec4!(x, y, z, x)
    );
    assert_eq!(
        &mut mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .y_axis,
        &mut vec4!(y, z, x, y)
    );
    assert_eq!(
        &mut mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .z_axis,
        &mut vec4!(z, x, y, z)
    );
    assert_eq!(
        &mut mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .w_axis,
        &mut vec4!(y, x, z, x)
    );

    assert_eq!(mat2!(vec2!(x, y), vec2!(z, x)).col(0), vec2!(x, y));
    assert_eq!(mat2!(vec2!(x, y), vec2!(z, x)).col(1), vec2!(z, x));
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).col(0),
        vec3!(x, y, z)
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).col(1),
        vec3!(y, z, x)
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).col(2),
        vec3!(z, x, y)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .col(0),
        vec4!(x, y, z, x)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .col(1),
        vec4!(y, z, x, y)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .col(2),
        vec4!(z, x, y, z)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .col(3),
        vec4!(y, x, z, x)
    );

    assert_panic!(mat2!(vec2!(x, y), vec2!(z, x)).col(2));
    assert_panic!(mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).col(3));
    assert_panic!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .col(4)
    );

    assert_eq!(mat2!(vec2!(x, y), vec2!(z, x)).col_mut(0), &mut vec2!(x, y));
    assert_eq!(mat2!(vec2!(x, y), vec2!(z, x)).col_mut(1), &mut vec2!(z, x));
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).col_mut(0),
        &mut vec3!(x, y, z)
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).col_mut(1),
        &mut vec3!(y, z, x)
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).col_mut(2),
        &mut vec3!(z, x, y)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .col_mut(0),
        &mut vec4!(x, y, z, x)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .col_mut(1),
        &mut vec4!(y, z, x, y)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .col_mut(2),
        &mut vec4!(z, x, y, z)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .col_mut(3),
        &mut vec4!(y, x, z, x)
    );

    assert_panic!(mat2!(vec2!(x, y), vec2!(z, x)).col_mut(2));
    assert_panic!(mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).col_mut(3));
    assert_panic!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        )
        .col_mut(4)
    );

    assert_eq!(mat2!(vec2!(x, z), vec2!(y, x)).row(0), vec2!(x, y));
    assert_eq!(mat2!(vec2!(x, z), vec2!(y, x)).row(1), vec2!(z, x));
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).row(0),
        vec3!(x, y, z)
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).row(1),
        vec3!(y, z, x)
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).row(2),
        vec3!(z, x, y)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, y),
            vec4!(y, z, x, x),
            vec4!(z, x, y, z),
            vec4!(x, y, z, x)
        )
        .row(0),
        vec4!(x, y, z, x)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, y),
            vec4!(y, z, x, x),
            vec4!(z, x, y, z),
            vec4!(x, y, z, x)
        )
        .row(1),
        vec4!(y, z, x, y)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, y),
            vec4!(y, z, x, x),
            vec4!(z, x, y, z),
            vec4!(x, y, z, x)
        )
        .row(2),
        vec4!(z, x, y, z)
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, y),
            vec4!(y, z, x, x),
            vec4!(z, x, y, z),
            vec4!(x, y, z, x)
        )
        .row(3),
        vec4!(y, x, z, x)
    );

    assert_panic!(mat2!(vec2!(x, z), vec2!(y, x)).row(2));
    assert_panic!(mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)).row(3));
    assert_panic!(
        mat4!(
            vec4!(x, y, z, y),
            vec4!(y, z, x, x),
            vec4!(z, x, y, z),
            vec4!(x, y, z, x)
        )
        .row(4)
    );

    let mut m = mat2!(vec2!(x, z), vec2!(y, x));
    m.set_row(0, vec2!(y, x));
    assert_eq!(m, mat2!(vec2!(y, z), vec2!(x, x)));
    m.set_row(1, vec2!(x, z));
    assert_eq!(m, mat2!(vec2!(y, x), vec2!(x, z)));
    assert_panic!(m.clone().set_row(2, vec2!(x, y)));

    let mut m = mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y));
    m.set_row(0, vec3!(y, x, x));
    assert_eq!(m, mat3!(vec3!(y, y, z), vec3!(x, z, x), vec3!(x, x, y)));
    m.set_row(1, vec3!(x, x, z));
    assert_eq!(m, mat3!(vec3!(y, x, z), vec3!(x, x, x), vec3!(x, z, y)));
    m.set_row(2, vec3!(x, y, x));
    assert_eq!(m, mat3!(vec3!(y, x, x), vec3!(x, x, y), vec3!(x, z, x)));
    assert_panic!(m.clone().set_row(3, vec3!(x, y, z)));

    let mut m = mat4!(
        vec4!(x, y, z, y),
        vec4!(y, z, x, x),
        vec4!(z, x, y, z),
        vec4!(x, y, z, x)
    );
    m.set_row(0, vec4!(y, x, x, z));
    assert_eq!(
        m,
        mat4!(
            vec4!(y, y, z, y),
            vec4!(x, z, x, x),
            vec4!(x, x, y, z),
            vec4!(z, y, z, x)
        )
    );
    m.set_row(1, vec4!(x, y, z, z));
    assert_eq!(
        m,
        mat4!(
            vec4!(y, x, z, y),
            vec4!(x, y, x, x),
            vec4!(x, z, y, z),
            vec4!(z, z, z, x)
        )
    );
    m.set_row(2, vec4!(x, y, x, y));
    assert_eq!(
        m,
        mat4!(
            vec4!(y, x, x, y),
            vec4!(x, y, y, x),
            vec4!(x, z, x, z),
            vec4!(z, z, y, x)
        )
    );
    m.set_row(3, vec4!(x, y, y, z));
    assert_eq!(
        m,
        mat4!(
            vec4!(y, x, x, x),
            vec4!(x, y, y, y),
            vec4!(x, z, x, y),
            vec4!(z, z, y, z)
        )
    );
    assert_panic!(m.clone().set_row(4, vec4!(x, y, z, x)));

    assert_eq!(
        format!("{:?}", mat2!(vec2!(x, y), vec2!(z, x))),
        format!("[({x:?}, {y:?}), ({z:?}, {x:?})]")
    );
    assert_eq!(
        format!(
            "{:?}",
            mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y))
        ),
        format!("[({x:?}, {y:?}, {z:?}), ({y:?}, {z:?}, {x:?}), ({z:?}, {x:?}, {y:?})]")
    );
    assert_eq!(
        format!(
            "{:?}",
            mat4!(
                vec4!(x, y, z, x),
                vec4!(y, z, x, y),
                vec4!(z, x, y, z),
                vec4!(y, x, z, x)
            )
        ),
        format!(
            "[({x:?}, {y:?}, {z:?}, {x:?}), ({y:?}, {z:?}, {x:?}, {y:?}), ({z:?}, {x:?}, {y:?}, {z:?}), ({y:?}, {x:?}, {z:?}, {x:?})]"
        )
    );

    assert_eq!(
        format!("{}", mat2!(vec2!(x, y), vec2!(z, x))),
        format!("[({x}, {y}), ({z}, {x})]")
    );
    assert_eq!(
        format!("{}", mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y))),
        format!("[({x}, {y}, {z}), ({y}, {z}, {x}), ({z}, {x}, {y})]")
    );
    assert_eq!(
        format!(
            "{}",
            mat4!(
                vec4!(x, y, z, x),
                vec4!(y, z, x, y),
                vec4!(z, x, y, z),
                vec4!(y, x, z, x)
            )
        ),
        format!(
            "[({x}, {y}, {z}, {x}), ({y}, {z}, {x}, {y}), ({z}, {x}, {y}, {z}), ({y}, {x}, {z}, {x})]"
        )
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, x)) == mat2!(vec2!(x, y), vec2!(z, x)),
        true
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y))
            == mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
        true
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ) == mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ),
        true
    );

    assert_eq!(
        mat2!(vec2!(y, y), vec2!(z, x)) == mat2!(vec2!(x, y), vec2!(z, x)),
        false
    );
    assert_eq!(
        mat3!(vec3!(y, y, z), vec3!(y, z, x), vec3!(z, x, y))
            == mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
        false
    );
    assert_eq!(
        mat4!(
            vec4!(y, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ) == mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ),
        false
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, y)) == mat2!(vec2!(x, y), vec2!(z, x)),
        false
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, y), vec3!(z, x, y))
            == mat3!(vec3!(x, y, z), vec3!(y, z, y), vec3!(z, x, x)),
        false
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ) == mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, y)
        ),
        false
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, y)) == mat2!(vec2!(y, x), vec2!(y, x)),
        false
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, y), vec3!(z, x, y))
            == mat3!(vec3!(y, x, x), vec3!(z, y, x), vec3!(x, y, z)),
        false
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ) == mat4!(
            vec4!(y, x, y, z),
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, y, y, z)
        ),
        false
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, x)) != mat2!(vec2!(x, y), vec2!(z, x)),
        false
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y))
            != mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
        false
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ) != mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ),
        false
    );

    assert_eq!(
        mat2!(vec2!(y, y), vec2!(z, x)) != mat2!(vec2!(x, y), vec2!(z, x)),
        true
    );
    assert_eq!(
        mat3!(vec3!(y, y, z), vec3!(y, z, x), vec3!(z, x, y))
            != mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
        true
    );
    assert_eq!(
        mat4!(
            vec4!(y, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ) != mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ),
        true
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, y)) != mat2!(vec2!(x, y), vec2!(z, x)),
        true
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, y), vec3!(z, x, y))
            != mat3!(vec3!(x, y, z), vec3!(y, z, y), vec3!(z, x, x)),
        true
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ) != mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, y)
        ),
        true
    );

    assert_eq!(
        mat2!(vec2!(x, y), vec2!(z, y)) != mat2!(vec2!(y, x), vec2!(y, x)),
        true
    );
    assert_eq!(
        mat3!(vec3!(x, y, z), vec3!(y, z, y), vec3!(z, x, y))
            != mat3!(vec3!(y, x, x), vec3!(z, y, x), vec3!(x, y, z)),
        true
    );
    assert_eq!(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(y, x, z, x)
        ) != mat4!(
            vec4!(y, x, y, z),
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, y, y, z)
        ),
        true
    );
}
