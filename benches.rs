//^
//^ HEAD
//^

//> HEAD -> LINTS
#![allow(incomplete_features)]

//> HEAD -> FEATURES
#![feature(generic_const_exprs)]

//> HEAD -> SUPER
use stack_array::Array;

//> HEAD -> CORE
use core::hint::black_box;

//> HEAD -> CRITERION
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
    Throughput
};

//> HEAD -> ARRAYVEC
use arrayvec::ArrayVec;


//^
//^ BENCHES
//^

//> BENCHES -> SETUP
criterion_group!(stack_array, benches);
criterion_main!(stack_array);

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("stack-array");
    const ITERATIONS: usize = 10000;
    group.throughput(Throughput::Elements(ITERATIONS as u64));
    let mut array = Array::<u8, 10>::from([1, 2, 3]);
    group.bench_function("pushpop", |bencher| bencher.iter(|| for _ in 0..ITERATIONS {
        array.push(black_box(0));
        let x = black_box(array.pop());
        black_box(x);
    }));
    let mut vector = Vec::with_capacity(10);
    vector.push(1);
    vector.push(2);
    vector.push(3);
    group.bench_function("veccomparison", |bencher| bencher.iter(|| for _ in 0..ITERATIONS {
        vector.push(black_box(0));
        let x = black_box(vector.pop());
        black_box(x);
    }));
    let mut competitor = ArrayVec::<u8, 10>::new();
    competitor.push(1);
    competitor.push(2);
    competitor.push(3);
    group.bench_function("arrayvec", |bencher| bencher.iter(|| for _ in 0..ITERATIONS {
        competitor.push(black_box(0));
        let x = black_box(competitor.pop());
        black_box(x);
    }));
    group.bench_function("insertremove", |bencher| bencher.iter(|| for _ in 0..ITERATIONS {
        array.insert(black_box(0), black_box(0));
        let x = array.remove(black_box(0));
        black_box(x);
    }));
    group.bench_function("extendclear", |bencher| bencher.iter(|| for _ in 0..ITERATIONS {
        array.extend(black_box([1, 2, 3, 4, 5]));
        array.clear();
    }));
}