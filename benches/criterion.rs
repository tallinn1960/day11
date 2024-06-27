// a criterion benchmark for p2, p2_reverse, and p2_maps

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use day11::{p1, p2, uncle_scientist};
use std::fs::File;
use std::io::Read;

fn bench_p1(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part1", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| p1(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

fn bench_p1_uncle_scientist(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part1_uncle_scientist", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| uncle_scientist::p1(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish();
}

fn bench_p2(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part2", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| p2(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish()
}

fn bench_p2_uncle_scientist(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part2_uncle_scientist", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| uncle_scientist::p2(&f),
            BatchSize::SmallInput,
        )
    });
    g.finish()
}

#[cfg(feature="Swift")]
fn bench_p1_swift(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part1_swift", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |mut f| day11_swift::part1_swift(&mut f),
            BatchSize::SmallInput,
        )
    });
    g.finish()
}

fn bench_p1_cpp(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part1_cpp", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| unsafe {day11_cpp::part1_cpp(f.as_ptr(), f.len())},
            BatchSize::SmallInput,
        )
    });
    g.finish()
}

fn bench_p2_cpp(c: &mut Criterion) {
    let mut g = c.benchmark_group("criterion");
    g.bench_function("part2_cpp", |b| {
        b.iter_batched(
            || {
                let mut f = File::open("input.txt").expect("can't open file");
                let mut buf = String::new();
                f.read_to_string(&mut buf).expect("can't read file");
                buf
            },
            |f| unsafe {day11_cpp::part2_cpp(f.as_ptr(), f.len())},
            BatchSize::SmallInput,
        )
    });
    g.finish()
}

#[cfg(not(feature="Swift"))]
criterion_group!(benches, bench_p1, bench_p1_cpp, bench_p1_uncle_scientist, bench_p2, bench_p2_cpp, bench_p2_uncle_scientist);
#[cfg(feature="Swift")]
criterion_group!(benches, bench_p1, bench_p1_cpp, bench_p1_uncle_scientist, bench_p1_swift, bench_p2, bench_p2_cpp, bench_p2_uncle_scientist);

criterion_main!(benches);
