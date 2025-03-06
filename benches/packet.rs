use criterion::*;
use criterion::measurement::WallTime;
use rsbuf::packet::Packet;

fn bench_pbit(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);
    packet.bits();

    group.bench_function("pbit", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.bit_pos = 0;
                for _ in 0..45714 {
                    // result: 201 147 38 76 153 50 100 over and over
                    packet.pbit(7, 100);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_gbit(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);
    packet.bits();

    for _ in 0..45714 {
        packet.pbit(7, 100);
    }

    group.bench_function("gbit", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.bit_pos = 0;
                for _ in 0..45714 {
                    packet.gbit(7);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_p1(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("p1", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..40000 {
                    packet.p1(255);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_g1(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..40000 {
        packet.p1(255);
    }

    group.bench_function("g1", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..40000 {
                    packet.g1();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_g1s(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..40000 {
        packet.p1(255);
    }

    group.bench_function("g1s", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..40000 {
                    packet.g1s();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_p2(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("p2", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..20000 {
                    packet.p2(65535);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_g2(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..20000 {
        packet.p2(65535);
    }

    group.bench_function("g2", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..20000 {
                    packet.g2();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_g2s(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..20000 {
        packet.p2(65535);
    }

    group.bench_function("g2s", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..20000 {
                    packet.g2s();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_p3(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("p3", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..13333 {
                    packet.p3(16777215);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_g3(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..13333 {
        packet.p3(16777215);
    }

    group.bench_function("g3", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..13333 {
                    packet.g3();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_p4(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("p4", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..10000 {
                    packet.p4(i32::MAX);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_g4s(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..10000 {
        packet.p4(i32::MAX);
    }

    group.bench_function("g4s", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..10000 {
                    packet.g4s();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_p8(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("p8", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..5000 {
                    packet.p8(i64::MAX);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_g8s(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..5000 {
        packet.p8(i64::MAX);
    }

    group.bench_function("g8s", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..5000 {
                    packet.g8s();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_pjstr(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("pjstr", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..8000 {
                    packet.pjstr("test", 0);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_gjstr(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..8000 {
        packet.pjstr("test", 0);
    }

    group.bench_function("gjstr", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..8000 {
                    packet.gjstr(0);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_psmart_1(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("psmart_1", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..40000 {
                    packet.psmart(69)
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_gsmart_1(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..40000 {
        packet.psmart(69);
    }

    group.bench_function("gsmart_1", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..40000 {
                    packet.gsmart();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_psmart_2(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("psmart_2", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..20000 {
                    packet.psmart(420)
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_gsmart_2(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..20000 {
        packet.psmart(420);
    }

    group.bench_function("gsmart_2", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..20000 {
                    packet.gsmart();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_psmarts_1(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("psmarts_1", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..40000 {
                    packet.psmarts(33)
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_gsmarts_1(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..40000 {
        packet.psmarts(33);
    }

    group.bench_function("gsmarts_1", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..40000 {
                    packet.gsmarts();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_psmarts_2(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("psmarts_2", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..20000 {
                    packet.psmarts(420)
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_gsmarts_2(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..20000 {
        packet.psmarts(420);
    }

    group.bench_function("gsmarts_2", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..20000 {
                    packet.gsmarts();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_pdata(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let packet: Packet = Packet::new(40000);

    group.bench_function("pdata", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..8000 {
                    packet.pdata(&vec![69, 59, 49, 39, 29], 0, 5);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_gdata(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("packet");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let mut packet: Packet = Packet::new(40000);

    for _ in 0..8000 {
        packet.pdata(&vec![69, 59, 49, 39, 29], 0, 5);
    }

    group.bench_function("gdata", move |b| {
        b.iter_batched(
            || packet.clone(),
            |mut packet| {
                packet.pos = 0;
                for _ in 0..8000 {
                    packet.gdata(&mut vec![0u8; 5], 0, 5);
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_pbit,
    bench_gbit,
    bench_p1,
    bench_g1,
    bench_g1s,
    bench_p2,
    bench_g2,
    bench_g2s,
    bench_p3,
    bench_g3,
    bench_p4,
    bench_g4s,
    bench_p8,
    bench_g8s,
    bench_pjstr,
    bench_gjstr,
    bench_psmart_1,
    bench_gsmart_1,
    bench_psmart_2,
    bench_gsmart_2,
    bench_psmarts_1,
    bench_gsmarts_1,
    bench_psmarts_2,
    bench_gsmarts_2,
    bench_pdata,
    bench_gdata
);

criterion_main!(benches);