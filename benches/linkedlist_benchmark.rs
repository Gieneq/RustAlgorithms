use std::{hint::black_box, time::Duration};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use rust_algorithms::linkedlist::LinkedList;

fn linkedlist_push_back_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Linkedlist 'push_back' comparison");
    let loops: i32 = 100;

    group.bench_with_input(BenchmarkId::new("My LinkedList", loops), &loops, |b, &loops| {
        b.iter(|| {
            let mut llist = LinkedList::<i32>::new();
            for i in 0..loops {
                llist.push_back(black_box(i));
            }
        });
    });

    group.bench_with_input(BenchmarkId::new("STD LinkedList", loops), &loops, |b, &loops| {
        b.iter(|| {
            let mut llist = std::collections::LinkedList::<i32>::new();
            for i in 0..loops {
                llist.push_back(black_box(i));
            }
        });
    });
}

fn configure_criterion() -> Criterion {
    Criterion::default().measurement_time(Duration::new(7, 0))
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = linkedlist_push_back_benchmark
);
criterion_main!(benches);