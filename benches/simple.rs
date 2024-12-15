use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rusty_join::{
    polars_join::{self, custom_polars},
    sample_program,
    versions::*,
};

fn bench_impls_smalldataset(c: &mut Criterion) {
    let mut group = c.benchmark_group("Joining");
    group.sample_size(10);
    let input_args = vec![
        "dumy".to_string(),
        "data/a.csv".to_string(),
        "data/b.csv".to_string(),
        "data/c.csv".to_string(),
        "data/d.csv".to_string(),
    ];
    group.bench_with_input(
        BenchmarkId::new("AntonErtlVersion", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| sample_program(inpt_args.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("baseline_v1", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| baseline_v1(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("baseline_v2", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| baseline_v2(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("baseline_v3", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| baseline_v3(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("baseline_v4", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| baseline_v4(inpt_args.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("sorting_v1", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| sorting_v1(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("sorting_v2", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| sorting_v1(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("sorting_v3", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| sorting_v3(inpt_args.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("hash_v1", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v1(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v2", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v2(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v3", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v3(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v4", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v4(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v5", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v5(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v6", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v6(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v7", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v7(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v8", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v8(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v9", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v9(inpt_args.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v1", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v1(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v2", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v2(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v3", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v3(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v4", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v4(inpt_args.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("parallel_hash", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| parallel_hash(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("parallel_hash_reduced", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| parallel_reduced_hash(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("polards", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| custom_polars(inpt_args.clone())),
    );

    group.finish()
}

fn bench_impls_large_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("ReadLarge");
    group.sample_size(10);
    let input_args = vec![
        "dumy".to_string(),
        "data/f1.csv".to_string(),
        "data/f2.csv".to_string(),
        "data/f3.csv".to_string(),
        "data/f4.csv".to_string(),
    ];

    group.bench_with_input(
        BenchmarkId::new("hash_v1_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v1_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v2_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v2_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v3_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v3_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v4_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v4_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v5_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v5_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v6_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v6_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v7_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v7_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v8_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v8_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v9_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v9_read(inpt_args.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v1_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v1_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v2_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v2_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v3_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v3_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v4_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v4_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("parallel_reduced_hash_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| parallel_reduced_hash_read(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("parallel_hash_read", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| parallel_hash_read(inpt_args.clone())),
    );
}

fn bench_impls_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("JoiningLarge");
    group.sample_size(10);
    let input_args = vec![
        "dumy".to_string(),
        "data/f1.csv".to_string(),
        "data/f2.csv".to_string(),
        "data/f3.csv".to_string(),
        "data/f4.csv".to_string(),
    ];
    group.bench_with_input(
        BenchmarkId::new("AntonErtlVersion", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| sample_program(inpt_args.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("hash_v1", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v1(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v2", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v2(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v3", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v3(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v4", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v4(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v5", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v5(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v6", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v6(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v7", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v7(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v8", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v8(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("hash_v9", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| hash_v9(inpt_args.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v1", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v1(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v2", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v2(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v3", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v3(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v4", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v4(inpt_args.clone())),
    );

    group.bench_with_input(
        BenchmarkId::new("parallel_hash", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| parallel_hash(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("parallel_hash_reduced", "LargeSet"),
        &input_args,
        |b, inpt_args| b.iter(|| parallel_reduced_hash(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("polards", "SmallSet"),
        &input_args,
        |b, inpt_args| b.iter(|| custom_polars(inpt_args.clone())),
    );

    group.finish()
}
//Default with time
criterion_group!(benches, bench_impls_large);
criterion_main!(benches);

// Cycle counts using other compatible library
// To use uncomment c:&mut Criterion<CyclesPerByte>
// see: https://github.com/wainwrightmark/criterion-cycles-per-byte
// criterion_group!(
//     name = my_bench;
//     config = Criterion::default().sample_size(10).with_measurement(CyclesPerByte);
//     targets = bench_impls_smalldataset
// );
// criterion_main!(my_bench);
