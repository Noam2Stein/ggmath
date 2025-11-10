use core::{
    fmt::{Debug, Display},
    hash::Hash,
    mem::{MaybeUninit, transmute},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Rem, Shl, Shr, Sub},
};

mod constructor;
mod deref;
mod dir;
mod ops;
mod primitive_api;
mod primitive_impls;
mod swizzle;
pub use constructor::*;
pub use dir::*;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Vector<const N: usize, T: Scalar>(pub VectorRepr<N, T>)
where
    VecLen<N>: SupportedVecLen;

type VectorRepr<const N: usize, T> = <VecLen<N> as SupportedVecLen>::Pick<
    <T as ScalarBackend<2>>::VectorRepr,
    <T as ScalarBackend<3>>::VectorRepr,
    <T as ScalarBackend<4>>::VectorRepr,
>;

impl<const N: usize, T: Scalar> Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    pub const fn from_array(array: [T; N]) -> Self {
        const {
            assert!(size_of::<Vector<N, T>>() >= size_of::<[T; N]>());
            assert!(align_of::<Vector<N, T>>() >= align_of::<[T; N]>());
            assert!(size_of::<Vector<N, T>>() % size_of::<T>() == 0);
        }

        let mut result = MaybeUninit::<Vector<N, T>>::zeroed();

        // This is safe because `SoundVectorRepr` requires that `*Self` can be
        // looked at as `*[T; N]`.
        unsafe {
            *result.as_mut_ptr().cast::<[T; N]>() = array;
        }

        // This is safe because the first `N` elements of `result` are initialized
        // and the padding elements are zeroed.
        unsafe { result.assume_init() }
    }

    #[inline(always)]
    pub const fn splat(value: T) -> Self {
        const {
            assert!(size_of::<Vector<N, T>>() >= size_of::<[T; N]>());
            assert!(align_of::<Vector<N, T>>() >= align_of::<[T; N]>());
            assert!(size_of::<Vector<N, T>>() % size_of::<T>() == 0);
        }

        let mut result = MaybeUninit::<Vector<N, T>>::uninit();

        let mut i = 0;
        while i < size_of::<Vector<N, T>>() / size_of::<T>() {
            // This is safe because a `SoundVectorRepr` is entirely made out of
            // `T`s.
            unsafe {
                *result.as_mut_ptr().cast::<T>().add(i) = value;
            }

            i += 1;
        }

        // This is safe because the first `N` elements of `result` are initialized
        // and the padding elements are zeroed.
        unsafe { result.assume_init() }
    }

    #[inline(always)]
    pub fn from_fn(f: impl FnMut(usize) -> T) -> Self {
        Vector::from_array(core::array::from_fn(f))
    }

    #[inline(always)]
    pub const fn len(self) -> usize {
        N
    }

    #[inline(always)]
    pub const fn to_array(self) -> [T; N] {
        *self.as_array_ref()
    }

    #[inline(always)]
    pub const fn as_array_ref(&self) -> &[T; N] {
        const {
            assert!(size_of::<Vector<N, T>>() >= size_of::<[T; N]>());
            assert!(align_of::<Vector<N, T>>() >= align_of::<[T; N]>());
            assert!(size_of::<Vector<N, T>>() % size_of::<T>() == 0);
        }

        // This is safe because `SoundVectorRepr` requires that `*Self` can be
        // looked at as `*[T; N]`.
        unsafe { &*(self as *const Self as *const [T; N]) }
    }

    #[inline(always)]
    pub const fn as_mut_array(&mut self) -> &mut [T; N] {
        const {
            assert!(size_of::<Vector<N, T>>() >= size_of::<[T; N]>());
            assert!(align_of::<Vector<N, T>>() >= align_of::<[T; N]>());
            assert!(size_of::<Vector<N, T>>() % size_of::<T>() == 0);
        }

        // This is safe because `SoundVectorRepr` requires that `*Self` can be
        // looked at as `*[T; N]`.
        unsafe { &mut *(self as *mut Self as *mut [T; N]) }
    }

    #[inline(always)]
    pub fn iter(self) -> core::array::IntoIter<T, N> {
        self.to_array().into_iter()
    }

    #[inline(always)]
    pub fn iter_ref(&self) -> core::slice::Iter<'_, T> {
        self.as_array_ref().iter()
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_mut_array().iter_mut()
    }

    #[inline(always)]
    pub fn map<U: Scalar>(self, f: impl FnMut(T) -> U) -> Vector<N, U> {
        Vector::from_array(self.to_array().map(f))
    }

    #[inline(always)]
    pub const fn get(self, index: usize) -> Option<T> {
        if index < N {
            Some(self.as_array_ref()[index])
        } else {
            None
        }
    }

    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < N {
            Some(&mut self.as_mut_array()[index])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn swizzle2<const X: usize, const Y: usize>(self) -> Vector<2, T> {
        specialize!(<T as ScalarBackend<N>>::vec_swizzle2::<X, Y>(self))
    }

    #[inline(always)]
    pub fn swizzle3<const X: usize, const Y: usize, const Z: usize>(self) -> Vector<3, T> {
        specialize!(<T as ScalarBackend<N>>::vec_swizzle3::<X, Y, Z>(self))
    }

    #[inline(always)]
    pub fn swizzle4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        self,
    ) -> Vector<4, T> {
        specialize!(<T as ScalarBackend<N>>::vec_swizzle4::<X, Y, Z, W>(self))
    }

    #[inline(always)]
    pub fn reverse(self) -> Self {
        (const {
            // This is safe because the trasmution is from a type to itself.
            unsafe {
                match N {
                    2 => transmute::<
                        fn(Vector<N, T>) -> Vector<2, T>,
                        fn(Vector<N, T>) -> Vector<N, T>,
                    >(Self::swizzle2::<1, 0>),

                    3 => transmute::<
                        fn(Vector<N, T>) -> Vector<3, T>,
                        fn(Vector<N, T>) -> Vector<N, T>,
                    >(Self::swizzle3::<2, 1, 0>),

                    4 => transmute::<
                        fn(Vector<N, T>) -> Vector<4, T>,
                        fn(Vector<N, T>) -> Vector<N, T>,
                    >(Self::swizzle4::<3, 2, 1, 0>),

                    _ => panic!("unsupported vector length"),
                }
            }
        })(self)
    }
}

impl<T: Scalar> Vector<2, T> {
    #[inline(always)]
    pub fn with_x(self, value: T) -> Self {
        let mut result = self;
        result.x = value;

        result
    }

    #[inline(always)]
    pub fn with_y(self, value: T) -> Self {
        let mut result = self;
        result.y = value;

        result
    }
}

impl<T: Scalar> Vector<3, T> {
    #[inline(always)]
    pub fn with_x(self, value: T) -> Self {
        let mut result = self;
        result.x = value;

        result
    }

    #[inline(always)]
    pub fn with_y(self, value: T) -> Self {
        let mut result = self;
        result.y = value;

        result
    }

    #[inline(always)]
    pub fn with_z(self, value: T) -> Self {
        let mut result = self;
        result.z = value;

        result
    }
}

impl<T: Scalar> Vector<4, T> {
    #[inline(always)]
    pub fn with_x(self, value: T) -> Self {
        let mut result = self;
        result.x = value;

        result
    }

    #[inline(always)]
    pub fn with_y(self, value: T) -> Self {
        let mut result = self;
        result.y = value;

        result
    }

    #[inline(always)]
    pub fn with_z(self, value: T) -> Self {
        let mut result = self;
        result.z = value;

        result
    }

    #[inline(always)]
    pub fn with_w(self, value: T) -> Self {
        let mut result = self;
        result.w = value;

        result
    }
}

impl<const N: usize, T: Scalar> From<[T; N]> for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn from(array: [T; N]) -> Self {
        Vector::from_array(array)
    }
}

impl<const N: usize, T: Scalar> From<Vector<N, T>> for [T; N]
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn from(vector: Vector<N, T>) -> Self {
        vector.to_array()
    }
}

impl<const N: usize, T: Scalar> Index<usize> for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_array_ref()[index]
    }
}

impl<const N: usize, T: Scalar> IndexMut<usize> for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_array()[index]
    }
}

impl<const N: usize, T: Scalar> IntoIterator for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    type Item = T;
    type IntoIter = core::array::IntoIter<T, N>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.to_array().into_iter()
    }
}

impl<'a, const N: usize, T: Scalar> IntoIterator for &'a Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_array_ref().iter()
    }
}

impl<'a, const N: usize, T: Scalar> IntoIterator for &'a mut Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_array().iter_mut()
    }
}

impl<const N: usize, T: Scalar + Debug> Debug for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;

        for i in 0..N {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", self[i])?;
        }

        write!(f, ")")?;

        Ok(())
    }
}

impl<const N: usize, T: Scalar + Display> Display for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;

        for i in 0..N {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self[i])?;
        }

        write!(f, ")")?;

        Ok(())
    }
}

impl<const N: usize, T: Scalar + Hash> Hash for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_array_ref().hash(state);
    }
}

impl<const N: usize, T: Scalar + Default> Default for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn default() -> Self {
        Self::splat(T::default())
    }
}

impl<const N: usize, T: Scalar + PartialEq> PartialEq for Vector<N, T>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        specialize!(<T as ScalarBackend<N>>::vec_eq(*self, *other))
    }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        specialize!(<T as ScalarBackend<N>>::vec_ne(*self, *other))
    }
}

impl<const N: usize, T: Scalar + Eq> Eq for Vector<N, T> where VecLen<N>: SupportedVecLen {}

////////////////////////////////////////////////////////////////////////////////
// Scalar
////////////////////////////////////////////////////////////////////////////////

pub trait Scalar:
    Send + Sync + Copy + 'static + ScalarBackend<2> + ScalarBackend<3> + ScalarBackend<4>
{
}

pub trait ScalarBackend<const N: usize>: Send + Sync + Copy + 'static
where
    VecLen<N>: SupportedVecLen,
{
    #[expect(private_bounds)]
    type VectorRepr: SoundVectorRepr<N, Self>;

    #[inline(always)]
    fn vec_swizzle2<const X: usize, const Y: usize>(vec: Vector<N, Self>) -> Vector<2, Self>
    where
        Self: Scalar,
    {
        vec2!(vec[X], vec[Y])
    }

    #[inline(always)]
    fn vec_swizzle3<const X: usize, const Y: usize, const Z: usize>(
        vec: Vector<N, Self>,
    ) -> Vector<3, Self>
    where
        Self: Scalar,
    {
        vec3!(vec[X], vec[Y], vec[Z])
    }

    #[inline(always)]
    fn vec_swizzle4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        vec: Vector<N, Self>,
    ) -> Vector<4, Self>
    where
        Self: Scalar,
    {
        vec4!(vec[X], vec[Y], vec[Z], vec[W])
    }

    #[inline(always)]
    fn vec_eq(vec: Vector<N, Self>, other: Vector<N, Self>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        (0..N).all(|i| vec[i] == other[i])
    }

    #[inline(always)]
    fn vec_ne(vec: Vector<N, Self>, other: Vector<N, Self>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        (0..N).any(|i| vec[i] != other[i])
    }

    #[inline(always)]
    fn vec_neg(vec: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + Neg<Output = Self>,
    {
        vec.map(|x| -x)
    }

    #[inline(always)]
    fn vec_not(vec: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + Not<Output = Self>,
    {
        vec.map(|x| !x)
    }

    #[inline(always)]
    fn vec_add(vec: Vector<N, Self>, other: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + Add<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] + other[i])
    }

    #[inline(always)]
    fn vec_sub(vec: Vector<N, Self>, other: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + Sub<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] - other[i])
    }

    #[inline(always)]
    fn vec_mul(vec: Vector<N, Self>, other: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + Mul<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] * other[i])
    }

    #[inline(always)]
    fn vec_div(vec: Vector<N, Self>, other: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + Div<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] / other[i])
    }

    #[inline(always)]
    fn vec_rem(vec: Vector<N, Self>, other: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + Rem<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] % other[i])
    }

    #[inline(always)]
    fn vec_shl(vec: Vector<N, Self>, other: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + Shl<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] << other[i])
    }

    #[inline(always)]
    fn vec_shr(vec: Vector<N, Self>, other: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + Shr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] >> other[i])
    }

    #[inline(always)]
    fn vec_bitand(vec: Vector<N, Self>, other: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + BitAnd<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] & other[i])
    }

    #[inline(always)]
    fn vec_bitor(vec: Vector<N, Self>, other: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + BitOr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] | other[i])
    }

    #[inline(always)]
    fn vec_bitxor(vec: Vector<N, Self>, other: Vector<N, Self>) -> Vector<N, Self>
    where
        Self: Scalar + BitXor<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] ^ other[i])
    }
}

pub unsafe trait ScalarWrapper<T: Scalar>: Scalar {}

// SAFETY: this trait has these safety requirements:
//
// - A `*Self` pointer can be looked at as a `*[T; N]` pointer.
// - For `N = 2`, `size_of::<Self>()` is equal to `size_of::<[T; 2]>()`.
// - Padding bytes accept all bit-patterns.
// - `size_of::<Self>()` is divisible by `size_of::<T>()`.
unsafe trait SoundVectorRepr<const N: usize, T>: Send + Sync + Copy + 'static {}

// SAFETY: `[T; N]` satisfies the safety requirements of `SoundVectorRepr`:
//
// - it can be looked at as a `*[T; N]` pointer because it is a `[T; N]`.
// - For `N = 2`, its size is equal to `size_of::<[T; 2]>()` because it is
//   a `[T; 2]`.
// - Padding bytes accept all bit-patterns because it doesn't have any.
// - `size_of::<[T; N]>()` is divisible by `size_of::<T>()` because it is
//   equal to `size_of::<T>() * N`.
unsafe impl<const N: usize, T: Scalar> SoundVectorRepr<N, T> for [T; N] {}

// SAFETY: `Vector<N, TInner>` satisfies the safety requirements of
// `SoundVectorRepr`:
//
// - It can be looked at as a `[T; N]` pointer because `TInner` can be looked
//   at as a `T`.
// - For `N = 2`, its size is equal to `size_of::<[T; 2]>()` because it is equal
//   to `size_of::<[TInner; 2]>()` and `TInner` has the same size as `T`.
// - Padding bytes accept all bit-patterns because its padding bytes are part of
//   a `SoundVectorRepr` themselves.
// - `size_of::<Vector<N, TInner>>()` is divisible by `size_of::<T>()` because
//   it is equal to `size_of::<TInner>() * N` which is equal to
//   `size_of::<T>() * N`.
unsafe impl<const N: usize, T: ScalarWrapper<TInner>, TInner: Scalar> SoundVectorRepr<N, T>
    for Vector<N, TInner>
where
    VecLen<N>: SupportedVecLen,
{
}

macro_rules! specialize {
    (<$T:ty as $Backend:ident<$N:tt>>::$f:ident$(::<$($ARG:ty),* $(,)?>)?($($arg:expr),* $(,)?)) => {
        (const {
            match $N {
                2 => specialize!(@fnptr <$T as $Backend<2>>::$f$(::<$($ARG),*>)?($($arg),*)),
                3 => specialize!(@fnptr <$T as $Backend<3>>::$f$(::<$($ARG),*>)?($($arg),*)),
                4 => specialize!(@fnptr <$T as $Backend<4>>::$f$(::<$($ARG),*>)?($($arg),*)),
                ..2 | 5.. => panic!("unsupported vector length"),
            }
        })($($arg),*)
    };

    (@fnptr <$T:ty as $Backend:ident<$N:tt>>::$f:ident$(::<$($ARG:ty),* $(,)?>)?($($arg:expr),* $(,)?)) => {
        unsafe {
            let fnptr: fn($(specialize!(@_ $arg)),*) -> _
                = <$T as $Backend<$N>>::$f$(::<$($ARG),*>)?;

            core::mem::transmute_copy::<
                fn($(specialize!(@_ $arg)),*) -> _,
                fn($(specialize!(@_ $arg)),*) -> _,
            >(&fnptr)
        }
    };

    { @_ $_:tt } => { _ }
}

use specialize;

////////////////////////////////////////////////////////////////////////////////
// Length
////////////////////////////////////////////////////////////////////////////////

pub struct VecLen<const N: usize>;

pub trait SupportedVecLen {
    type Pick<
        For2: Send + Sync + Copy + 'static,
        For3: Send + Sync + Copy + 'static,
        For4: Send + Sync + Copy + 'static,
    >: Send + Sync + Copy + 'static;
}

impl SupportedVecLen for VecLen<2> {
    type Pick<
        For2: Send + Sync + Copy + 'static,
        For3: Send + Sync + Copy + 'static,
        For4: Send + Sync + Copy + 'static,
    > = For2;
}

impl SupportedVecLen for VecLen<3> {
    type Pick<
        For2: Send + Sync + Copy + 'static,
        For3: Send + Sync + Copy + 'static,
        For4: Send + Sync + Copy + 'static,
    > = For3;
}

impl SupportedVecLen for VecLen<4> {
    type Pick<
        For2: Send + Sync + Copy + 'static,
        For3: Send + Sync + Copy + 'static,
        For4: Send + Sync + Copy + 'static,
    > = For4;
}
