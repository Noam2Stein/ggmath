use super::*;

impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn default() -> Self {
        Self::splat(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(Vec2::<f32>::default(), splat2(f32::default()));
        assert_eq!(Vec3::<f32>::default(), splat3(f32::default()));
        assert_eq!(Vec4::<f32>::default(), splat4(f32::default()));
        assert_eq!(Vec2P::<f32>::default(), splat2p(f32::default()));
        assert_eq!(Vec3P::<f32>::default(), splat3p(f32::default()));
        assert_eq!(Vec4P::<f32>::default(), splat4p(f32::default()));
    }
}
