use super::*;

macro_rules! swizzle_get_fns {
    { [$($x:ident => $idx:literal)*] } => {
        $(swizzle_get_fns!($x => $idx);)*
    };

    { [$($x:ident)*] $ys:tt } => {
        $(swizzle_get_fns!($x $ys);)*
    };
    { $x:ident [$($y:ident)*] } => {
        $(swizzle_get_fns!($x $y);)*
    };

    { [$($x:ident)*] $ys:tt $zs:tt } => {
        $(swizzle_get_fns!($x $ys $zs);)*
    };
    { $x:ident [$($y:ident)*] $zs:tt } => {
        $(swizzle_get_fns!($x $y $zs);)*
    };
    { $x:ident $y:ident [$($z:ident)*] } => {
        $(swizzle_get_fns!($x $y $z);)*
    };

    { [$($x:ident)*] $ys:tt $zs:tt $ws:tt } => {
        $(swizzle_get_fns!($x $ys $zs $ws);)*
    };
    { $x:ident [$($y:ident)*] $zs:tt $ws:tt } => {
        $(swizzle_get_fns!($x $y $zs $ws);)*
    };
    { $x:ident $y:ident [$($z:ident)*] $ws:tt } => {
        $(swizzle_get_fns!($x $y $z $ws);)*
    };
    { $x:ident $y:ident $z:ident [$($w:ident)*] } => {
        $(swizzle_get_fns!($x $y $z $w);)*
    };

    { $x:ident => $idx:literal } => {
        paste::paste! {
            #[inline(always)]
            pub const fn [<$x>](self) -> T {
                self.array[$idx]
            }
        }
    };
    { $x:ident $y:ident } => {
        paste::paste! {
            #[inline(always)]
            pub const fn [<$x $y>](self) -> Vector<2, T, A> {
                Vector::from_array([self.$x(), self.$y()])
            }
        }
    };
    { $x:ident $y:ident $z:ident } => {
        paste::paste! {
            #[inline(always)]
            pub const fn [<$x $y $z>](self) -> Vector<3, T, A> {
                Vector::from_array([self.$x(), self.$y(), self.$z()])
            }
        }
    };
    { $x:ident $y:ident $z:ident $w:ident } => {
        paste::paste! {
            #[inline(always)]
            pub const fn [<$x $y $z $w>](self) -> Vector<4, T, A> {
                Vector::from_array([self.$x(), self.$y(), self.$z(), self.$w()])
            }
        }
    };
}

macro_rules! swizzle_get_ref_fns {
    { $($components:ident => $start:literal $(..=$end:literal)?), * $(,)? } => {
        $(swizzle_get_ref_fns!(@ $components => $start $(..=$end)?);)*
    };

    { @ $components:ident => $idx:literal } => {
        paste::paste! {
            #[inline(always)]
            pub const fn [<$components _ref>](&self) -> &T {
                &self.array[$idx]
            }
        }
    };
    { @ $components:ident => $start:literal..=$end:literal } => {
        paste::paste! {
            #[inline(always)]
            pub const fn [<$components _ref>](&self) -> &Vector<{ $end - $start + 1 }, T, VecPacked> {
                self.get_vec_ref::<{ $end - $start + 1 }>($start).unwrap()
            }
        }
    };
}

macro_rules! swizzle_get_mut_fns {
    { $(($($components:ident => $start:literal $(..=$end:literal)?), * $(,)?)), * $(,)? } => {
        $(swizzle_get_mut_fns!(@ ($($components => $start $(..=$end)?), *));)*
    };

    { @ ($($components:ident => $idx:literal $(..=$end:literal)?), * $(,)?) } => {
        paste::paste! {
            #[inline(always)]
            #[allow(unused_parens)]
            pub const fn [<$($components)_* _mut>](&mut self) -> ($(swizzle_get_mut_fns!(@output $components => $idx $(..=$end)?)), *) {
                ($(swizzle_get_mut_fns!(@eval self $components => $idx $(..=$end)?)), *)
            }
        }
    };

    { @output $components:ident => $idx:literal } => {
        &mut T
    };
    { @output $components:ident => $start:literal..=$end:literal } => {
        &mut Vector<{ $end - $start + 1 }, T, VecPacked>
    };

    { @eval $self:ident $components:ident => $idx:literal } => {
        unsafe { std::mem::transmute::<&mut T, &mut T>($self.get_mut_unchecked($idx)) }
    };
    { @eval $self:ident $components:ident => $start:literal..=$end:literal } => {
        unsafe {
            std::mem::transmute::<&mut Vector<{ $end - $start + 1 }, T, VecPacked>, &mut Vector<{ $end - $start + 1 }, T, VecPacked>>(
                $self.get_vec_mut_unchecked::<{ $end - $start + 1 }>($start)
            )
        }
    };
}

macro_rules! swizzle_set_and_with_fns {
    { [$($x:ident)*] } => {
        $(swizzle_set_and_with_fns!($x);)*
    };

    { [$($x:ident)*] $ys:tt } => {
        $(swizzle_set_and_with_fns!($x $ys);)*
    };
    { $x:ident [$($y:ident)*] } => {
        $(swizzle_set_and_with_fns!($x $y);)*
    };

    { [$($x:ident)*] $ys:tt $zs:tt } => {
        $(swizzle_set_and_with_fns!($x $ys $zs);)*
    };
    { $x:ident [$($y:ident)*] $zs:tt } => {
        $(swizzle_set_and_with_fns!($x $y $zs);)*
    };
    { $x:ident $y:ident [$($z:ident)*] } => {
        $(swizzle_set_and_with_fns!($x $y $z);)*
    };

    { [$($x:ident)*] $ys:tt $zs:tt $ws:tt } => {
        $(swizzle_set_and_with_fns!($x $ys $zs $ws);)*
    };
    { $x:ident [$($y:ident)*] $zs:tt $ws:tt } => {
        $(swizzle_set_and_with_fns!($x $y $zs $ws);)*
    };
    { $x:ident $y:ident [$($z:ident)*] $ws:tt } => {
        $(swizzle_set_and_with_fns!($x $y $z $ws);)*
    };
    { $x:ident $y:ident $z:ident [$($w:ident)*] } => {
        $(swizzle_set_and_with_fns!($x $y $z $w);)*
    };

    { x x } => {};
    { y y } => {};
    { z z } => {};
    { w w } => {};

    { x x $_:ident } => {};
    { y y $_:ident } => {};
    { z z $_:ident } => {};
    { w w $_:ident } => {};
    { x $_:ident x } => {};
    { y $_:ident y } => {};
    { z $_:ident z } => {};
    { w $_:ident w } => {};
    { $_:ident x x } => {};
    { $_:ident y y } => {};
    { $_:ident z z } => {};
    { $_:ident w w } => {};

    { x x $_:ident $__:ident } => {};
    { y y $_:ident $__:ident } => {};
    { z z $_:ident $__:ident } => {};
    { w w $_:ident $__:ident } => {};
    { x $_:ident x $__:ident } => {};
    { y $_:ident y $__:ident } => {};
    { z $_:ident z $__:ident } => {};
    { w $_:ident w $__:ident } => {};
    { $_:ident x x $__:ident } => {};
    { $_:ident y y $__:ident } => {};
    { $_:ident z z $__:ident } => {};
    { $_:ident w w $__:ident } => {};

    { x $_:ident $__:ident x } => {};
    { y $_:ident $__:ident y } => {};
    { z $_:ident $__:ident z } => {};
    { w $_:ident $__:ident w } => {};
    { $_:ident x $__:ident x } => {};
    { $_:ident y $__:ident y } => {};
    { $_:ident z $__:ident z } => {};
    { $_:ident w $__:ident w } => {};
    { $_:ident $__:ident x x } => {};
    { $_:ident $__:ident y y } => {};
    { $_:ident $__:ident z z } => {};
    { $_:ident $__:ident w w } => {};

    { $x:ident } => {
        paste::paste! {
            #[inline(always)]
            pub const fn [<set_ $x>](&mut self, value: T) {
                *self.[<$x _mut>]() = value;
            }

            #[inline(always)]
            pub const fn [<with_ $x>](mut self, value: T) -> Self {
                self.[<set_ $x>](value);

                self
            }
        }
    };
    { $x:ident $y:ident } => {
        paste::paste! {
            #[inline(always)]
            pub const fn [<set_ $x $y>](&mut self, value: Vector<2, T, impl VecAlignment>) {
                *self.[<$x _mut>]() = value.x();
                *self.[<$y _mut>]() = value.y();
            }

            #[inline(always)]
            pub const fn [<with_ $x $y>](mut self, value: Vector<2, T, impl VecAlignment>) -> Self {
                self.[<set_ $x $y>](value);

                self
            }
        }
    };
    { $x:ident $y:ident $z:ident } => {
        paste::paste! {
            #[inline(always)]
            pub const fn [<set_ $x $y $z>](&mut self, value: Vector<3, T, impl VecAlignment>) {
                *self.[<$x _mut>]() = value.x();
                *self.[<$y _mut>]() = value.y();
                *self.[<$z _mut>]() = value.z();
            }

            #[inline(always)]
            pub const fn [<with_ $x $y $z>](mut self, value: Vector<3, T, impl VecAlignment>) -> Self {
                self.[<set_ $x $y $z>](value);

                self
            }
        }
    };
    { $x:ident $y:ident $z:ident $w:ident } => {
        paste::paste! {
            #[inline(always)]
            pub const fn [<set_ $x $y $z $w>](&mut self, value: Vector<4, T, impl VecAlignment>) {
                *self.[<$x _mut>]() = value.x();
                *self.[<$y _mut>]() = value.y();
                *self.[<$z _mut>]() = value.z();
                *self.[<$w _mut>]() = value.w();
            }

            #[inline(always)]
            pub const fn [<with_ $x $y $z $w>](mut self, value: Vector<4, T, impl VecAlignment>) -> Self {
                self.[<set_ $x $y $z $w>](value);

                self
            }
        }
    };
}

impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    swizzle_get_fns! { [x => 0 y => 1] }
    swizzle_get_fns! { [x y] [x y] }
    swizzle_get_fns! { [x y] [x y] [x y] }
    swizzle_get_fns! { [x y] [x y] [x y] [x y] }

    swizzle_get_ref_fns! {
        x => 0,
        y => 1,
        xy => 0..=1,
    }

    swizzle_get_mut_fns! {
        (x => 0),
        (y => 1),
        (xy => 0..=1),

        (x => 0, y => 1),
    }

    swizzle_set_and_with_fns! { [x y] }
    swizzle_set_and_with_fns! { [x y] [x y] }
}

impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    swizzle_get_fns! { [x => 0 y => 1 z => 2] }
    swizzle_get_fns! { [x y z] [x y z] }
    swizzle_get_fns! { [x y z] [x y z] [x y z] }
    swizzle_get_fns! { [x y z] [x y z] [x y z] [x y z] }

    swizzle_get_ref_fns! {
        x => 0,
        y => 1,
        z => 2,
        xy => 0..=1,
        yz => 1..=2,
        xyz => 0..=2,
    }

    swizzle_get_mut_fns! {
        (x => 0),
        (y => 1),
        (z => 2),
        (xy => 0..=1),
        (yz => 1..=2),
        (xyz => 0..=2),

        (x => 0, y => 1),
        (x => 0, z => 2),
        (x => 0, yz => 1..=2),
        (y => 1, z => 2),
        (xy => 0..=1, z => 2),

        (x => 0, y => 1, z => 2),
    }

    swizzle_set_and_with_fns! { [x y z] }
    swizzle_set_and_with_fns! { [x y z] [x y z] }
    swizzle_set_and_with_fns! { [x y z] [x y z] [x y z] }
}

impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    swizzle_get_fns! { [x => 0 y => 1 z => 2 w => 3] }
    swizzle_get_fns! { [x y z w] [x y z w] }
    swizzle_get_fns! { [x y z w] [x y z w] [x y z w] }
    swizzle_get_fns! { [x y z w] [x y z w] [x y z w] [x y z w] }

    swizzle_get_ref_fns! {
        x => 0,
        y => 1,
        z => 2,
        w => 3,
        xy => 0..=1,
        yz => 1..=2,
        zw => 2..=3,
        xyz => 0..=2,
        yzw => 1..=3,
        xyzw => 0..=3,
    }

    swizzle_get_mut_fns! {
        (x => 0),
        (y => 1),
        (z => 2),
        (w => 3),
        (xy => 0..=1),
        (yz => 1..=2),
        (zw => 2..=3),
        (xyz => 0..=2),
        (yzw => 1..=3),
        (xyzw => 0..=3),

        (x => 0, y => 1),
        (x => 0, z => 2),
        (x => 0, w => 3),
        (x => 0, yz => 1..=2),
        (x => 0, zw => 2..=3),
        (x => 0, yzw => 1..=3),
        (y => 1, z => 2),
        (y => 1, w => 3),
        (y => 1, zw => 2..=3),
        (z => 2, w => 3),
        (xy => 0..=1, z => 2),
        (xy => 0..=1, w => 3),
        (xy => 0..=1, zw => 2..=3),
        (yz => 1..=2, w => 3),
        (xyz => 0..=2, w => 3),

        (x => 0, y => 1, z => 2),
        (x => 0, y => 1, w => 3),
        (x => 0, y => 1, zw => 2..=3),
        (x => 0, z => 2, w => 3),
        (x => 0, yz => 1..=2, w => 3),
        (xy => 0..=1, z => 2, w => 3),

        (x => 0, y => 1, z => 2, w => 3),
    }

    swizzle_set_and_with_fns! { [x y z w] }
    swizzle_set_and_with_fns! { [x y z w] [x y z w] }
    swizzle_set_and_with_fns! { [x y z w] [x y z w] [x y z w] }
    swizzle_set_and_with_fns! { [x y z w] [x y z w] [x y z w] [x y z w] }
}
