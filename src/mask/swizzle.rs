use crate::{Alignment, Mask, Scalar};

impl<T, A: Alignment> Mask<2, T, A>
where
    T: Scalar,
{
    declare_swizzle_fns! {
        xx: [x, x],
        xy: [x, y],
        yx: [y, x],
        yy: [y, y],
    }

    declare_swizzle_fns! {
        xxx: [x, x, x],
        xxy: [x, x, y],
        xyx: [x, y, x],
        xyy: [x, y, y],
        yxx: [y, x, x],
        yxy: [y, x, y],
        yyx: [y, y, x],
        yyy: [y, y, y],
    }

    declare_swizzle_fns! {
        xxxx: [x, x, x, x],
        xxxy: [x, x, x, y],
        xxyx: [x, x, y, x],
        xxyy: [x, x, y, y],
        xyxx: [x, y, x, x],
        xyxy: [x, y, x, y],
        xyyx: [x, y, y, x],
        xyyy: [x, y, y, y],
        yxxx: [y, x, x, x],
        yxxy: [y, x, x, y],
        yxyx: [y, x, y, x],
        yxyy: [y, x, y, y],
        yyxx: [y, y, x, x],
        yyxy: [y, y, x, y],
        yyyx: [y, y, y, x],
        yyyy: [y, y, y, y],
    }

    declare_with_swizzle_fns! {
        with_x: ([x], [set_x]),
        with_y: ([y], [set_y]),
    }

    declare_with_swizzle_fns! {
        with_xy: ([x, y], [set_x, set_y]),
        with_yx: ([y, x], [set_y, set_x]),
    }
}

impl<T, A: Alignment> Mask<3, T, A>
where
    T: Scalar,
{
    declare_swizzle_fns! {
        xx: [x, x],
        xy: [x, y],
        xz: [x, z],
        yx: [y, x],
        yy: [y, y],
        yz: [y, z],
        zx: [z, x],
        zy: [z, y],
        zz: [z, z],
    }

    declare_swizzle_fns! {
        xxx: [x, x, x],
        xxy: [x, x, y],
        xxz: [x, x, z],
        xyx: [x, y, x],
        xyy: [x, y, y],
        xyz: [x, y, z],
        xzx: [x, z, x],
        xzy: [x, z, y],
        xzz: [x, z, z],
        yxx: [y, x, x],
        yxy: [y, x, y],
        yxz: [y, x, z],
        yyx: [y, y, x],
        yyy: [y, y, y],
        yyz: [y, y, z],
        yzx: [y, z, x],
        yzy: [y, z, y],
        yzz: [y, z, z],
        zxx: [z, x, x],
        zxy: [z, x, y],
        zxz: [z, x, z],
        zyx: [z, y, x],
        zyy: [z, y, y],
        zyz: [z, y, z],
        zzx: [z, z, x],
        zzy: [z, z, y],
        zzz: [z, z, z],
    }

    declare_swizzle_fns! {
        xxxx: [x, x, x, x],
        xxxy: [x, x, x, y],
        xxxz: [x, x, x, z],
        xxyx: [x, x, y, x],
        xxyy: [x, x, y, y],
        xxyz: [x, x, y, z],
        xxzx: [x, x, z, x],
        xxzy: [x, x, z, y],
        xxzz: [x, x, z, z],
        xyxx: [x, y, x, x],
        xyxy: [x, y, x, y],
        xyxz: [x, y, x, z],
        xyyx: [x, y, y, x],
        xyyy: [x, y, y, y],
        xyyz: [x, y, y, z],
        xyzx: [x, y, z, x],
        xyzy: [x, y, z, y],
        xyzz: [x, y, z, z],
        xzxx: [x, z, x, x],
        xzxy: [x, z, x, y],
        xzxz: [x, z, x, z],
        xzyx: [x, z, y, x],
        xzyy: [x, z, y, y],
        xzyz: [x, z, y, z],
        xzzx: [x, z, z, x],
        xzzy: [x, z, z, y],
        xzzz: [x, z, z, z],
        yxxx: [y, x, x, x],
        yxxy: [y, x, x, y],
        yxxz: [y, x, x, z],
        yxyx: [y, x, y, x],
        yxyy: [y, x, y, y],
        yxyz: [y, x, y, z],
        yxzx: [y, x, z, x],
        yxzy: [y, x, z, y],
        yxzz: [y, x, z, z],
        yyxx: [y, y, x, x],
        yyxy: [y, y, x, y],
        yyxz: [y, y, x, z],
        yyyx: [y, y, y, x],
        yyyy: [y, y, y, y],
        yyyz: [y, y, y, z],
        yyzx: [y, y, z, x],
        yyzy: [y, y, z, y],
        yyzz: [y, y, z, z],
        yzxx: [y, z, x, x],
        yzxy: [y, z, x, y],
        yzxz: [y, z, x, z],
        yzyx: [y, z, y, x],
        yzyy: [y, z, y, y],
        yzyz: [y, z, y, z],
        yzzx: [y, z, z, x],
        yzzy: [y, z, z, y],
        yzzz: [y, z, z, z],
        zxxx: [z, x, x, x],
        zxxy: [z, x, x, y],
        zxxz: [z, x, x, z],
        zxyx: [z, x, y, x],
        zxyy: [z, x, y, y],
        zxyz: [z, x, y, z],
        zxzx: [z, x, z, x],
        zxzy: [z, x, z, y],
        zxzz: [z, x, z, z],
        zyxx: [z, y, x, x],
        zyxy: [z, y, x, y],
        zyxz: [z, y, x, z],
        zyyx: [z, y, y, x],
        zyyy: [z, y, y, y],
        zyyz: [z, y, y, z],
        zyzx: [z, y, z, x],
        zyzy: [z, y, z, y],
        zyzz: [z, y, z, z],
        zzxx: [z, z, x, x],
        zzxy: [z, z, x, y],
        zzxz: [z, z, x, z],
        zzyx: [z, z, y, x],
        zzyy: [z, z, y, y],
        zzyz: [z, z, y, z],
        zzzx: [z, z, z, x],
        zzzy: [z, z, z, y],
        zzzz: [z, z, z, z],
    }

    declare_with_swizzle_fns! {
        with_x: ([x], [set_x]),
        with_y: ([y], [set_y]),
        with_z: ([z], [set_z]),
    }

    declare_with_swizzle_fns! {
        with_xy: ([x, y], [set_x, set_y]),
        with_xz: ([x, z], [set_x, set_z]),
        with_yx: ([y, x], [set_y, set_x]),
        with_yz: ([y, z], [set_y, set_z]),
        with_zx: ([z, x], [set_z, set_x]),
        with_zy: ([z, y], [set_z, set_y]),
    }

    declare_with_swizzle_fns! {
        with_xyz: ([x, y, z], [set_x, set_y, set_z]),
        with_xzy: ([x, z, y], [set_x, set_z, set_y]),
        with_yxz: ([y, x, z], [set_y, set_x, set_z]),
        with_yzx: ([y, z, x], [set_y, set_z, set_x]),
        with_zxy: ([z, x, y], [set_z, set_x, set_y]),
        with_zyx: ([z, y, x], [set_z, set_y, set_x]),
    }
}

impl<T, A: Alignment> Mask<4, T, A>
where
    T: Scalar,
{
    declare_swizzle_fns! {
        xx: [x, x],
        xy: [x, y],
        xz: [x, z],
        xw: [x, w],
        yx: [y, x],
        yy: [y, y],
        yz: [y, z],
        yw: [y, w],
        zx: [z, x],
        zy: [z, y],
        zz: [z, z],
        zw: [z, w],
        wx: [w, x],
        wy: [w, y],
        wz: [w, z],
        ww: [w, w],
    }

    declare_swizzle_fns! {
        xxx: [x, x, x],
        xxy: [x, x, y],
        xxz: [x, x, z],
        xxw: [x, x, w],
        xyx: [x, y, x],
        xyy: [x, y, y],
        xyz: [x, y, z],
        xyw: [x, y, w],
        xzx: [x, z, x],
        xzy: [x, z, y],
        xzz: [x, z, z],
        xzw: [x, z, w],
        xwx: [x, w, x],
        xwy: [x, w, y],
        xwz: [x, w, z],
        xww: [x, w, w],
        yxx: [y, x, x],
        yxy: [y, x, y],
        yxz: [y, x, z],
        yxw: [y, x, w],
        yyx: [y, y, x],
        yyy: [y, y, y],
        yyz: [y, y, z],
        yyw: [y, y, w],
        yzx: [y, z, x],
        yzy: [y, z, y],
        yzz: [y, z, z],
        yzw: [y, z, w],
        ywx: [y, w, x],
        ywy: [y, w, y],
        ywz: [y, w, z],
        yww: [y, w, w],
        zxx: [z, x, x],
        zxy: [z, x, y],
        zxz: [z, x, z],
        zxw: [z, x, w],
        zyx: [z, y, x],
        zyy: [z, y, y],
        zyz: [z, y, z],
        zyw: [z, y, w],
        zzx: [z, z, x],
        zzy: [z, z, y],
        zzz: [z, z, z],
        zzw: [z, z, w],
        zwx: [z, w, x],
        zwy: [z, w, y],
        zwz: [z, w, z],
        zww: [z, w, w],
        wxx: [w, x, x],
        wxy: [w, x, y],
        wxz: [w, x, z],
        wxw: [w, x, w],
        wyx: [w, y, x],
        wyy: [w, y, y],
        wyz: [w, y, z],
        wyw: [w, y, w],
        wzx: [w, z, x],
        wzy: [w, z, y],
        wzz: [w, z, z],
        wzw: [w, z, w],
        wwx: [w, w, x],
        wwy: [w, w, y],
        wwz: [w, w, z],
        www: [w, w, w],
    }

    declare_swizzle_fns! {
        xxxx: [x, x, x, x],
        xxxy: [x, x, x, y],
        xxxz: [x, x, x, z],
        xxxw: [x, x, x, w],
        xxyx: [x, x, y, x],
        xxyy: [x, x, y, y],
        xxyz: [x, x, y, z],
        xxyw: [x, x, y, w],
        xxzx: [x, x, z, x],
        xxzy: [x, x, z, y],
        xxzz: [x, x, z, z],
        xxzw: [x, x, z, w],
        xxwx: [x, x, w, x],
        xxwy: [x, x, w, y],
        xxwz: [x, x, w, z],
        xxww: [x, x, w, w],
        xyxx: [x, y, x, x],
        xyxy: [x, y, x, y],
        xyxz: [x, y, x, z],
        xyxw: [x, y, x, w],
        xyyx: [x, y, y, x],
        xyyy: [x, y, y, y],
        xyyz: [x, y, y, z],
        xyyw: [x, y, y, w],
        xyzx: [x, y, z, x],
        xyzy: [x, y, z, y],
        xyzz: [x, y, z, z],
        xyzw: [x, y, z, w],
        xywx: [x, y, w, x],
        xywy: [x, y, w, y],
        xywz: [x, y, w, z],
        xyww: [x, y, w, w],
        xzxx: [x, z, x, x],
        xzxy: [x, z, x, y],
        xzxz: [x, z, x, z],
        xzxw: [x, z, x, w],
        xzyx: [x, z, y, x],
        xzyy: [x, z, y, y],
        xzyz: [x, z, y, z],
        xzyw: [x, z, y, w],
        xzzx: [x, z, z, x],
        xzzy: [x, z, z, y],
        xzzz: [x, z, z, z],
        xzzw: [x, z, z, w],
        xzwx: [x, z, w, x],
        xzwy: [x, z, w, y],
        xzwz: [x, z, w, z],
        xzww: [x, z, w, w],
        xwxx: [x, w, x, x],
        xwxy: [x, w, x, y],
        xwxz: [x, w, x, z],
        xwxw: [x, w, x, w],
        xwyx: [x, w, y, x],
        xwyy: [x, w, y, y],
        xwyz: [x, w, y, z],
        xwyw: [x, w, y, w],
        xwzx: [x, w, z, x],
        xwzy: [x, w, z, y],
        xwzz: [x, w, z, z],
        xwzw: [x, w, z, w],
        xwwx: [x, w, w, x],
        xwwy: [x, w, w, y],
        xwwz: [x, w, w, z],
        xwww: [x, w, w, w],
        yxxx: [y, x, x, x],
        yxxy: [y, x, x, y],
        yxxz: [y, x, x, z],
        yxxw: [y, x, x, w],
        yxyx: [y, x, y, x],
        yxyy: [y, x, y, y],
        yxyz: [y, x, y, z],
        yxyw: [y, x, y, w],
        yxzx: [y, x, z, x],
        yxzy: [y, x, z, y],
        yxzz: [y, x, z, z],
        yxzw: [y, x, z, w],
        yxwx: [y, x, w, x],
        yxwy: [y, x, w, y],
        yxwz: [y, x, w, z],
        yxww: [y, x, w, w],
        yyxx: [y, y, x, x],
        yyxy: [y, y, x, y],
        yyxz: [y, y, x, z],
        yyxw: [y, y, x, w],
        yyyx: [y, y, y, x],
        yyyy: [y, y, y, y],
        yyyz: [y, y, y, z],
        yyyw: [y, y, y, w],
        yyzx: [y, y, z, x],
        yyzy: [y, y, z, y],
        yyzz: [y, y, z, z],
        yyzw: [y, y, z, w],
        yywx: [y, y, w, x],
        yywy: [y, y, w, y],
        yywz: [y, y, w, z],
        yyww: [y, y, w, w],
        yzxx: [y, z, x, x],
        yzxy: [y, z, x, y],
        yzxz: [y, z, x, z],
        yzxw: [y, z, x, w],
        yzyx: [y, z, y, x],
        yzyy: [y, z, y, y],
        yzyz: [y, z, y, z],
        yzyw: [y, z, y, w],
        yzzx: [y, z, z, x],
        yzzy: [y, z, z, y],
        yzzz: [y, z, z, z],
        yzzw: [y, z, z, w],
        yzwx: [y, z, w, x],
        yzwy: [y, z, w, y],
        yzwz: [y, z, w, z],
        yzww: [y, z, w, w],
        ywxx: [y, w, x, x],
        ywxy: [y, w, x, y],
        ywxz: [y, w, x, z],
        ywxw: [y, w, x, w],
        ywyx: [y, w, y, x],
        ywyy: [y, w, y, y],
        ywyz: [y, w, y, z],
        ywyw: [y, w, y, w],
        ywzx: [y, w, z, x],
        ywzy: [y, w, z, y],
        ywzz: [y, w, z, z],
        ywzw: [y, w, z, w],
        ywwx: [y, w, w, x],
        ywwy: [y, w, w, y],
        ywwz: [y, w, w, z],
        ywww: [y, w, w, w],
        zxxx: [z, x, x, x],
        zxxy: [z, x, x, y],
        zxxz: [z, x, x, z],
        zxxw: [z, x, x, w],
        zxyx: [z, x, y, x],
        zxyy: [z, x, y, y],
        zxyz: [z, x, y, z],
        zxyw: [z, x, y, w],
        zxzx: [z, x, z, x],
        zxzy: [z, x, z, y],
        zxzz: [z, x, z, z],
        zxzw: [z, x, z, w],
        zxwx: [z, x, w, x],
        zxwy: [z, x, w, y],
        zxwz: [z, x, w, z],
        zxww: [z, x, w, w],
        zyxx: [z, y, x, x],
        zyxy: [z, y, x, y],
        zyxz: [z, y, x, z],
        zyxw: [z, y, x, w],
        zyyx: [z, y, y, x],
        zyyy: [z, y, y, y],
        zyyz: [z, y, y, z],
        zyyw: [z, y, y, w],
        zyzx: [z, y, z, x],
        zyzy: [z, y, z, y],
        zyzz: [z, y, z, z],
        zyzw: [z, y, z, w],
        zywx: [z, y, w, x],
        zywy: [z, y, w, y],
        zywz: [z, y, w, z],
        zyww: [z, y, w, w],
        zzxx: [z, z, x, x],
        zzxy: [z, z, x, y],
        zzxz: [z, z, x, z],
        zzxw: [z, z, x, w],
        zzyx: [z, z, y, x],
        zzyy: [z, z, y, y],
        zzyz: [z, z, y, z],
        zzyw: [z, z, y, w],
        zzzx: [z, z, z, x],
        zzzy: [z, z, z, y],
        zzzz: [z, z, z, z],
        zzzw: [z, z, z, w],
        zzwx: [z, z, w, x],
        zzwy: [z, z, w, y],
        zzwz: [z, z, w, z],
        zzww: [z, z, w, w],
        zwxx: [z, w, x, x],
        zwxy: [z, w, x, y],
        zwxz: [z, w, x, z],
        zwxw: [z, w, x, w],
        zwyx: [z, w, y, x],
        zwyy: [z, w, y, y],
        zwyz: [z, w, y, z],
        zwyw: [z, w, y, w],
        zwzx: [z, w, z, x],
        zwzy: [z, w, z, y],
        zwzz: [z, w, z, z],
        zwzw: [z, w, z, w],
        zwwx: [z, w, w, x],
        zwwy: [z, w, w, y],
        zwwz: [z, w, w, z],
        zwww: [z, w, w, w],
        wxxx: [w, x, x, x],
        wxxy: [w, x, x, y],
        wxxz: [w, x, x, z],
        wxxw: [w, x, x, w],
        wxyx: [w, x, y, x],
        wxyy: [w, x, y, y],
        wxyz: [w, x, y, z],
        wxyw: [w, x, y, w],
        wxzx: [w, x, z, x],
        wxzy: [w, x, z, y],
        wxzz: [w, x, z, z],
        wxzw: [w, x, z, w],
        wxwx: [w, x, w, x],
        wxwy: [w, x, w, y],
        wxwz: [w, x, w, z],
        wxww: [w, x, w, w],
        wyxx: [w, y, x, x],
        wyxy: [w, y, x, y],
        wyxz: [w, y, x, z],
        wyxw: [w, y, x, w],
        wyyx: [w, y, y, x],
        wyyy: [w, y, y, y],
        wyyz: [w, y, y, z],
        wyyw: [w, y, y, w],
        wyzx: [w, y, z, x],
        wyzy: [w, y, z, y],
        wyzz: [w, y, z, z],
        wyzw: [w, y, z, w],
        wywx: [w, y, w, x],
        wywy: [w, y, w, y],
        wywz: [w, y, w, z],
        wyww: [w, y, w, w],
        wzxx: [w, z, x, x],
        wzxy: [w, z, x, y],
        wzxz: [w, z, x, z],
        wzxw: [w, z, x, w],
        wzyx: [w, z, y, x],
        wzyy: [w, z, y, y],
        wzyz: [w, z, y, z],
        wzyw: [w, z, y, w],
        wzzx: [w, z, z, x],
        wzzy: [w, z, z, y],
        wzzz: [w, z, z, z],
        wzzw: [w, z, z, w],
        wzwx: [w, z, w, x],
        wzwy: [w, z, w, y],
        wzwz: [w, z, w, z],
        wzww: [w, z, w, w],
        wwxx: [w, w, x, x],
        wwxy: [w, w, x, y],
        wwxz: [w, w, x, z],
        wwxw: [w, w, x, w],
        wwyx: [w, w, y, x],
        wwyy: [w, w, y, y],
        wwyz: [w, w, y, z],
        wwyw: [w, w, y, w],
        wwzx: [w, w, z, x],
        wwzy: [w, w, z, y],
        wwzz: [w, w, z, z],
        wwzw: [w, w, z, w],
        wwwx: [w, w, w, x],
        wwwy: [w, w, w, y],
        wwwz: [w, w, w, z],
        wwww: [w, w, w, w],
    }

    declare_with_swizzle_fns! {
        with_x: ([x], [set_x]),
        with_y: ([y], [set_y]),
        with_z: ([z], [set_z]),
        with_w: ([w], [set_w]),
    }

    declare_with_swizzle_fns! {
        with_xy: ([x, y], [set_x, set_y]),
        with_xz: ([x, z], [set_x, set_z]),
        with_xw: ([x, w], [set_x, set_w]),
        with_yx: ([y, x], [set_y, set_x]),
        with_yz: ([y, z], [set_y, set_z]),
        with_yw: ([y, w], [set_y, set_w]),
        with_zx: ([z, x], [set_z, set_x]),
        with_zy: ([z, y], [set_z, set_y]),
        with_zw: ([z, w], [set_z, set_w]),
        with_wx: ([w, x], [set_w, set_x]),
        with_wy: ([w, y], [set_w, set_y]),
        with_wz: ([w, z], [set_w, set_z]),
    }

    declare_with_swizzle_fns! {
        with_xyz: ([x, y, z], [set_x, set_y, set_z]),
        with_xyw: ([x, y, w], [set_x, set_y, set_w]),
        with_xzy: ([x, z, y], [set_x, set_z, set_y]),
        with_xzw: ([x, z, w], [set_x, set_z, set_w]),
        with_xwy: ([x, w, y], [set_x, set_w, set_y]),
        with_xwz: ([x, w, z], [set_x, set_w, set_z]),
        with_yxz: ([y, x, z], [set_y, set_x, set_z]),
        with_yxw: ([y, x, w], [set_y, set_x, set_w]),
        with_yzx: ([y, z, x], [set_y, set_z, set_x]),
        with_yzw: ([y, z, w], [set_y, set_z, set_w]),
        with_ywx: ([y, w, x], [set_y, set_w, set_x]),
        with_ywz: ([y, w, z], [set_y, set_w, set_z]),
        with_zxy: ([z, x, y], [set_z, set_x, set_y]),
        with_zxw: ([z, x, w], [set_z, set_x, set_w]),
        with_zyx: ([z, y, x], [set_z, set_y, set_x]),
        with_zyw: ([z, y, w], [set_z, set_y, set_w]),
        with_zwx: ([z, w, x], [set_z, set_w, set_x]),
        with_zwy: ([z, w, y], [set_z, set_w, set_y]),
        with_wxy: ([w, x, y], [set_w, set_x, set_y]),
        with_wxz: ([w, x, z], [set_w, set_x, set_z]),
        with_wyx: ([w, y, x], [set_w, set_y, set_x]),
        with_wyz: ([w, y, z], [set_w, set_y, set_z]),
        with_wzx: ([w, z, x], [set_w, set_z, set_x]),
        with_wzy: ([w, z, y], [set_w, set_z, set_y]),
    }

    declare_with_swizzle_fns! {
        with_xyzw: ([x, y, z, w], [set_x, set_y, set_z, set_w]),
        with_xywz: ([x, y, w, z], [set_x, set_y, set_w, set_z]),
        with_xzyw: ([x, z, y, w], [set_x, set_z, set_y, set_w]),
        with_xzwy: ([x, z, w, y], [set_x, set_z, set_w, set_y]),
        with_xwyz: ([x, w, y, z], [set_x, set_w, set_y, set_z]),
        with_xwzy: ([x, w, z, y], [set_x, set_w, set_z, set_y]),
        with_yxzw: ([y, x, z, w], [set_y, set_x, set_z, set_w]),
        with_yxwz: ([y, x, w, z], [set_y, set_x, set_w, set_z]),
        with_yzxw: ([y, z, x, w], [set_y, set_z, set_x, set_w]),
        with_yzwx: ([y, z, w, x], [set_y, set_z, set_w, set_x]),
        with_ywxz: ([y, w, x, z], [set_y, set_w, set_x, set_z]),
        with_ywzx: ([y, w, z, x], [set_y, set_w, set_z, set_x]),
        with_zxyw: ([z, x, y, w], [set_z, set_x, set_y, set_w]),
        with_zxwy: ([z, x, w, y], [set_z, set_x, set_w, set_y]),
        with_zyxw: ([z, y, x, w], [set_z, set_y, set_x, set_w]),
        with_zywx: ([z, y, w, x], [set_z, set_y, set_w, set_x]),
        with_zwxy: ([z, w, x, y], [set_z, set_w, set_x, set_y]),
        with_zwyx: ([z, w, y, x], [set_z, set_w, set_y, set_x]),
        with_wxyz: ([w, x, y, z], [set_w, set_x, set_y, set_z]),
        with_wxzy: ([w, x, z, y], [set_w, set_x, set_z, set_y]),
        with_wyxz: ([w, y, x, z], [set_w, set_y, set_x, set_z]),
        with_wyzx: ([w, y, z, x], [set_w, set_y, set_z, set_x]),
        with_wzxy: ([w, z, x, y], [set_w, set_z, set_x, set_y]),
        with_wzyx: ([w, z, y, x], [set_w, set_z, set_y, set_x]),
    }
}

macro_rules! declare_swizzle_fns {
    ($($f:ident: [$x:ident, $y:ident]),*$(,)?) => {$(
        #[doc = concat!(
            "Returns `(self.",
            stringify!($x),
            ", self.",
            stringify!($y),
            ")`."
        )]
        #[inline]
        #[must_use]
        pub fn $f(self) -> Mask<2, T, A> {
            Mask::<2, T, A>::new(self.$x(), self.$y())
        }
    )*};

    ($($f:ident: [$x:ident, $y:ident, $z:ident]),*$(,)?) => {$(
        #[doc = concat!(
            "Returns `(self.",
            stringify!($x),
            ", self.",
            stringify!($y),
            ", self.",
            stringify!($z),
            ")`."
        )]
        #[inline]
        #[must_use]
        pub fn $f(self) -> Mask<3, T, A> {
            Mask::<3, T, A>::new(self.$x(), self.$y(), self.$z())
        }
    )*};

    ($($f:ident: [$x:ident, $y:ident, $z:ident, $w:ident]),*$(,)?) => {$(
        #[doc = concat!(
            "Returns `(self.",
            stringify!($x),
            ", self.",
            stringify!($y),
            ", self.",
            stringify!($z),
            ", self.",
            stringify!($w),
            ")`."
        )]
        #[inline]
        #[must_use]
        pub fn $f(self) -> Mask<4, T, A> {
            Mask::<4, T, A>::new(self.$x(), self.$y(), self.$z(), self.$w())
        }
    )*};
}

use declare_swizzle_fns;

macro_rules! declare_with_swizzle_fns {
    ($($f:ident: ([$x:ident], [$set_x:ident])),*$(,)?) => {$(
        #[doc = concat!(
            "Returns the mask with the `",
            stringify!($x),
            "` component set to the given value.",
        )]
        #[inline]
        #[must_use]
        pub fn $f(mut self, value: bool) -> Self {
            self.$set_x(value);
            self
        }
    )*};

    ($($f:ident: ([$x:ident, $y:ident], [$set_x:ident, $set_y:ident])),*$(,)?) => {$(
        #[doc = concat!(
            "Returns the mask with the `",
            stringify!($x),
            "` and `",
            stringify!($y),
            "` components set to the given values.",
        )]
        #[inline]
        #[must_use]
        pub fn $f(mut self, value: Mask<2, T, A>) -> Self {
            self.$set_x(value.x());
            self.$set_y(value.y());
            self
        }
    )*};

    ($(
        $f:ident: (
            [$x:ident, $y:ident, $z:ident],
            [$set_x:ident, $set_y:ident, $set_z:ident]
        )
    ),*$(,)?) => {$(
        #[doc = concat!(
            "Returns the mask with the `",
            stringify!($x),
            "`, `",
            stringify!($y),
            "` and `",
            stringify!($z),
            "` components set to the given values.",
        )]
        #[inline]
        #[must_use]
        pub fn $f(mut self, value: Mask<3, T, A>) -> Self {
            self.$set_x(value.x());
            self.$set_y(value.y());
            self.$set_z(value.z());
            self
        }
    )*};

    ($(
        $f:ident: (
            [$x:ident, $y:ident, $z:ident, $w:ident],
            [$set_x:ident, $set_y:ident, $set_z:ident, $set_w:ident]
        )
    ),*$(,)?) => {$(
        #[doc = concat!(
            "Returns the mask with the `",
            stringify!($x),
            "`, `",
            stringify!($y),
            "`, `",
            stringify!($z),
            "` and `",
            stringify!($w),
            "` components set to the given values.",
        )]
        #[inline]
        #[must_use]
        pub fn $f(mut self, value: Mask<4, T, A>) -> Self {
            self.$set_x(value.x());
            self.$set_y(value.y());
            self.$set_z(value.z());
            self.$set_w(value.w());
            self
        }
    )*};
}

use declare_with_swizzle_fns;
