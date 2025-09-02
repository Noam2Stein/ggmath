mod float;

pub fn bench_cmp(slow_fns: &mut Vec<String>) {
    float::bench_cmp(slow_fns);
}
