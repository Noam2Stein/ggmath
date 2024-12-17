#[allow(unused_imports)]
use std::{
    fmt::Debug,
    mem::forget,
    panic::{catch_unwind, set_hook, take_hook},
};
use std::{
    fmt::{self, Formatter},
    panic::UnwindSafe,
};

use ggmath::vector::*;

use crate::{ScalarTestingError, TestableScalar};

pub fn test_scalar<T: TestableScalar>() -> Result<(), ScalarTestingError> {
    set_hook(Box::new(|_| {}));

    test_scalar_n_a::<2, T, VecAligned>()?;
    test_scalar_n_a::<2, T, VecPacked>()?;
    test_scalar_n_a::<3, T, VecAligned>()?;
    test_scalar_n_a::<3, T, VecPacked>()?;
    test_scalar_n_a::<4, T, VecAligned>()?;
    test_scalar_n_a::<4, T, VecPacked>()?;

    let _ = take_hook();

    Ok(())
}

enum MaybePanic<T: UnwindSafe> {
    Value(T),
    Panic,
}
impl<T: UnwindSafe> MaybePanic<T> {
    fn from_fn(f: impl FnOnce() -> T + UnwindSafe) -> Self {
        match catch_unwind(f) {
            Ok(ok) => Self::Value(ok),
            Err(_) => Self::Panic,
        }
    }
}
impl<T: UnwindSafe + Debug> Debug for MaybePanic<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{value:?}"),
            Self::Panic => write!(f, "panic"),
        }
    }
}
impl<T: UnwindSafe + PartialEq<Rhs>, Rhs: UnwindSafe> PartialEq<MaybePanic<Rhs>> for MaybePanic<T> {
    fn eq(&self, other: &MaybePanic<Rhs>) -> bool {
        match self {
            Self::Value(value) => match other {
                MaybePanic::Value(other) => value == other,
                MaybePanic::Panic => false,
            },
            Self::Panic => match other {
                MaybePanic::Value(_) => false,
                MaybePanic::Panic => true,
            },
        }
    }
}

fn assert_lhs_fn<
    const N: usize,
    T: TestableScalar,
    A: VecAlignment,
    V: PartialEq<E> + Debug,
    E: Debug,
>(
    vector_fn: &'static str,
    value: V,
    expected_value: E,
    value_expr: &'static str,
    expected_value_expr: &'static str,
    inputs: &[String],
) -> Result<(), ScalarTestingError>
where
    ScalarCount<N>: VecLen,
{
    if value == expected_value {
        Ok(())
    } else {
        Err(ScalarTestingError::new::<N, T, A>(
                vector_fn,
                format!("{value_expr} == {expected_value_expr} was not met\n\nexpected `{expected_value:?}`\nfound `{value:?}`\n\n{}", inputs.join(""))
            ))
    }
}

macro_rules! assert_lhs {
    ($vector_fn:ident: $value:expr, $expected_value:expr $(, $input:ident)* $(,)?) => {
        assert_lhs_fn::<N, T, A, _, _>(
            stringify!($vector_fn),
            MaybePanic::from_fn(|| $value),
            MaybePanic::from_fn(|| $expected_value),
            stringify!($value),
            stringify!($expected_value),
            &[$(format!(" * {} = {:?} \n", stringify!($input), $input)), *],
        )?;
    };
}
fn test_scalar_n_a<const N: usize, T: TestableScalar, A: VecAlignment>(
) -> Result<(), ScalarTestingError>
where
    ScalarCount<N>: VecLen,
{
    test_vector_get::<N, T, A>()?;

    Ok(())
}

fn test_vector_get<const N: usize, T: TestableScalar, A: VecAlignment>(
) -> Result<(), ScalarTestingError>
where
    ScalarCount<N>: VecLen,
{
    let vector = Vector::<N, T, A>::from_array(T::n_special_values(0));

    for i in 0..=4 {
        assert_lhs!(get:
            vector.get(i),
            vector.as_array().get(i).map(|some| *some),
        vector, i);

        for i_1 in 0..=4 {
            assert_lhs!(get_1_1:
                vector.get_1_1([i, i_1]).map_or([None; 2], |some| some.into_array().map(Some)),
                [vector.get(i), vector.get(i_1)],
            vector, i, i_1);

            for i_2 in 0..=4 {
                assert_lhs!(get_1_1_1:
                    vector.get_1_1_1([i, i_1, i_2]).map_or([None; 3], |some| some.into_array().map(Some)),
                    [vector.get(i), vector.get(i_1), vector.get(i_2)],
                vector, i, i_1, i_2);

                for i_3 in 0..=4 {
                    assert_lhs!(get_1_1_1_1:
                        vector.get_1_1_1_1([i, i_1, i_2, i_3]).map_or([None; 4], |some| some.into_array().map(Some)),
                        [vector.get(i), vector.get(i_1), vector.get(i_2), vector.get(i_3)],
                    vector, i, i_1, i_2, i_3);
                }
            }
        }
    }

    for i in 0..N {
        assert_lhs!(get_unchecked:
            unsafe { vector.get_unchecked(i) },
            vector[i],
        vector, i);

        for i_1 in 0..N {
            assert_lhs!(get_1_1_unchecked:
                unsafe { vector.get_1_1_unchecked([i, i_1]) },
                Vec2::from_array([vector[i], vector[i_1]]),
            vector, i, i_1);

            for i_2 in 0..N {
                assert_lhs!(get_1_1_1_unchecked:
                    unsafe { vector.get_1_1_1_unchecked([i, i_1, i_2]) },
                    Vec3::from_array([vector[i], vector[i_1], vector[i_2]]),
                vector, i, i_1, i_2);

                for i_3 in 0..N {
                    assert_lhs!(get_1_1_1_1:
                        vector.get_1_1_1_1([i, i_1, i_2, i_3]).map_or([None; 4], |some| some.into_array().map(Some)),
                        [vector.get(i), vector.get(i_1), vector.get(i_2), vector.get(i_3)],
                    vector, i, i_1, i_2, i_3);
                }
            }
        }
    }

    Ok(())
}
