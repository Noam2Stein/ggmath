use super::*;

/// Empty type that is aligned to the specified power of two.
///
/// ```rust
/// const _: () = assert!(size_of::<Align<16>>() == 0);
/// const _: () = assert!(align_of::<Align<16>>() == 16);
/// ```
#[allow(private_bounds)]
#[derive(Clone, Copy, Default)]
pub struct Align<const A: usize>
where
    Usize<A>: PowOfTwo,
{
    _inner: <Usize<A> as PowOfTwo>::Archetype,
}

/// Is only implemented for `Align<A>`s.
/// Is made for this pattern:
///
/// ```rust
/// trait MyTrait {
///     type AlignPlease: AlignTrait;
/// }
///
/// impl MyTrait for ... {
///     type AlignPlease = Align<16>;
/// }
/// ```
pub unsafe trait AlignTrait: Construct + Default {
    /// A value of the type.
    ///
    /// Alignment types are zero-sized and always have only a single value.
    /// This constant allows safe initialization of the type.
    const VALUE: Self;
}

trait PowOfTwo {
    type Archetype: Construct + Default;

    const ARCHETYPE: Self::Archetype;
}

unsafe impl<const A: usize> AlignTrait for Align<A>
where
    Usize<A>: PowOfTwo,
{
    const VALUE: Self = Self {
        _inner: <Usize<A> as PowOfTwo>::ARCHETYPE,
    };
}

repetitive! {
    @for A in [
        1,
        2,
        4,
        8,
        16,
        32,
        64,
        128,
        256,
        512,
        1024,
        2048,
        4096,
        8192,
        16384,
        32768,
        65536,
        131072,
        262144,
        524288,
    ] {
        @let ArchetypeA = @['Archetype A];

        impl PowOfTwo for Usize<@A> {
            type Archetype = @ArchetypeA;

            const ARCHETYPE: Self::Archetype = @ArchetypeA;
        }

        #[repr(align(@A))]
        #[derive(Clone, Copy, Default)]
        struct @ArchetypeA;

        const _: () = assert!(size_of::<@ArchetypeA>() == 0);
        const _: () = assert!(align_of::<@ArchetypeA>() == @A);
    }
}
