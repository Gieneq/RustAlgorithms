use std::{hint::black_box, time::Duration};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use rust_algorithms::{solver_bf, solver_bt};

const INPUTS: &[&str] = &[
    ("I + BB == ILL"),
    ("A == B"),
    ("ACA + DD == BD"),
    ("A + A + A + A + A + A + A + A + A + A + A + B == BCC"),
    ("AS + A == MOM"),
    ("NO + NO + TOO == LATE"),
    ("HE + SEES + THE == LIGHT"),
    ("SEND + MORE == MONEY"),
    ("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE"),
];

fn solvers_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Solvers 6 letters comparison");
    let loops: usize = 1;

    group.bench_with_input(BenchmarkId::new("Bruteforce", loops), &loops, |b, &loops| {
        b.iter(|| {
            for _ in 0..loops {
                for input in INPUTS {
                    _ = solver_bf::solve(black_box(input));
                }
            }
        });
    });

    group.bench_with_input(BenchmarkId::new("Backtracking", loops), &loops, |b, &loops| {
        b.iter(|| {
            for _ in 0..loops {
                for input in INPUTS {
                    _ = solver_bt::solve(black_box(input));
                }
            }
        });
    });
}

fn configure_criterion() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::new(300, 0))
        .sample_size(100)
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = solvers_benchmark
);
criterion_main!(benches);