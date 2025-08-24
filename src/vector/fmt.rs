use std::fmt::{Debug, Display, Formatter, Result};

use super::*;

impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({})",
            self.to_array().map(|c| format!("{c:?}")).join(", ")
        )
    }
}

impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({})", self.to_array().map(|c| c.to_string()).join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!(format!("{:?}", vec2!(1, 2)), "(1, 2)");
        assert_eq!(format!("{}", vec3p!(1, 2, 3)), "(1, 2, 3)");

        assert_eq!(format!("{:?}", vec2!(1.0, 2.0)), "(1.0, 2.0)");
        assert_eq!(format!("{}", vec4p!(1.0, 2.0, 3.0, 4.0)), "(1, 2, 3, 4)");
    }
}
