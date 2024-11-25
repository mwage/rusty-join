use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rusty_join::{all_hash, quintuple_sort, split_no_encode};

fn bench_impls_smalldataset(c: &mut Criterion) {
    let mut group = c.benchmark_group("Joining");
    let input_args = vec![
        "dumy".to_string(),
        "data/a.csv".to_string(),
        "data/b.csv".to_string(),
        "data/c.csv".to_string(),
        "data/d.csv".to_string(),
    ];
    group.bench_with_input(
        BenchmarkId::new("SplitNoEncode", "Small"),
        &input_args,
        |b, inpt_args| b.iter(|| split_no_encode(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("AllHash", "Small"),
        &input_args,
        |b, inpt_args| b.iter(|| all_hash(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("QuintupleSort", "Small"),
        &input_args,
        |b, inpt_args| b.iter(|| quintuple_sort(inpt_args.clone())),
    );
    group.bench_with_input(
        BenchmarkId::new("SplitDuringRead", "Small"),
        &input_args,
        |b, inpt_args| b.iter(|| quintuple_sort(inpt_args.clone())),
    );
}

criterion_group!(benches, bench_impls_smalldataset);
criterion_main!(benches);
