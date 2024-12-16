use ggmath::vector::*;

use crate::testable_scalar::*;

pub fn test_scalar<T: TestableScalar>() {
    tests_for_n_t_a::<2, T, VecAligned>();
    tests_for_n_t_a::<2, T, VecPacked>();
    tests_for_n_t_a::<3, T, VecAligned>();
    tests_for_n_t_a::<3, T, VecPacked>();
    tests_for_n_t_a::<4, T, VecAligned>();
    tests_for_n_t_a::<4, T, VecPacked>();
}

fn tests_for_n_t_a<const N: usize, T: TestableScalar, A: VecAlignment>()
where
    ScalarCount<N>: VecLen,
{
    let array = T::n_special_values::<N>(0);
    let vector = Vector::<N, T, A>::from_array(array);

    assert_eq!(vector.into_array(), array);

    let indicies = [0, N / 2, N - 1, N];
    let valid_indicies = [0, N / 2, N - 1];

    for valid_index in valid_indicies {
        assert_eq!(vector[valid_index], array[valid_index]);
    }

    for index in indicies {
        assert_eq!(vector.get(index), array.get(index).map(|some| *some));
    }

    for x in valid_indicies {
        assert_eq!(unsafe { vector.get_unchecked(x) }, array[x]);
    }
    for x in valid_indicies {
        for y in valid_indicies {
            assert_eq!(
                vector.get_1_1([x, y]),
                Some(Vector::<2, T, A>::from_array([array[x], array[y]]))
            );
            assert_eq!(
                unsafe { vector.get_1_1_unchecked([x, y]) },
                Vector::<2, T, A>::from_array([array[x], array[y]])
            );
        }
    }
    for x in valid_indicies {
        for y in valid_indicies {
            for z in valid_indicies {
                assert_eq!(
                    vector.get_1_1_1([x, y, z]),
                    Some(Vector::<3, T, A>::from_array([
                        array[x], array[y], array[z]
                    ]))
                );
                assert_eq!(
                    unsafe { vector.get_1_1_1_unchecked([x, y, z]) },
                    Vector::<3, T, A>::from_array([array[x], array[y], array[z]])
                );
            }
        }
    }
    for x in valid_indicies {
        for y in valid_indicies {
            for z in valid_indicies {
                for w in valid_indicies {
                    assert_eq!(
                        vector.get_1_1_1_1([x, y, z, w]),
                        Some(Vector::<4, T, A>::from_array([
                            array[x], array[y], array[z], array[w]
                        ]))
                    );
                    assert_eq!(
                        unsafe { vector.get_1_1_1_1_unchecked([x, y, z, w]) },
                        Vector::<4, T, A>::from_array([array[x], array[y], array[z], array[w]])
                    );
                }
            }
        }
    }
}
