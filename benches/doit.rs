use bench_lea_mov_array::*;
// use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use rand::Rng;

criterion_main!(benches);

criterion_group!(benches, bench_medium_repeat,);

fn bench_medium_repeat(c: &mut Criterion) {
    // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
    let r = black_box(|| 0_usize.wrapping_mul(1664525).wrapping_add(1013904223));
    let r = r();

    // let mut rng = rand::thread_rng();
    type IndexFn = dyn Fn(usize) -> u8;
    let sizes_and_fns: [(usize, &IndexFn, &IndexFn); 4] = [
        (32, &index_array_random_stack_32, &index_array_random_rodata_32),
        (64, &index_array_random_stack_64, &index_array_random_rodata_64),
        (128, &index_array_random_stack_128, &index_array_random_rodata_128),
        (256, &index_array_random_stack_256, &index_array_random_rodata_256),
    ];

    let mut group = c.benchmark_group("medium_array");

    for (size, on_stack, on_rodata) in sizes_and_fns {
        let x = r % size;
        group.bench_function(format!("stack_{size}"), |b| b.iter(|| on_stack(x)));
        group.bench_function(format!("rodata_{size}"), |b| b.iter(|| on_rodata(x)));
    }

    group.finish()
}
