use day11::{p1, p2, uncle_scientist};

static INPUT : &str = include_str!(
    "../input.txt",
);

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    p1(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    p2(divan::black_box(INPUT));
}

#[divan::bench]
fn part1_uncle_scientist() {
    uncle_scientist::p1(divan::black_box(INPUT));
}

#[divan::bench]
fn part2_uncle_scientist() {
    uncle_scientist::p2(divan::black_box(INPUT));
}
