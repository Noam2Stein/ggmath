use genco::quote;

use crate::{
    constants::{
        COMPARISON_OP_DOCS, COMPARISON_OP_TOKENS, COMPARISON_OP_TRAITS, COMPARISON_OPS,
        COMPONENT_ORDINALS, COMPONENTS, LENGTH_NAMES, LENGTHS,
    },
    join_or,
    module::{ModDir, TokensExt},
};

mod constructor;
mod dir;
mod ops;
mod primitives;
mod scalar;
mod swizzle;

pub fn mod_() -> ModDir {
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
        $("/// - `A`: The \"alignment\" of the vector, which enables or disables SIMD memory-alignment")
        $("///")
        $("/// This type has very very useful type-aliases:")
        $("/// - `Vec{N}<T>` like `Vec2<T>` is for SIMD vectors")
        $("/// - `Vec{N}P<T>` like `Vec2P<T>` is for non-SIMD vectors")
        $("///")
        $("/// # Alignment")
        $("///")
        $("/// SIMD improves the performance of vector operations but increases the size of the vector in memory.")
        $("///")
        $("/// The `A` generic parameter controls whether or not the vector is SIMD-aligned.")
        $("/// `Vector<N, T, VecAligned>` vectors are SIMD-aligned and `Vector<N, T, VecPacked>` vectors are not.")
        $("///")
        $("/// Most of the time you should use `VecAligned` vectors (e.g. `Vec3<T>`),")
        $("/// and only use `VecPacked` vectors in memory-critical scenarios.")
        $("///")
        $("/// The exact alignment rules are:")
        $("///")
        $("/// - `VecPacked` vectors are always stored as `[T; N]`.")
        $("///")
        $("/// - `VecAligned` storage is specified by the [`Scalar`] implementation,")
        $("/// and follows the rule of using whatever format is most performant.")
        $("/// This means that if a vector doesn't benefit from SIMD it should not be SIMD-aligned.")
        $("///")
        $("/// # Example")
        $("/// ```")
        $("/// use ggmath::*;")
        $("///")
        $("/// // This is a non memory critical scenario so we should use `VecAligned`.")
        $("/// struct PlayerState {{")
        $("///     // Vector<3, f32, VecAligned>")
        $("///     position: Vec3<f32>,")
        $("/// }}")
        $("///")
        $("/// // This is a memory critical scenario so we should use `VecPacked`.")
        $("/// struct GpuVertex {{")
        $("///     // Vector<3, f32, VecPacked>")
        $("///     position: Vec3P<f32>,")
        $("///     // Vector<3, f32, VecPacked>")
        $("///     normal: Vec3P<f32>,")
        $("///     // Vector<2, f32, VecPacked>")
        $("///     uv: Vec2P<f32>,")
        $("/// }}")
        $("///")
        $("/// fn initial_player_state() -> PlayerState {{")
        $("///     PlayerState {{")
        $("///         position: vec3!(0.0, 1.0, 2.0),")
        $("///     }}")
        $("/// }}")
        $("///")
        $("/// fn triangle_vertices() -> [GpuVertex; 3] {{")
        $("///     [")
        $("///         GpuVertex {{")
        $("///             position: vec3p!(-1.0, -1.0, 0.0),")
        $("///             normal: vec3p!(0.0, 0.0, 1.0),")
        $("///             uv: vec2p!(0.0, 0.0),")
        $("///         }},")
        $("///         GpuVertex {{")
        $("///             position: vec3p!(1.0, -1.0, 0.0),")
        $("///             normal: vec3p!(0.0, 0.0, 1.0),")
        $("///             uv: vec2p!(1.0, 0.0),")
        $("///         }},")
        $("///         GpuVertex {{")
        $("///             position: vec3p!(0.0, 1.0, 0.0),")
        $("///             normal: vec3p!(0.0, 0.0, 1.0),")
        $("///             uv: vec2p!(0.5, 1.0),")
        $("///         }},")
        $("///     ]")
        $("/// }}")
        $("/// ```")
        #[repr(transparent)]
        pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>(pub A::InnerVector<N, T>)
        where
            Usize<N>: VecLen;

        $(
            for &n in LENGTHS =>

            $(format!("/// Type alias for [`Vector<{n}, T, VecAligned>`][Vector]."))
            pub type Vec$(n)<T> = Vector<$n, T, VecAligned>;
            $['\n']
        )
        $(
            for &n in LENGTHS =>

            $(format!("/// Type alias for [`Vector<{n}, T, VecPacked>`][Vector]."))
            pub type Vec$(n)P<T> = Vector<$n, T, VecPacked>;
            $['\n']
        )

        $("/// Generates vector type-aliases for a specific scalar type.")
        $("///")
        $("/// # Example")
        $("///")
        $("/// ```")
        $("/// use ggmath::*;")
        $("///")
        $("/// vector_aliases!(pub type F = f32);")
        $("/// ```")
        $("/// Generates:")
        $("/// ```")
        $(for &n in LENGTHS => $(format!("/// pub type FVec{n} = Vec{n}<f32>;"))$['\r'])
        $(for &n in LENGTHS => $(format!("/// pub type FVec{n}P = Vec{n}P<f32>;"))$['\r'])
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

                        #[doc = $(format!("\"Type alias to [`Vector<{n}, \"")) $$t ", VecAligned>`][Vector]."]
                        $$(#[$$($$attr)*])*
                        $$($$vis)* type [<$$prefix Vec$(n)>] = $$crate::Vec$(n)<$$t>;

                        #[doc = $(format!("\"Type alias to [`Vector<{n}, \"")) $$t ", VecPacked>`][Vector]."]
                        $$(#[$$($$attr)*])*
                        $$($$vis)* type [<$$prefix Vec$(n)P>] = $$crate::Vec$(n)P<$$t>;

                        $['\n']
                    )
                }
            };
        }

        $("/// Marks a `Usize<N>` type as a valid vector length.")
        pub trait VecLen {
            $("/// The inner type contained inside `Vector<N, T, VecAligned>`.")
            $("///")
            $("/// This redirects to `T::InnerAlignedVec{N}`,")
            $("/// for example `T::InnerAlignedVec2` for `Usize<2>`.`")
            type InnerAlignedVector<T: Scalar>: Construct;

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
        pub trait VecAlignment: 'static {
            $("/// The inner type contained inside [`Vector`].")
            $("///")
            $("/// For `VecAligned` vectors this is `T::InnerAlignedVec{N}`,")
            $("/// for example `T::InnerAlignedVec2` for `Vec2`.")
            $("///")
            $("/// For `VecPacked` vectors this is `[T; N]`,")
            $("/// for example `[T; 2]` for `Vec2`.")
            type InnerVector<const N: usize, T: Scalar>: Construct
            where
                Usize<N>: VecLen;

            $("/// Whether or not the vector is SIMD aligned.")
            const IS_ALIGNED: bool;
        }

        $("/// See [`Vector`] for information.")
        pub struct VecAligned;

        $("/// See [`Vector`] for information.")
        pub struct VecPacked;

        $(
            for (&n, &length_name) in LENGTHS.iter().zip(LENGTH_NAMES.iter()) join($['\n'])=>

            impl VecLen for Usize<$n> {
                type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec$(n);

                const ENUM: VecLenEnum = VecLenEnum::$length_name;
            }
        )

        impl VecAlignment for VecAligned {
            type InnerVector<const N: usize, T: Scalar>
                = <Usize<N> as VecLen>::InnerAlignedVector<T>
            where
                Usize<N>: VecLen;

            const IS_ALIGNED: bool = true;
        }

        impl VecAlignment for VecPacked {
            type InnerVector<const N: usize, T: Scalar>
                = [T; N]
            where
                Usize<N>: VecLen;
                
            const IS_ALIGNED: bool = false;
        }

        impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            $("/// Returns true if the vector is aligned.")
            $("/// The output is strictly determined by the type of the vector.")
            #[inline(always)]
            pub const fn is_aligned(self) -> bool {
                A::IS_ALIGNED
            }

            $("/// Converts the vector to an aligned vector.")
            #[inline(always)]
            pub fn align(self) -> Vector<N, T, VecAligned> {
                self.to_storage()
            }

            $("/// Converts the vector to a packed vector.")
            #[inline(always)]
            pub fn pack(self) -> Vector<N, T, VecPacked> {
                self.to_storage()
            }

            $("/// Converts the vector to the specified alignment.")
            #[inline(always)]
            pub fn to_storage<A2: VecAlignment>(self) -> Vector<N, T, A2> {
                specialize! {
                    (self: Vector<N, T, A>) -> Vector<N, T, A2>:

                    for (Vector<N, T, A>) -> Vector<N, T, A> {
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
                    (array: [T; N]) -> Vector<N, T, A>:

                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for ([T; $n]) -> Vector<$n, T, VecAligned> {
                            |array| T::vec$(n)_from_array(array)
                        }
                    )
                    for ([T; N]) -> Vector<N, T, VecPacked> {
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
                    (value: T) -> Vector<N, T, A>:
   
                    $(
                        for &n in LENGTHS join($['\r']) =>
    
                        for (T) -> Vector<$n, T, VecAligned> {
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
                    (self: Vector<N, T, A>) -> [T; N]:

                    $(
                        for &n in LENGTHS join($['\r']) =>
    
                        for (Vector<$n, T, VecAligned>) -> [T; $n] {
                            |vec| T::vec$(n)_as_array(vec)
                        }
                    )
                    for (Vector<N, T, VecPacked>) -> [T; N] {
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
                    (self: Vector<N, T, A>, index: usize) -> T:

                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for (Vector<$n, T, VecAligned>, usize) -> T {
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
                    ((*self): Vector<N, T, A>, index: usize, value: T) -> Vector<N, T, A>:
                    
                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for (Vector<$n, T, VecAligned>, usize, T) -> Vector<$n, T, VecAligned> {
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
                pub fn shuffle_$(n2)<$(for i in 0..n2 join(, ) => const $(COMPONENTS[i].to_uppercase())_SRC: usize)>(self) -> Vector<$n2, T, A> {
                    specialize! {
                        (self: Vector<N, T, A>) -> Vector<$n2, T, A>:

                        $(
                            for &n in LENGTHS join($['\r']) =>

                            for (Vector<$n, T, VecAligned>) -> Vector<$n2, T, VecAligned> {
                                |vec| T::vec$(n)_shuffle_$(n2)::<$(for i in 0..n2 join(, ) => $(COMPONENTS[i].to_uppercase())_SRC)>(vec)
                            }
                        )
                        else {
                            Vector::<$n2, T, A>::from_array([$(for i in 0..n2 join(, ) => self.index($(COMPONENTS[i].to_uppercase())_SRC))])
                        }
                    }
                }
            )

            $("/// Maps each component of the vector to a new value using the given function.")
            #[inline(always)]
            pub fn map<T2: Scalar, F: Fn(T) -> T2>(self, f: F) -> Vector<N, T2, A>
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
                pub fn $(cmp_lower)_mask<T2: Scalar>(self, other: Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
                where
                    T: $cmp_trait<T2>,
                {
                    specialize! {
                        (self: Vector<N, T, A>, other: Vector<N, T2, _>) -> Vector<N, bool, A>:
    
                        $(
                            for &n in LENGTHS join($['\r']) =>
    
                            for (Vector<$n, T, VecAligned>, Vector<$n, T2, VecAligned>) -> Vector<$n, bool, VecAligned> {
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
                    (self: Vector<N, T, A>) -> T:

                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for (Vector<$n, T, VecAligned>) -> T {
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
                    (self: Vector<N, T, A>) -> T:

                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for (Vector<$n, T, VecAligned>) -> T {
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
            pub fn dot(self, other: Vector<N, T, impl VecAlignment>) -> T
            where
                T: Add<Output = T> + Mul<Output = T>,
            {
                (self * other).sum()
            }
        }

        impl<const N: usize, T: Scalar> Vector<N, T, VecPacked>
        where
            Usize<N>: VecLen,
        {
            $("/// Converts an array reference to a vector reference.")
            #[inline(always)]
            pub const fn from_array_ref(array: &[T; N]) -> &Self {
                unsafe { transmute::<&[T; N], &Vector<N, T, VecPacked>>(array) }
            }

            $("/// Converts a mutable array reference to a mutable vector reference.")
            #[inline(always)]
            pub const fn from_mut_array(array: &mut [T; N]) -> &mut Self {
                unsafe { transmute::<&mut [T; N], &mut Vector<N, T, VecPacked>>(array) }
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

            impl<T: Scalar, A: VecAlignment> Vector<$n, T, A> {
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
                    pub fn perp_dot(self, other: Vector<2, T, impl VecAlignment>) -> T
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
                    pub fn cross(self, other: Vector<3, T, impl VecAlignment>) -> Self
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
                    pub fn with_shuffle_$(n2)<$(for i in 0..n2 join(, ) => const $(COMPONENTS[i].to_uppercase())_DST: usize)>(self, value: Vector<$n2, T, impl VecAlignment>) -> Self {
                        specialize! {
                            (self: Vector<$n, T, A>, value: Vector<$n2, T, _>) -> Vector<$n, T, A>:
    
                            for (Vector<$n, T, VecAligned>, Vector<$n2, T, VecAligned>) -> Vector<$n, T, VecAligned> {
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

            impl<T: Scalar> Vector<$n, T, VecPacked> {
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

        impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A> where Usize<N>: VecLen {}

        impl<const N: usize, T: Scalar, A: VecAlignment> IntoIterator for Vector<N, T, A>
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

        impl<'a, const N: usize, T: Scalar> IntoIterator for &'a Vector<N, T, VecPacked>
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

        impl<'a, const N: usize, T: Scalar> IntoIterator for &'a mut Vector<N, T, VecPacked>
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

        impl<const N: usize, T: Scalar, I: SliceIndex<[T]>> Index<I> for Vector<N, T, VecPacked>
        where
            Usize<N>: VecLen,
        {
            type Output = I::Output;

            #[inline(always)]
            fn index(&self, index: I) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<const N: usize, T: Scalar, I: SliceIndex<[T]>> IndexMut<I> for Vector<N, T, VecPacked>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn index_mut(&mut self, index: I) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl<const N: usize, T: Scalar + PartialEq<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
            PartialEq<Vector<N, T2, A2>> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn eq(&self, other: &Vector<N, T2, A2>) -> bool {
                specialize! {
                    ((*self): Vector<N, T, A>, (*other): Vector<N, T2, A2>) -> bool:

                    $(
                        for &n in LENGTHS join($['\r']) =>
                        
                        for (Vector<$n, T, VecAligned>, Vector<$n, T2, VecAligned>) -> bool {
                            |vec, other| T::vec$(n)_eq(vec, other)
                        }
                    )
                    else {
                        self.iter().zip(*other).all(|(a, b)| a == b)
                    }
                }
            }

            #[inline(always)]
            fn ne(&self, other: &Vector<N, T2, A2>) -> bool {
                specialize! {
                    ((*self): Vector<N, T, A>, (*other): Vector<N, T2, A2>) -> bool:

                    $(
                        for &n in LENGTHS join($['\r']) =>

                        for (Vector<$n, T, VecAligned>, Vector<$n, T2, VecAligned>) -> bool {
                            |vec, other| T::vec$(n)_ne(vec, other)
                        }
                    )
                    else {
                        self.iter().zip(*other).any(|(a, b)| a != b)
                    }
                }
            }
        }

        impl<const N: usize, T: Scalar + Eq, A: VecAlignment> Eq for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {}

        impl<const N: usize, T: Scalar + Hash, A: VecAlignment> Hash for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.as_array().hash(state)
            }
        }

        impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn default() -> Self {
                Self::splat(T::default())
            }
        }

        impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
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

        impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
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
    .to_mod_dir("vector")
    .with_submod_dir(primitives::mod_())
    .with_submod_file(constructor::mod_())
    .with_submod_file(swizzle::mod_())
    .with_submod_file(scalar::mod_())
    .with_submod_file(ops::mod_())
    .with_submod_file(dir::mod_())
}
