use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;
use rusty_join::versions::*;

//Uncomment for default
// fn bench_impls_smalldataset(c: &mut Criterion) {

//Uncomment for Cyclecount
// see below
fn bench_impls_smalldataset(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("Joining");
    let input_args = vec![
        "dumy".to_string(),
        "data/f1.csv".to_string(),
        "data/f2.csv".to_string(),
        "data/f3.csv".to_string(),
        "data/f4.csv".to_string(),
    ];
    
    group.bench_with_input(
        BenchmarkId::new("reduced_hash_v4", "Small"),
        &input_args,
        |b, inpt_args| b.iter(|| reduced_hash_v4(inpt_args.clone())),
    );
    group.finish()
}
//Default with time
// criterion_group!(benches, bench_impls_smalldataset);
// criterion_main!(benches);

// Cycle counts using other compatible library
// To use uncomment c:&mut Criterion<CyclesPerByte>
// see: https://github.com/wainwrightmark/criterion-cycles-per-byte
criterion_group!(
    name = my_bench;
    config = Criterion::default().sample_size(10).with_measurement(CyclesPerByte);
    targets = bench_impls_smalldataset
);
criterion_main!(my_bench);
