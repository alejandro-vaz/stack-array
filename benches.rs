//^
//^ HEAD
//^

//> HEAD -> LINTS
#![allow(incomplete_features)]

//> HEAD -> FEATURES
#![feature(generic_const_exprs)]

//> HEAD -> IMPORTS
use {
    stack_array::Array,
    core::hint::black_box,
    criterion::{
        Criterion,
        criterion_group,
        criterion_main,
        Throughput,
        BenchmarkGroup,
        measurement::WallTime
    },
    arrayvec::ArrayVec,
    smallvec::SmallVec
};


//^
//^ BENCHES
//^

//> BENCHES -> SETUP
criterion_group!(stack_array, benches);
criterion_main!(stack_array);

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    push(criterion.benchmark_group("push"));
    pushpop(criterion.benchmark_group("pushpop"));
}

//> BENCHES -> PUSH
fn push(mut group: BenchmarkGroup<'_, WallTime>) -> () {
    const SIZE: usize = 2usize.pow(16);
    group.throughput(Throughput::Bytes(SIZE as u64 * 8));
    group.bench_function("array", |bencher| {
        let mut array = Array::<usize, SIZE>::default();
        bencher.iter(|| {
            for index in 0..SIZE {
                array.push(black_box(index));
            }
            array.clear();
        }
    )});
    group.bench_function("arrayvec", |bencher| {
        let mut arrayvec = ArrayVec::<usize, SIZE>::default();
        bencher.iter(|| {
            for index in 0..SIZE {
                arrayvec.push(black_box(index));
            }
            arrayvec.clear();
        }
    )});
    group.bench_function("smallvec", |bencher| {
        let mut smallvec = SmallVec::<[usize; SIZE]>::default();
        bencher.iter(|| {
            for index in 0..SIZE {
                smallvec.push(black_box(index));
            }
            smallvec.clear();
        }
    )});
    group.bench_function("vec", |bencher| {
        let mut vec = Vec::<usize>::with_capacity(SIZE);
        bencher.iter(|| {
            for index in 0..SIZE {
                vec.push(black_box(index));
            }
            vec.clear();
        }
    )});
}

//> BENCHES -> PUSHPOP
fn pushpop(mut group: BenchmarkGroup<'_, WallTime>) {
    const SIZE: usize = 2usize.pow(16);
    group.throughput(Throughput::Bytes(SIZE as u64 * 8));
    group.bench_function("array", |bencher| {
        let mut array = Array::<usize, 1>::default();
        bencher.iter(|| {
            for index in 0..SIZE {
                array.push(black_box(index));
                black_box(array.pop());
            }
            array.clear();
        }
    )});
    group.bench_function("arrayvec", |bencher| {
        let mut arrayvec = ArrayVec::<usize, 1>::default();
        bencher.iter(|| {
            for index in 0..SIZE {
                arrayvec.push(black_box(index));
                black_box(arrayvec.pop());
            }
            arrayvec.clear();
        }
    )});
    group.bench_function("smallvec", |bencher| {
        let mut smallvec = SmallVec::<[usize; 1]>::default();
        bencher.iter(|| {
            for index in 0..SIZE {
                smallvec.push(black_box(index));
                black_box(smallvec.pop());
            }
            smallvec.clear();
        }
    )});
    group.bench_function("vec", |bencher| {
        let mut vec = Vec::<usize>::with_capacity(1);
        bencher.iter(|| {
            for index in 0..SIZE {
                vec.push(black_box(index));
                black_box(vec.pop());
            }
            vec.clear();
        }
    )});
}