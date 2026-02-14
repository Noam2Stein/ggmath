use std::{
    fmt::{Debug, Display},
    panic::{RefUnwindSafe, UnwindSafe},
};

use ggmath::{Affine, Alignment, Matrix, Scalar, Vector, mat2, mat3, mat4, vec2, vec3, vec4};

pub fn test<T, A: Alignment>(x: T, y: T, z: T, w: T)
where
    T: Scalar + Debug + Display + PartialEq + PartialOrd + Default + UnwindSafe + RefUnwindSafe,
{
    assert!(y > x && w > z);

    assert!(
        size_of::<Affine<2, T, A>>() == size_of::<Matrix<2, T, A>>() + size_of::<Vector<2, T, A>>()
            || size_of::<Affine<2, T, A>>() == size_of::<Vector<2, T, A>>() * 4
    );
    assert!(
        size_of::<Affine<3, T, A>>() == size_of::<Matrix<3, T, A>>() + size_of::<Vector<3, T, A>>()
            || size_of::<Matrix<3, T, A>>() == size_of::<[T; 9]>()
                && size_of::<Vector<3, T, A>>() == size_of::<[T; 3]>()
                && size_of::<Affine<3, T, A>>() == size_of::<[T; 16]>()
    );
    assert!(
        size_of::<Affine<4, T, A>>() == size_of::<Matrix<4, T, A>>() + size_of::<Vector<4, T, A>>()
    );

    assert!(
        align_of::<Affine<2, T, A>>() == align_of::<Matrix<2, T, A>>()
            || align_of::<Affine<2, T, A>>() == size_of::<Vector<2, T, A>>() * 4
    );
    assert!(
        align_of::<Affine<3, T, A>>() == align_of::<Matrix<3, T, A>>()
            || size_of::<Matrix<3, T, A>>() == size_of::<[T; 9]>()
                && size_of::<Vector<3, T, A>>() == size_of::<[T; 3]>()
                && align_of::<Affine<3, T, A>>() == size_of::<[T; 16]>()
    );
    assert!(align_of::<Affine<4, T, A>>() == align_of::<Matrix<4, T, A>>());

    assert_eq!(
        Affine::<2, T, A>::from_cols(vec2!(x, y), vec2!(y, z), vec2!(z, x)).to_col_array(),
        [vec2!(x, y), vec2!(y, z), vec2!(z, x)]
    );
    assert_eq!(
        Affine::<3, T, A>::from_cols(
            vec3!(x, y, z),
            vec3!(y, z, x),
            vec3!(z, x, y),
            vec3!(z, y, x)
        )
        .to_col_array(),
        [
            vec3!(x, y, z),
            vec3!(y, z, x),
            vec3!(z, x, y),
            vec3!(z, y, x)
        ]
    );
    assert_eq!(
        Affine::<4, T, A>::from_cols(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z),
            vec4!(z, z, y, x)
        )
        .to_col_array(),
        [
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z),
            vec4!(z, z, y, x)
        ]
    );

    assert_eq!(
        Affine::<2, T, A>::from_col_array(&[vec2!(x, y), vec2!(y, z), vec2!(z, x)]).to_col_array(),
        [vec2!(x, y), vec2!(y, z), vec2!(z, x)]
    );
    assert_eq!(
        Affine::<3, T, A>::from_col_array(&[
            vec3!(x, y, z),
            vec3!(y, z, x),
            vec3!(z, x, y),
            vec3!(z, y, x)
        ])
        .to_col_array(),
        [
            vec3!(x, y, z),
            vec3!(y, z, x),
            vec3!(z, x, y),
            vec3!(z, y, x)
        ]
    );
    assert_eq!(
        Affine::<4, T, A>::from_col_array(&[
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z),
            vec4!(z, z, y, x)
        ])
        .to_col_array(),
        [
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z),
            vec4!(z, z, y, x)
        ]
    );

    assert_eq!(
        Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(y, z)), vec2!(z, x))
            .to_col_array(),
        [vec2!(x, y), vec2!(y, z), vec2!(z, x)]
    );
    assert_eq!(
        Affine::<3, T, A>::from_mat_translation(
            mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
            vec3!(z, y, x)
        )
        .to_col_array(),
        [
            vec3!(x, y, z),
            vec3!(y, z, x),
            vec3!(z, x, y),
            vec3!(z, y, x)
        ]
    );
    assert_eq!(
        Affine::<4, T, A>::from_mat_translation(
            mat4!(
                vec4!(x, y, z, x),
                vec4!(y, z, x, y),
                vec4!(z, x, y, z),
                vec4!(z, y, x, z)
            ),
            vec4!(z, z, y, x)
        )
        .to_col_array(),
        [
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z),
            vec4!(z, z, y, x)
        ]
    );

    assert_eq!(
        Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(y, z)), vec2!(z, x))
            .matrix,
        mat2!(vec2!(x, y), vec2!(y, z))
    );
    assert_eq!(
        Affine::<3, T, A>::from_mat_translation(
            mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
            vec3!(z, y, x)
        )
        .matrix,
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y))
    );
    assert_eq!(
        Affine::<4, T, A>::from_mat_translation(
            mat4!(
                vec4!(x, y, z, x),
                vec4!(y, z, x, y),
                vec4!(z, x, y, z),
                vec4!(z, y, x, z)
            ),
            vec4!(z, z, y, x)
        )
        .matrix,
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z)
        )
    );

    assert_eq!(
        Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(y, z)), vec2!(z, x))
            .translation,
        vec2!(z, x)
    );
    assert_eq!(
        Affine::<3, T, A>::from_mat_translation(
            mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
            vec3!(z, y, x)
        )
        .translation,
        vec3!(z, y, x)
    );
    assert_eq!(
        Affine::<4, T, A>::from_mat_translation(
            mat4!(
                vec4!(x, y, z, x),
                vec4!(y, z, x, y),
                vec4!(z, x, y, z),
                vec4!(z, y, x, z)
            ),
            vec4!(z, z, y, x)
        )
        .translation,
        vec4!(z, z, y, x)
    );

    let mut a =
        Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(y, z)), vec2!(z, x));
    a.matrix = mat2!(vec2!(x, x), vec2!(x, z));
    assert_eq!(
        a,
        Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, x), vec2!(x, z)), vec2!(z, x))
    );

    let mut a = Affine::<3, T, A>::from_mat_translation(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
        vec3!(z, y, x),
    );
    a.matrix = mat3!(vec3!(x, y, x), vec3!(y, y, x), vec3!(y, x, y));
    assert_eq!(
        a,
        Affine::<3, T, A>::from_mat_translation(
            mat3!(vec3!(x, y, x), vec3!(y, y, x), vec3!(y, x, y)),
            vec3!(z, y, x),
        )
    );

    let mut a = Affine::<4, T, A>::from_mat_translation(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z)
        ),
        vec4!(z, z, y, x),
    );
    a.matrix = mat4!(
        vec4!(x, y, x, x),
        vec4!(y, x, x, y),
        vec4!(z, x, y, y),
        vec4!(z, x, x, z)
    );
    assert_eq!(
        a,
        Affine::<4, T, A>::from_mat_translation(
            mat4!(
                vec4!(x, y, x, x),
                vec4!(y, x, x, y),
                vec4!(z, x, y, y),
                vec4!(z, x, x, z)
            ),
            vec4!(z, z, y, x),
        )
    );

    let mut a =
        Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(y, z)), vec2!(z, x));
    a.translation = vec2!(y, y);
    assert_eq!(
        a,
        Affine::<2, T, A>::from_mat_translation(mat2!(vec2!(x, y), vec2!(y, z)), vec2!(y, y))
    );

    let mut a = Affine::<3, T, A>::from_mat_translation(
        mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
        vec3!(z, y, x),
    );
    a.translation = vec3!(y, x, y);
    assert_eq!(
        a,
        Affine::<3, T, A>::from_mat_translation(
            mat3!(vec3!(x, y, z), vec3!(y, z, x), vec3!(z, x, y)),
            vec3!(y, x, y)
        )
    );

    let mut a = Affine::<4, T, A>::from_mat_translation(
        mat4!(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z)
        ),
        vec4!(z, z, y, x),
    );
    a.translation = vec4!(y, y, z, y);
    assert_eq!(
        a,
        Affine::<4, T, A>::from_mat_translation(
            mat4!(
                vec4!(x, y, z, x),
                vec4!(y, z, x, y),
                vec4!(z, x, y, z),
                vec4!(z, y, x, z)
            ),
            vec4!(y, y, z, y)
        )
    );

    assert_eq!(
        Affine::<2, T, A>::from_cols(vec2!(x, y), vec2!(y, z), vec2!(z, x)).as_col_array_ref(),
        &[vec2!(x, y), vec2!(y, z), vec2!(z, x)]
    );
    assert_eq!(
        Affine::<3, T, A>::from_cols(
            vec3!(x, y, z),
            vec3!(y, z, x),
            vec3!(z, x, y),
            vec3!(z, y, x)
        )
        .as_col_array_ref(),
        &[
            vec3!(x, y, z),
            vec3!(y, z, x),
            vec3!(z, x, y),
            vec3!(z, y, x)
        ]
    );
    assert_eq!(
        Affine::<4, T, A>::from_cols(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z),
            vec4!(z, z, y, x)
        )
        .as_col_array_ref(),
        &[
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z),
            vec4!(z, z, y, x)
        ]
    );

    assert_eq!(
        Affine::<2, T, A>::from_cols(vec2!(x, y), vec2!(y, z), vec2!(z, x)).as_col_array_mut(),
        &mut [vec2!(x, y), vec2!(y, z), vec2!(z, x)]
    );
    assert_eq!(
        Affine::<3, T, A>::from_cols(
            vec3!(x, y, z),
            vec3!(y, z, x),
            vec3!(z, x, y),
            vec3!(z, y, x)
        )
        .as_col_array_mut(),
        &mut [
            vec3!(x, y, z),
            vec3!(y, z, x),
            vec3!(z, x, y),
            vec3!(z, y, x)
        ]
    );
    assert_eq!(
        Affine::<4, T, A>::from_cols(
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z),
            vec4!(z, z, y, x)
        )
        .as_col_array_mut(),
        &mut [
            vec4!(x, y, z, x),
            vec4!(y, z, x, y),
            vec4!(z, x, y, z),
            vec4!(z, y, x, z),
            vec4!(z, z, y, x)
        ]
    );

    assert_eq!(
        format!(
            "{:?}",
            Affine::<2, T, A>::from_cols(vec2!(x, y), vec2!(y, z), vec2!(z, x))
        ),
        format!("[({x:?}, {y:?}), ({y:?}, {z:?}), ({z:?}, {x:?})]")
    );
    assert_eq!(
        format!(
            "{:?}",
            Affine::<3, T, A>::from_cols(
                vec3!(x, y, z),
                vec3!(y, z, x),
                vec3!(z, x, y),
                vec3!(z, y, x)
            )
        ),
        format!(
            "[({x:?}, {y:?}, {z:?}), ({y:?}, {z:?}, {x:?}), ({z:?}, {x:?}, {y:?}), ({z:?}, {y:?}, {x:?})]"
        )
    );
    assert_eq!(
        format!(
            "{:?}",
            Affine::<4, T, A>::from_cols(
                vec4!(x, y, z, x),
                vec4!(y, z, x, y),
                vec4!(z, x, y, z),
                vec4!(z, y, x, z),
                vec4!(z, z, y, x)
            )
        ),
        format!(
            "[({x:?}, {y:?}, {z:?}, {x:?}), ({y:?}, {z:?}, {x:?}, {y:?}), ({z:?}, {x:?}, {y:?}, {z:?}), ({z:?}, {y:?}, {x:?}, {z:?}), ({z:?}, {z:?}, {y:?}, {x:?})]"
        )
    );

    assert_eq!(
        format!(
            "{}",
            Affine::<2, T, A>::from_cols(vec2!(x, y), vec2!(y, z), vec2!(z, x))
        ),
        format!("[({x}, {y}), ({y}, {z}), ({z}, {x})]")
    );
    assert_eq!(
        format!(
            "{}",
            Affine::<3, T, A>::from_cols(
                vec3!(x, y, z),
                vec3!(y, z, x),
                vec3!(z, x, y),
                vec3!(z, y, x)
            )
        ),
        format!("[({x}, {y}, {z}), ({y}, {z}, {x}), ({z}, {x}, {y}), ({z}, {y}, {x})]")
    );
    assert_eq!(
        format!(
            "{}",
            Affine::<4, T, A>::from_cols(
                vec4!(x, y, z, x),
                vec4!(y, z, x, y),
                vec4!(z, x, y, z),
                vec4!(z, y, x, z),
                vec4!(z, z, y, x)
            )
        ),
        format!(
            "[({x}, {y}, {z}, {x}), ({y}, {z}, {x}, {y}), ({z}, {x}, {y}, {z}), ({z}, {y}, {x}, {z}), ({z}, {z}, {y}, {x})]"
        )
    );
}
