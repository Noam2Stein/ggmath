#[macro_export]
macro_rules! bench_unary_operator {
    (
        $name:ident { $op_punct:tt } for

        ggmath::$($ggmath_path:ident)::* { $ggmath_vec:expr },
        $vs:ident::$($vs_path:ident)::* { $vs_vec:expr }
    ) => {
        paste::paste! {
            #[library_benchmark]
            fn [<ggmath_$name>]() -> ggmath::$($ggmath_path)::* {
                #[allow(unused_imports)]
                use ggmath::*;
                bench_unary_operator!(@use_mod ggmath::$($ggmath_path)::*);

                black_box($op_punct black_box($ggmath_vec))
            }

            #[library_benchmark]
            fn [<$vs _$name>]() -> $vs::$($vs_path)::* {
                #[allow(unused_imports)]
                use $vs::*;
                bench_unary_operator!(@use_mod $vs::$($vs_path)::*);

                black_box($op_punct black_box($vs_vec))
            }

            library_benchmark_group!(name = $name; benchmarks = [<ggmath_$name>], [<$vs _$name>]);
        }
    };

    (@use_mod $ident0:ident::$_item:ident) => { #[allow(unused_imports)] use $ident0::*; };
    (@use_mod $ident0:ident::$idnet1:ident::$_item:ident) => { #[allow(unused_imports)] use $ident0::$idnet1::*; };
}

#[macro_export]
macro_rules! bench_binary_operator {
    (
        $name:ident { $op_punct:tt } for

        ggmath::$($ggmath_path:ident)::* { $ggmath_lhs:expr, $ggmath_rhs:expr },
        $vs:ident::$($vs_path:ident)::* { $vs_lhs:expr, $vs_rhs:expr }
    ) => {
        paste::paste! {
            #[library_benchmark]
            fn [<ggmath_$name>]() -> ggmath::$($ggmath_path)::* {
                #[allow(unused_imports)]
                use ggmath::*;
                bench_binary_operator!(@use_mod ggmath::$($ggmath_path)::*);

                black_box(black_box($ggmath_lhs) $op_punct black_box($ggmath_rhs))
            }

            #[library_benchmark]
            fn [<$vs _$name>]() -> $vs::$($vs_path)::* {
                #[allow(unused_imports)]
                use $vs::*;
                bench_binary_operator!(@use_mod $vs::$($vs_path)::*);

                black_box(black_box($vs_lhs) $op_punct black_box($vs_rhs))
            }

            library_benchmark_group!(name = $name; benchmarks = [<ggmath_$name>], [<$vs _$name>]);
        }
    };

    (@use_mod $ident0:ident::$_item:ident) => { #[allow(unused_imports)] use $ident0::*; };
    (@use_mod $ident0:ident::$idnet1:ident::$_item:ident) => { #[allow(unused_imports)] use $ident0::$idnet1::*; };
}
