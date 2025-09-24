use genco::quote;

use crate::{
    constants::{
        COMPARISON_OPS, COMPARISON_OP_DOCS, COMPARISON_OP_TOKENS, COMPARISON_OP_TRAITS, COMPONENTS, COMPONENT_ORDINALS, LENGTHS, LENGTH_NAMES, PRIMITIVES
    },
    join_or,
    module::{SrcDir, TestDir, TokensExt},
};

mod constructor;
mod dir;
mod ops;
mod primitives;
mod scalar;
mod swizzle;

pub fn src_mod() -> SrcDir {
    quote! {
        $("//! Vector related types and traits")

        use core::{
            fmt::{Debug, Display},
            ops::*,
            slice::SliceIndex,
            mem::transmute,
            hash::{Hash, Hasher},
        };

        use crate::{Construct, Usize, IndexOutOfBoundsError, specialize};

        mod constructor;
        mod dir;
        mod primitives;
        mod ops;
        mod scalar;
        #[cfg(feature = "swizzle")]
        mod swizzle;
        pub use scalar::*;
        pub use constructor::*;
        pub use dir::*;

        $(let length_list = join_or(LENGTHS.iter().map(|&n| n.to_string())))

        $("/// A generic vector type.")
        $("///")
        $("/// This type is generic over 3 parameters:")
        $(format!("/// - `N`: The length of the vector, which currently can be {length_list}"))
        $("/// - `T`: The scalar type of the vector, which must implement [`Scalar`]")
        $("/// - `S`: The \"simdness\" of the vector, which must be either [`Simd`] or [`NonSimd`]")
        $("///")
        $("/// This type has very very useful type-aliases:")
        $("/// - `Vec{N}<T>` like [`Vec2<T>`] is for [`Simd`] vectors")
        $("/// - `Vec{N}S<T>` like [`Vec2S<T>`] is for [`NonSimd`] vectors (\"s\" stands for \"scalar\")")
        $("///")
        $("/// # Simdness")
        $("///")
        $("/// SIMD improves the performance of vector operations but increases the size of the vector in memory.")
        $("///")
        $("/// The `S` generic parameter controls whether or not the vector is SIMD.")
        $("/// [`Simd`] vectors are SIMD and [`NonSimd`] vectors are not.")
        $("///")
        $("/// Most of the time you should use [`Simd`] vectors (e.g. [`Vec3<T>`]),")
        $("/// and only use [`NonSimd`] vectors in memory-critical scenarios.")
        $("///")
        $("/// The exact vector layout rules are:")
        $("///")
        $("/// - [`NonSimd`] vectors are always stored as `[T; N]`.")
        $("///")
        $("/// - [`Simd`] vectors aren't actually guarenteed to be SIMD, especially if it doesn't have a performance benefit.")
        $("///")
        $("/// # Example")
        $("/// ```")
        $("/// use ggmath::*;")
        $("///")
        $("/// // This is a non memory critical scenario so we should use SIMD vectors.")
        $("/// struct PlayerState {")
        $("///     // Vector<3, f32, Simd>")
        $("///     position: Vec3<f32>,")
        $("/// }")
        $("///")
        $("/// // This is a memory critical scenario so we should use non-SIMD vectors.")
        $("/// struct GpuVertex {")
        $("///     // Vector<3, f32, NonSimd>")
        $("///     position: Vec3S<f32>,")
        $("///     // Vector<3, f32, NonSimd>")
        $("///     normal: Vec3S<f32>,")
        $("///     // Vector<2, f32, NonSimd>")
        $("///     uv: Vec2S<f32>,")
        $("/// }")
        $("///")
        $("/// fn initial_player_state() -> PlayerState {")
        $("///     PlayerState {")
        $("///         position: vec3!(0.0, 1.0, 2.0),")
        $("///     }")
        $("/// }")
        $("///")
        $("/// fn triangle_vertices() -> [GpuVertex; 3] {")
        $("///     [")
        $("///         GpuVertex {")
        $("///             position: vec3s!(-1.0, -1.0, 0.0),")
        $("///             normal: vec3s!(0.0, 0.0, 1.0),")
        $("///             uv: vec2s!(0.0, 0.0),")
        $("///         },")
        $("///         GpuVertex {")
        $("///             position: vec3s!(1.0, -1.0, 0.0),")
        $("///             normal: vec3s!(0.0, 0.0, 1.0),")
        $("///             uv: vec2s!(1.0, 0.0),")
        $("///         },")
        $("///         GpuVertex {")
        $("///             position: vec3s!(0.0, 1.0, 0.0),")
        $("///             normal: vec3s!(0.0, 0.0, 1.0),")
        $("///             uv: vec2s!(0.5, 1.0),")
        $("///         },")
        $("///     ]")
        $("/// }")
        $("/// ```")
        #[repr(transparent)]
        pub struct Vector<const N: usize, T: Scalar, S: Simdness>(pub S::InnerVector<N, T>)
        where
            Usize<N>: VecLen;

        $(
            for &n in LENGTHS =>

            $(format!("/// Type alias for [`Vector<{n}, T, Simd>`][Vector]."))
            pub type Vec$(n)<T> = Vector<$n, T, Simd>;
            $['\n']
        )
        $(
            for &n in LENGTHS =>

            $(format!("/// Type alias for [`Vector<{n}, T, NonSimd>`][Vector]."))
            $(r#"/// "S" stands for "scalar""#)
            pub type Vec$(n)S<T> = Vector<$n, T, NonSimd>;
            $['\n']
        )

        $("/// Generates vector type-aliases for a specific scalar type.")
        $("///")
        $("/// # Example")
        $("///")
        $("/// ```")
        $("/// use ggmath::*;")
        $("///")
        $("/// vector_aliases!(pub type F => f32);")
        $("/// ```")
        $("/// Generates:")
        $("/// ```")
        $("/// use ggmath::*;")
        $("///")
        $(for &n in LENGTHS => $(format!("/// pub type FVec{n} = Vec{n}<f32>;"))$['\r'])
        $(for &n in LENGTHS => $(format!("/// pub type FVec{n}S = Vec{n}S<f32>;"))$['\r'])
        $("/// ```")
        #[macro_export]
        macro_rules! vector_aliases {
            ($$(#[$$($$attr:tt)*])* pub($$($$vis:tt)*) type $$prefix:ident => $$t:ty) => {
                $$crate::vector_aliases!(@(pub type $$($$vis)*) $$(#[$$($$attr)*])* $$prefix => $$t);
            };
            ($$(#[$$($$attr:tt)*])* pub type $$prefix:ident => $$t:ty) => {
                $$crate::vector_aliases!(@(pub) $$(#[$$($$attr)*])* type $$prefix => $$t);
            };
            ($$(#[$$($$attr:tt)*])* type $$prefix:ident => $$t:ty) => {
                $$crate::vector_aliases!(@() $$(#[$$($$attr)*])* type $$prefix => $$t);
            };

            (@($$($$vis:tt)*) $$(#[$$($$attr:tt)*])* type $$prefix:ident => $$t:ty) => {
                $$crate::_hidden_::paste! {
                    $(
                        for &n in LENGTHS =>

                        #[doc = $(format!("\"Type alias to [`Vector<{n}, \"")) $$t ", Simd>`][Vector]."]
                        $$(#[$$($$attr)*])*
                        $$($$vis)* type [<$$prefix Vec$(n)>] = $$crate::Vec$(n)<$$t>;

                        #[doc = $(format!("\"Type alias to [`Vector<{n}, \"")) $$t ", NonSimd>`][Vector]."]
                        #[doc = r#""S" stands for "scalar""#]
                        $$(#[$$($$attr)*])*
                        $$($$vis)* type [<$$prefix Vec$(n)S>] = $$crate::Vec$(n)S<$$t>;

                        $['\n']
                    )
                }
            };
        }

        $("/// Marks a `Usize<N>` type as a valid vector length.")
        pub trait VecLen {
            $("/// The inner type contained inside `Vector<N, T, Simd>`.")
            type InnerSimdVector<T: Scalar>: Construct;

            $("/// The length value as an enum.")
            const ENUM: VecLenEnum;
        }

        $("/// An enum with all currently supported vector lengths.")
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum VecLenEnum {
            $(
                for (&n, &length_name) in LENGTHS.iter().zip(LENGTH_NAMES.iter()) =>

                $(format!("/// `{n}`"))
                $length_name,
                $['\r']
            )
        }

        $("/// See [`Vector`] for information.")
        pub trait Simdness: 'static {
            $("/// The inner type contained inside [`Vector`].")
            type InnerVector<const N: usize, T: Scalar>: Construct
            where
                Usize<N>: VecLen;

            $("/// Whether or not the vector is SIMD.")
            $("/// This does not guarentee an actual SIMD layout,")
            $("/// it only means that the layout is optimized for performance, not for size.")
            const IS_SIMD: bool;
        }

        $("/// See [`Vector`] for information.")
        pub struct Simd;

        $("/// See [`Vector`] for information.")
        pub struct NonSimd;

        $(
            for (&n, &length_name) in LENGTHS.iter().zip(LENGTH_NAMES.iter()) join($['\n'])=>

            impl VecLen for Usize<$n> {
                type InnerSimdVector<T: Scalar> = T::InnerSimdVec$(n);

                const ENUM: VecLenEnum = VecLenEnum::$length_name;
            }
        )

        impl Simdness for Simd {
            type InnerVector<const N: usize, T: Scalar>
                = <Usize<N> as VecLen>::InnerSimdVector<T>
            where
                Usize<N>: VecLen;

            const IS_SIMD: bool = true;
        }

        impl Simdness for NonSimd {
            type InnerVector<const N: usize, T: Scalar>
                = [T; N]
            where
                Usize<N>: VecLen;
                
            const IS_SIMD: bool = false;
        }

        impl<const N: usize, T: Scalar, S: Simdness> Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            $("/// Returns true if the vector is `S = Simd`.")
            $("/// The output only depends on `S` and does not rely on runtime values.")
            #[inline(always)]
            pub const fn is_simd(self) -> bool {
                S::IS_SIMD
            }

            $("/// Converts the vector to a `S = Simd` vector.")
            #[inline(always)]
            pub fn as_simd(self) -> Vector<N, T, Simd> {
                self.as_storage()
            }

            $("/// Converts the vector to a `S = NonSimd` vector.")
            #[inline(always)]
            pub fn as_non_simd(self) -> Vector<N, T, NonSimd> {
                self.as_storage()
            }

            $("/// Converts the vector to the specified simdness.")
            #[inline(always)]
            pub fn as_storage<S2: Simdness>(self) -> Vector<N, T, S2> {
                specialize! {
                    (self: Vector<N, T, S>) -> Vector<N, T, S2>:

                    for (Vector<N, T, S>) -> Vector<N, T, S> {
                        |vec| vec
                    }
                    else {
                        Vector::from_array(self.as_array())
                    }
                }
            }

            $("/// Creates a new vector from an array.")
            #[inline(always)]
            pub fn from_array(array: [T; N]) -> Self {
                specialize! {
                    (array: [T; N]) -> Vector<N, T, S>:

                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for ([T; $n]) -> Vector<$n, T, Simd> {
                            |array| T::vec$(n)_from_array(array)
                        }
                    )
                    for ([T; N]) -> Vector<N, T, NonSimd> {
                        |array| Vector(array)
                    }
                    else {
                        unreachable!("unusual vector type")
                    }
                }
            }

            $("/// Creates a new vector where each component is the same value.")
            #[inline(always)]
            pub fn splat(value: T) -> Self {
                specialize! {
                    (value: T) -> Vector<N, T, S>:
   
                    $(
                        for &n in LENGTHS join($['\r']) =>
    
                        for (T) -> Vector<$n, T, Simd> {
                            |value| T::vec$(n)_splat(value)
                        }
                    )
                    else {
                        Vector::from_array([value; N])
                    }
                }
            }

            $("/// Creates a new vector where each component is evaluated from the given function called with the component index.")
            $("/// The function is called in order.")
            #[inline(always)]
            pub fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self {
                Vector::from_array(core::array::from_fn(f))
            }

            $("/// Returns the number of components in the vector.")
            #[inline(always)]
            pub const fn len(self) -> usize {
                N
            }

            $("/// Converts the vector to an array.")
            #[inline(always)]
            pub fn as_array(self) -> [T; N] {
                specialize! {
                    (self: Vector<N, T, S>) -> [T; N]:

                    $(
                        for &n in LENGTHS join($['\r']) =>
    
                        for (Vector<$n, T, Simd>) -> [T; $n] {
                            |vec| T::vec$(n)_as_array(vec)
                        }
                    )
                    for (Vector<N, T, NonSimd>) -> [T; N] {
                        |vec| vec.0
                    }
                    else {
                        unreachable!("unusual vector type")
                    }
                }
            }

            $("/// Returns the component at the given index or panics if the index is out of bounds.")
            #[inline(always)]
            pub fn index(self, index: usize) -> T {
                if index >= N {
                    panic!("index out of bounds: the len is {N} but the index is {index}");
                }

                unsafe { self.get_unchecked(index) }
            }

            $("/// Returns the component at the given index or returns None if the index is out of bounds.")
            #[inline(always)]
            pub fn get(self, index: usize) -> Option<T> {
                if index >= N {
                    None
                } else {
                    Some(unsafe { self.get_unchecked(index) })
                }
            }

            $("/// Returns the component at the given index with no bounds checking.")
            $("///")
            $("/// # Safety")
            $("/// The caller must ensure that the index is in bounds.")
            #[inline(always)]
            pub unsafe fn get_unchecked(self, index: usize) -> T {
                specialize! {
                    (self: Vector<N, T, S>, index: usize) -> T:

                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for (Vector<$n, T, Simd>, usize) -> T {
                            |vec, index| unsafe { T::vec$(n)_get_unchecked(vec, index) }
                        }
                    )
                    else {
                        unsafe { *self.as_array().get_unchecked(index) }
                    }
                }
            }

            $("/// Sets the component at the given index or panics if the index is out of bounds.")
            #[inline(always)]
            pub fn set(&mut self, index: usize, value: T) {
                if index >= N {
                    panic!("index out of bounds: the len is {N} but the index is {index}");
                }

                unsafe { self.set_unchecked(index, value) }
            }

            $("/// Sets the component at the given index or returns an error if the index is out of bounds.")
            #[inline(always)]
            pub fn try_set(&mut self, index: usize, value: T) -> Result<(), IndexOutOfBoundsError> {
                if index >= N {
                    Err(IndexOutOfBoundsError)
                } else {
                    unsafe { self.set_unchecked(index, value) }

                    Ok(())
                }
            }

            $("/// Sets the component at the given index with no bounds checking.")
            $("///")
            $("/// # Safety")
            $("/// The caller must ensure that the index is in bounds.")
            #[inline(always)]
            pub unsafe fn set_unchecked(&mut self, index: usize, value: T) {
                *self = specialize! {
                    ((*self): Vector<N, T, S>, index: usize, value: T) -> Vector<N, T, S>:
                    
                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for (Vector<$n, T, Simd>, usize, T) -> Vector<$n, T, Simd> {
                            |vec, index, value| unsafe { T::vec$(n)_with_unchecked(vec, index, value) }
                        }
                    )
                    else {
                        let mut array = self.as_array();
                        unsafe {
                            *array.get_unchecked_mut(index) = value;
                        }
                        Vector::from_array(array)
                    }
                };
            }

            $(
                for &n2 in LENGTHS join($['\n']) =>

                $(format!("/// Returns a vec{n2} where:"))
                $(
                    for i in 0..n2 =>

                    $(format!("/// - The `{}` ({}) component is `self[{}_SRC]`", COMPONENTS[i], COMPONENT_ORDINALS[i], COMPONENTS[i].to_uppercase()))
                    $['\r']
                )
                $("///")
                $("/// Out of bounds indices are compile time errors.")
                #[inline(always)]
                pub fn shuffle_$(n2)<$(for i in 0..n2 join(, ) => const $(COMPONENTS[i].to_uppercase())_SRC: usize)>(self) -> Vector<$n2, T, S> {
                    specialize! {
                        (self: Vector<N, T, S>) -> Vector<$n2, T, S>:

                        $(
                            for &n in LENGTHS join($['\r']) =>

                            for (Vector<$n, T, Simd>) -> Vector<$n2, T, Simd> {
                                |vec| T::vec$(n)_shuffle_$(n2)::<$(for i in 0..n2 join(, ) => $(COMPONENTS[i].to_uppercase())_SRC)>(vec)
                            }
                        )
                        else {
                            Vector::<$n2, T, S>::from_array([$(for i in 0..n2 join(, ) => self.index($(COMPONENTS[i].to_uppercase())_SRC))])
                        }
                    }
                }
            )

            $("/// Maps each component of the vector to a new value using the given function.")
            #[inline(always)]
            pub fn map<T2: Scalar, F: Fn(T) -> T2>(self, f: F) -> Vector<N, T2, S>
            where
                Usize<N>: VecLen,
            {
                Vector::from_array(self.as_array().map(f))
            }

            $("/// Returns an iterator over the components of the vector.")
            #[inline(always)]
            pub fn iter(self) -> <[T; N] as IntoIterator>::IntoIter {
                self.into_iter()
            }

            $("/// Folds the vector into a single value by repeatedly applying the given function to an accumulator on the vector's elements,")
            $("/// using the given initial value.")
            #[inline(always)]
            pub fn fold(self, init: T, mut f: impl FnMut(T, T) -> T) -> T {
                let mut output = init;

                for component in self {
                    output = f(output, component);
                }

                output
            }

            $("/// Reduces the vector into a single value by repeatedly applying the given function to an accumulator on the vector's elements,")
            $("/// using the first component as the initial value.")
            #[inline(always)]
            pub fn reduce(self, mut f: impl FnMut(T, T) -> T) -> T {
                let mut output = self.index(0);

                for component in self.iter().skip(1) {
                    output = f(output, component);
                }

                output
            }

            $("/// Returns true if all components of the vector satisfy the given predicate.")
            $("/// If a component does not satisfy the predicate,")
            $("/// the function returns false immediately without evaluating the remaining components.")
            #[inline(always)]
            pub fn all(self, f: impl FnMut(T) -> bool) -> bool {
                self.iter().all(f)
            }

            $("/// Returns true if any component of the vector satisfies the given predicate.")
            $("/// If a component satisfies the predicate,")
            $("/// the function returns true immediately without evaluating the remaining components.")
            #[inline(always)]
            pub fn any(self, f: impl FnMut(T) -> bool) -> bool {
                self.iter().any(f)
            }

            $("/// Returns the number of components that satisfy the given predicate.")
            #[inline(always)]
            pub fn count(self, mut f: impl FnMut(T) -> bool) -> usize {
                self.iter().filter(|x| f(*x)).count()
            }

            $(
                for cmp_op_idx in 0..COMPARISON_OPS.len() join($['\n']) =>

                $(let cmp_lower = COMPARISON_OPS[cmp_op_idx])
                $(let cmp_tt = COMPARISON_OP_TOKENS[cmp_op_idx])
                $(let cmp_trait = COMPARISON_OP_TRAITS[cmp_op_idx])
                $(let cmp_doc = COMPARISON_OP_DOCS[cmp_op_idx])

                $(format!("/// Returns a vector of booleans where each component is `true` if the corresponding component of `self` is {cmp_doc} the corresponding component of `other`."))
                #[inline(always)]
                pub fn $(cmp_lower)_mask<T2: Scalar>(self, other: Vector<N, T2, S>) -> Vector<N, bool, S>
                where
                    T: $cmp_trait<T2>,
                {
                    specialize! {
                        (self: Vector<N, T, S>, other: Vector<N, T2, S>) -> Vector<N, bool, S>:
    
                        $(
                            for &n in LENGTHS join($['\r']) =>
    
                            for (Vector<$n, T, Simd>, Vector<$n, T2, Simd>) -> Vector<$n, bool, Simd> {
                                |vec, other| T::vec$(n)_$(cmp_lower)_mask(vec, other)
                            }
                        )
                        else {
                            Vector::from_fn(|i| self.index(i) $cmp_tt other.index(i))
                        }
                    }
                }
            )

            $("/// Returns the sum of the components of the vector.")
            #[inline(always)]
            pub fn sum(self) -> T
            where
                T: Add<Output = T>,
            {
                specialize! {
                    (self: Vector<N, T, S>) -> T:

                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for (Vector<$n, T, Simd>) -> T {
                            |vec| T::vec$(n)_sum(vec)
                        }
                    )
                    else {
                        self.reduce(|a, b| a + b)
                    }
                }
            }

            $("/// Returns the product of the components of the vector.")
            #[inline(always)]
            pub fn product(self) -> T
            where
                T: Mul<Output = T>,
            {
                specialize! {
                    (self: Vector<N, T, S>) -> T:

                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for (Vector<$n, T, Simd>) -> T {
                            |vec| T::vec$(n)_product(vec)
                        }
                    )
                    else {
                        self.reduce(|a, b| a * b)
                    }
                }
            }

            $("/// Returns the square of the magnitude of the vector.")
            #[inline(always)]
            pub fn mag_sq(self) -> T
            where
                T: Add<Output = T> + Mul<Output = T>,
            {
                (self * self).sum()
            }

            $("/// Returns the dot product of `self` and `other`.")
            #[inline(always)]
            pub fn dot(self, other: Self) -> T
            where
                T: Add<Output = T> + Mul<Output = T>,
            {
                (self * other).sum()
            }
        }

        impl<const N: usize, T: Scalar> Vector<N, T, NonSimd>
        where
            Usize<N>: VecLen,
        {
            $("/// Converts an array reference to a vector reference.")
            #[inline(always)]
            pub const fn from_array_ref(array: &[T; N]) -> &Self {
                unsafe { transmute::<&[T; N], &Vector<N, T, NonSimd>>(array) }
            }

            $("/// Converts a mutable array reference to a mutable vector reference.")
            #[inline(always)]
            pub const fn from_mut_array(array: &mut [T; N]) -> &mut Self {
                unsafe { transmute::<&mut [T; N], &mut Vector<N, T, NonSimd>>(array) }
            }

            $("/// Converts a vector reference to an array reference.")
            #[inline(always)]
            pub const fn as_array_ref(&self) -> &[T; N] {
                &self.0
            }

            $("/// Converts a mutable vector reference to a mutable array reference.")
            #[inline(always)]
            pub const fn as_mut_array(&mut self) -> &mut [T; N] {
                &mut self.0
            }

            $("/// Returns a pointer to the first element of the vector.")
            #[inline(always)]
            pub const fn as_ptr(&self) -> *const T {
                self.0.as_ptr()
            }

            $("/// Returns a mutable pointer to the first element of the vector.")
            #[inline(always)]
            pub const fn as_mut_ptr(&mut self) -> *mut T {
                self.0.as_mut_ptr()
            }

            $("/// Returns an iterator over the references to the components of the vector.")
            #[inline(always)]
            pub fn iter_ref(&self) -> <&[T; N] as IntoIterator>::IntoIter {
                self.into_iter()
            }

            $("/// Returns an iterator over the mutable references to the components of the vector.")
            #[inline(always)]
            pub fn iter_mut(&mut self) -> <&mut [T; N] as IntoIterator>::IntoIter {
                self.into_iter()
            }
        }

        $(
            for &n in LENGTHS join($['\n']) =>

            impl<T: Scalar, S: Simdness> Vector<$n, T, S> {
                $(
                    if n == 2 =>

                    $(format!("/// Returns `self` rotated 90 degrees counter-clockwise."))
                    #[inline(always)]
                    pub fn perp(self) -> Self
                    where
                        T: Neg<Output = T>,
                    {
                        vec2g!(-self.y(), self.x())
                    }

                    $(format!("/// Returns `self` rotated 90 degrees clockwise."))
                    #[inline(always)]
                    pub fn perp_cw(self) -> Self
                    where
                        T: Neg<Output = T>,
                    {
                        vec2g!(self.y(), -self.x())
                    }

                    $("/// Returns the perpendicular dot product of `self` and `other`.")
                    #[inline(always)]
                    pub fn perp_dot(self, other: Self) -> T
                    where
                        T: Mul<Output = T> + Sub<Output = T>,
                    {
                        self.x() * other.y() - self.y() * other.x()
                    }
                )

                $(
                    if n == 3 =>

                    $(format!("/// Returns the cross product of `self` and `other`."))
                    #[inline(always)]
                    pub fn cross(self, other: Self) -> Self
                    where
                        T: Mul<Output = T> + Sub<Output = T>,
                    {
                        self.yzx() * other.zxy() - self.zxy() * other.yzx()
                    }
                )

                $(
                    for i in 0..n join($['\n']) =>

                    $(format!("/// Returns the `{}` ({}) component of `self`.", COMPONENTS[i], COMPONENT_ORDINALS[i]))
                    #[inline(always)]
                    pub fn $(COMPONENTS[i])(self) -> T {
                        self.index($i)
                    }
                )

                $(
                    for i in 0..n join($['\n']) =>

                    $(format!("/// Returns `self` but with the `{}` ({}) component set to `value`.", COMPONENTS[i], COMPONENT_ORDINALS[i]))
                    #[inline(always)]
                    pub fn with_$(COMPONENTS[i])(self, value: T) -> Self {
                        let mut output = self;
                        output.set($i, value);
                        output
                    }
                )

                $(
                    for i in 0..n join($['\n']) =>

                    $(format!("/// Sets the `{}` ({}) component of `self` to `value`.", COMPONENTS[i], COMPONENT_ORDINALS[i]))
                    #[inline(always)]
                    pub fn set_$(COMPONENTS[i])(&mut self, value: T) {
                        *self = self.with_$(COMPONENTS[i])(value);
                    }
                )

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) join($['\n']) =>
    
                    $(format!("/// Returns `self` but with:"))
                    $(
                        for i in 0..n2 =>

                        $(format!("/// - `self[{}_DST]` set to the `{}` ({}) component of `value`", COMPONENTS[i].to_uppercase(), COMPONENTS[i], COMPONENT_ORDINALS[i]))
                        $['\r']
                    )
                    $("///")
                    $("/// Out of bounds indices are compile time errors.")
                    #[inline(always)]
                    pub fn with_shuffle_$(n2)<$(for i in 0..n2 join(, ) => const $(COMPONENTS[i].to_uppercase())_DST: usize)>(self, value: Vector<$n2, T, S>) -> Self {
                        specialize! {
                            (self: Vector<$n, T, S>, value: Vector<$n2, T, _>) -> Vector<$n, T, S>:
    
                            for (Vector<$n, T, Simd>, Vector<$n2, T, Simd>) -> Vector<$n, T, Simd> {
                                |vec, value| T::vec$(n)_with_shuffle_$(n2)::<$(for i in 0..n2 join(, ) => $(COMPONENTS[i].to_uppercase())_DST)>(vec, value)
                            }
                            else {
                                let mut output = self;
                                $(
                                    for i in 0..n2 =>
    
                                    output.set($(COMPONENTS[i].to_uppercase())_DST, value.$(COMPONENTS[i])());
                                    $['\r']
                                )
    
                                output
                            }
                        }
                    }
                )
            }
        )

        $(
            for &n in LENGTHS join($['\n']) =>

            impl<T: Scalar> Vector<$n, T, NonSimd> {
                $(
                    for i in 0..n join($['\n']) =>

                    $(format!("/// Returns a reference to the `{}` ({}) component of `self`.", COMPONENTS[i], COMPONENT_ORDINALS[i]))
                    #[inline(always)]
                    pub const fn $(COMPONENTS[i])_ref(&self) -> &T {
                        &self.0[$i]
                    }
                )
                
                $(
                    for i in 0..n join($['\n']) =>

                    $(format!("/// Returns a mutable reference to the `{}` ({}) component of `self`.", COMPONENTS[i], COMPONENT_ORDINALS[i]))
                    #[inline(always)]
                    pub const fn $(COMPONENTS[i])_mut(&mut self) -> &mut T {
                        &mut self.0[$i]
                    }
                )
            }
        )

        impl<const N: usize, T: Scalar, S: Simdness> Clone for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<const N: usize, T: Scalar, S: Simdness> Copy for Vector<N, T, S> where Usize<N>: VecLen {}

        impl<const N: usize, T: Scalar, S: Simdness> IntoIterator for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            type Item = T;
            type IntoIter = <[T; N] as IntoIterator>::IntoIter;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                self.as_array().into_iter()
            }
        }

        impl<'a, const N: usize, T: Scalar> IntoIterator for &'a Vector<N, T, NonSimd>
        where
            Usize<N>: VecLen,
        {
            type Item = &'a T;
            type IntoIter = <&'a [T; N] as IntoIterator>::IntoIter;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                self.as_array_ref().into_iter()
            }
        }

        impl<'a, const N: usize, T: Scalar> IntoIterator for &'a mut Vector<N, T, NonSimd>
        where
            Usize<N>: VecLen,
        {
            type Item = &'a mut T;
            type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                self.as_mut_array().into_iter()
            }
        }

        impl<const N: usize, T: Scalar, I: SliceIndex<[T]>> Index<I> for Vector<N, T, NonSimd>
        where
            Usize<N>: VecLen,
        {
            type Output = I::Output;

            #[inline(always)]
            fn index(&self, index: I) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<const N: usize, T: Scalar, I: SliceIndex<[T]>> IndexMut<I> for Vector<N, T, NonSimd>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn index_mut(&mut self, index: I) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl<const N: usize, T: Scalar + PartialEq<T2>, S: Simdness, T2: Scalar>
            PartialEq<Vector<N, T2, S>> for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn eq(&self, other: &Vector<N, T2, S>) -> bool {
                specialize! {
                    ((*self): Vector<N, T, S>, (*other): Vector<N, T2, S>) -> bool:

                    $(
                        for &n in LENGTHS join($['\r']) =>
                        
                        for (Vector<$n, T, Simd>, Vector<$n, T2, Simd>) -> bool {
                            |vec, other| T::vec$(n)_eq(vec, other)
                        }
                    )
                    else {
                        self.iter().zip(*other).all(|(a, b)| a == b)
                    }
                }
            }

            #[inline(always)]
            fn ne(&self, other: &Vector<N, T2, S>) -> bool {
                specialize! {
                    ((*self): Vector<N, T, S>, (*other): Vector<N, T2, S>) -> bool:

                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for (Vector<$n, T, Simd>, Vector<$n, T2, Simd>) -> bool {
                            |vec, other| T::vec$(n)_ne(vec, other)
                        }
                    )
                    else {
                        self.iter().zip(*other).any(|(a, b)| a != b)
                    }
                }
            }
        }

        impl<const N: usize, T: Scalar + Eq, S: Simdness> Eq for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {}

        impl<const N: usize, T: Scalar + Hash, S: Simdness> Hash for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.as_array().hash(state)
            }
        }

        impl<const N: usize, T: Scalar + Default, S: Simdness> Default for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn default() -> Self {
                Self::splat(T::default())
            }
        }

        impl<const N: usize, T: Scalar + Debug, S: Simdness> Debug for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "(")?;

                for i in 0..N {
                    if i != 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{:?}", self.index(i))?;
                }

                write!(f, ")")?;

                Ok(())
            }
        }

        impl<const N: usize, T: Scalar + Display, S: Simdness> Display for Vector<N, T, S>
        where
            Usize<N>: VecLen,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "(")?;

                for i in 0..N {
                    if i != 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", self.index(i))?;
                }

                write!(f, ")")?;

                Ok(())
            }
        }
    }
    .to_src_dir("vector")
    .with_submod_dir(primitives::src_mod())
    .with_submod_file(constructor::src_mod())
    .with_submod_file(swizzle::src_mod())
    .with_submod_file(scalar::src_mod())
    .with_submod_file(ops::src_mod())
    .with_submod_file(dir::src_mod())
}

pub fn test_mod() -> TestDir {
    quote!{
        $(
            for &primitive in PRIMITIVES join($['\r']) => mod $primitive;
        )
    }.to_test_dir("vector").with_submod_files(primitives::test_mods())
}
