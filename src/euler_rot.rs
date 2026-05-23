// Ported from https://github.com/bitshifter/glam-rs.

// Based on Ken Shoemake. 1994. Euler angle conversion. Graphics gems IV.
// Academic Press Professional, Inc., USA, 222–229.

/// An Euler rotation order/sequence.
///
/// Three-axis rotations may be extrinsic (rotations about the axes XYZ of the
/// original coordinate system, which remains motionless), or intrinsic
/// (rotations about the axes of the rotating coordinate system XYZ, which
/// changes its orientation after each elemental rotation).
///
/// ```
/// # use ggmath::{EulerRot, Mat3};
/// #
/// # let x = core::f32::consts::FRAC_PI_2;
/// # let y = core::f32::consts::FRAC_PI_4;
/// # let z = core::f32::consts::FRAC_PI_8;
/// #
/// let intrinsic = Mat3::<f32>::from_euler(EulerRot::Yxz, y, x, z);
/// let other = Mat3::<f32>::from_rotation_z(z)
///     * Mat3::<f32>::from_rotation_x(x)
///     * Mat3::<f32>::from_rotation_y(y);
/// assert!(intrinsic.abs_diff_eq(&other, 2e-6));
///
/// let extrinsic = Mat3::<f32>::from_euler(EulerRot::YxzEx, y, x, z);
/// let other = Mat3::<f32>::from_rotation_y(y)
///     * Mat3::<f32>::from_rotation_x(x)
///     * Mat3::<f32>::from_rotation_z(z);
/// assert!(extrinsic.abs_diff_eq(&other, 2e-6));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EulerRot {
    /// Intrinsic three-axis rotation ZYX.
    Zyx,
    /// Intrinsic three-axis rotation ZXY.
    Zxy,
    /// Intrinsic three-axis rotation YXZ.
    Yxz,
    /// Intrinsic three-axis rotation YZX.
    Yzx,
    /// Intrinsic three-axis rotation XYZ.
    Xyz,
    /// Intrinsic three-axis rotation XZY.
    Xzy,

    /// Intrinsic two-axis rotation ZYZ.
    Zyz,
    /// Intrinsic two-axis rotation ZXZ.
    Zxz,
    /// Intrinsic two-axis rotation YXY.
    Yxy,
    /// Intrinsic two-axis rotation YZY.
    Yzy,
    /// Intrinsic two-axis rotation XYX.
    Xyx,
    /// Intrinsic two-axis rotation XZX.
    Xzx,

    /// Extrinsic three-axis rotation ZYX.
    ZyxEx,
    /// Extrinsic three-axis rotation ZXY.
    ZxyEx,
    /// Extrinsic three-axis rotation YXZ.
    YxzEx,
    /// Extrinsic three-axis rotation YZX.
    YzxEx,
    /// Extrinsic three-axis rotation XYZ.
    XyzEx,
    /// Extrinsic three-axis rotation XZY.
    XzyEx,

    /// Extrinsic two-axis rotation ZYZ.
    ZyzEx,
    /// Extrinsic two-axis rotation ZXZ.
    ZxzEx,
    /// Extrinsic two-axis rotation YXY.
    YxyEx,
    /// Extrinsic two-axis rotation YZY.
    YzyEx,
    /// Extrinsic two-axis rotation XYX.
    XyxEx,
    /// Extrinsic two-axis rotation XZX.
    XzxEx,
}

impl Default for EulerRot {
    /// The default value [`Yxz`] as yaw (y-axis), pitch (x-axis) and roll
    /// (z-axis).
    ///
    /// [`Yxz`]: Self::Yxz
    #[inline]
    fn default() -> Self {
        Self::Yxz
    }
}

#[derive(Clone, Copy)]
pub(crate) struct EulerRotProperties {
    pub(crate) initial_axis: usize,
    pub(crate) parity_even: bool,
    pub(crate) initial_repeated: bool,
    pub(crate) frame_static: bool,
}

impl EulerRot {
    pub(crate) fn properties(self) -> EulerRotProperties {
        enum Axis {
            X = 0,
            Y = 1,
            Z = 2,
        }

        enum Parity {
            Odd = 0,
            Even = 1,
        }

        enum Repeated {
            No = 0,
            Yes = 1,
        }

        enum Frame {
            Relative = 0,
            Static = 1,
        }

        fn properties(
            initial_axis: Axis,
            parity: Parity,
            repeated: Repeated,
            frame: Frame,
        ) -> EulerRotProperties {
            EulerRotProperties {
                initial_axis: initial_axis as usize,
                parity_even: parity as u32 == Parity::Even as u32,
                initial_repeated: repeated as u32 == Repeated::Yes as u32,
                frame_static: frame as u32 == Frame::Static as u32,
            }
        }

        match self {
            Self::Zyx => properties(Axis::Z, Parity::Odd, Repeated::No, Frame::Static),
            Self::Zxy => properties(Axis::Z, Parity::Even, Repeated::No, Frame::Static),
            Self::Yxz => properties(Axis::Y, Parity::Odd, Repeated::No, Frame::Static),
            Self::Yzx => properties(Axis::Y, Parity::Even, Repeated::No, Frame::Static),
            Self::Xyz => properties(Axis::X, Parity::Even, Repeated::No, Frame::Static),
            Self::Xzy => properties(Axis::X, Parity::Odd, Repeated::No, Frame::Static),

            Self::Zyz => properties(Axis::Z, Parity::Odd, Repeated::Yes, Frame::Static),
            Self::Zxz => properties(Axis::Z, Parity::Even, Repeated::Yes, Frame::Static),
            Self::Yxy => properties(Axis::Y, Parity::Odd, Repeated::Yes, Frame::Static),
            Self::Yzy => properties(Axis::Y, Parity::Even, Repeated::Yes, Frame::Static),
            Self::Xyx => properties(Axis::X, Parity::Even, Repeated::Yes, Frame::Static),
            Self::Xzx => properties(Axis::X, Parity::Odd, Repeated::Yes, Frame::Static),

            Self::ZyxEx => properties(Axis::X, Parity::Even, Repeated::No, Frame::Relative),
            Self::ZxyEx => properties(Axis::Y, Parity::Odd, Repeated::No, Frame::Relative),
            Self::YxzEx => properties(Axis::Z, Parity::Even, Repeated::No, Frame::Relative),
            Self::YzxEx => properties(Axis::X, Parity::Odd, Repeated::No, Frame::Relative),
            Self::XyzEx => properties(Axis::Z, Parity::Odd, Repeated::No, Frame::Relative),
            Self::XzyEx => properties(Axis::Y, Parity::Even, Repeated::No, Frame::Relative),

            Self::ZyzEx => properties(Axis::Z, Parity::Odd, Repeated::Yes, Frame::Relative),
            Self::ZxzEx => properties(Axis::Z, Parity::Even, Repeated::Yes, Frame::Relative),
            Self::YxyEx => properties(Axis::Y, Parity::Odd, Repeated::Yes, Frame::Relative),
            Self::YzyEx => properties(Axis::Y, Parity::Even, Repeated::Yes, Frame::Relative),
            Self::XyxEx => properties(Axis::X, Parity::Even, Repeated::Yes, Frame::Relative),
            Self::XzxEx => properties(Axis::X, Parity::Odd, Repeated::Yes, Frame::Relative),
        }
    }
}

impl EulerRotProperties {
    pub(crate) fn axes_indices(self) -> (usize, usize, usize) {
        fn next_axis(i: usize) -> usize {
            (i + 1) % 3
        }

        fn prev_axis(i: usize) -> usize {
            if i > 0 { i - 1 } else { 2 }
        }

        let i = self.initial_axis;
        let j = if self.parity_even {
            next_axis(i)
        } else {
            prev_axis(i)
        };
        let k = if self.parity_even {
            prev_axis(i)
        } else {
            next_axis(i)
        };

        (i, j, k)
    }
}
