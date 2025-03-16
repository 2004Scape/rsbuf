use std::collections::HashSet;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkGroup, Criterion, Throughput};
use criterion::measurement::WallTime;
use rsbuf::build::IdBitSet;

fn bench_bitset_contains(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("build");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut players: IdBitSet = IdBitSet::new(2048, 250);

    for index in 0..2048 {
        players.insert(index);
    }

    group.bench_function("bitset contains", move |b| {
        b.iter_batched(
            || players.clone(),
            |collection| {
                for _ in 0..8000 {
                    collection.contains(2047);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_hashset_contains(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("build");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut players: HashSet<i32> = HashSet::with_capacity(2048);

    for index in 0..2048 {
        players.insert(index);
    }

    group.bench_function("hashset contains", move |b| {
        b.iter_batched(
            || players.clone(),
            |collection| {
                for _ in 0..8000 {
                    collection.contains(&2047);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_bitset_insert(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("build");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let players: IdBitSet = IdBitSet::new(2048, 250);

    group.bench_function("bitset insert", move |b| {
        b.iter_batched(
            || players.clone(),
            |mut collection| {
                for index in 0..2048 {
                    collection.insert(index);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_hashset_insert(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("build");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let players: HashSet<i32> = HashSet::with_capacity(2048);

    group.bench_function("hashset insert", move |b| {
        b.iter_batched(
            || players.clone(),
            |mut collection| {
                for index in 0..2048 {
                    collection.insert(index);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_bitset_remove(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("build");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut players: IdBitSet = IdBitSet::new(2048, 250);

    for index in 0..2048 {
        players.insert(index);
    }

    group.bench_function("bitset remove", move |b| {
        b.iter_batched(
            || players.clone(),
            |mut collection| {
                for index in 0..2048 {
                    collection.remove(index);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_hashset_remove(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("build");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut players: HashSet<i32> = HashSet::with_capacity(2048);

    for index in 0..2048 {
        players.insert(index);
    }

    group.bench_function("hashset remove", move |b| {
        b.iter_batched(
            || players.clone(),
            |mut collection| {
                for index in 0..2048 {
                    collection.remove(&index);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_bitset_contains,
    bench_hashset_contains,
    bench_bitset_insert,
    bench_hashset_insert,
    bench_bitset_remove,
    bench_hashset_remove,
);

criterion_main!(benches);