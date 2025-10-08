use ggmath::{f32_aliases::FVec3, vec3};

fn main() {
    // In here, right is the positive direction.
    {
        use ggmath::right::*;

        assert_eq!(FVec3::RIGHT, vec3!(1.0, 0.0, 0.0));
        assert_eq!(FVec3::LEFT, vec3!(-1.0, 0.0, 0.0));
    }

    // In here, left is the positive direction.
    {
        use ggmath::left::*;

        assert_eq!(FVec3::RIGHT, vec3!(-1.0, 0.0, 0.0));
        assert_eq!(FVec3::LEFT, vec3!(1.0, 0.0, 0.0));
    }

    // We can do the same for up, down, forwards, and backwards.
}
