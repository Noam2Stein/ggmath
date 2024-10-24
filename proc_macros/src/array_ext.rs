use std::mem::MaybeUninit;

pub trait ArrayExt<T, const N: usize> {
    fn try_map<F, U, E>(self, f: F) -> Result<[U; N], E>
    where
        F: FnMut(T) -> Result<U, E>;

    fn zip<U>(self, rhs: [U; N]) -> [(T, U); N];
}

impl<T, const N: usize> ArrayExt<T, N> for [T; N] {
    fn try_map<F, U, E>(self, mut f: F) -> Result<[U; N], E>
    where
        F: FnMut(T) -> Result<U, E>,
    {
        let mut array: [MaybeUninit<U>; N] = [(); N].map(|_| MaybeUninit::uninit());
        let mut iterator = self.into_iter();

        for item in array.iter_mut() {
            // NOTE: it is guranteed that this will not panic
            let next = iterator.next().unwrap();
            *item = MaybeUninit::new((f)(next)?);
        }

        // SAFETY: because of the previous loops all values are initialized
        Ok(array.map(|value: MaybeUninit<U>| unsafe { value.assume_init() }))
    }

    fn zip<U>(self, rhs: [U; N]) -> [(T, U); N] {
        let mut rhs = rhs.into_iter();
        self.map(|item| (item, rhs.next().unwrap()))
    }
}
