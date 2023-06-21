use blaseball_vcr::db_manager::*;
use blaseball_vcr::*;
use criterion::{criterion_group, criterion_main, Criterion};
use vcr_schemas::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut db_manager = DatabaseManager::new();
    for entry in std::fs::read_dir("../vhs_tapes").unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            let stem = path.file_stem().unwrap().to_string_lossy().to_owned();
            println!("-> loading {}", stem);
            call_method_by_type!(
                db_wrapper::from_single_and_insert,
                (&mut db_manager, &entry.path()),
                stem.as_ref(),
                { continue }
            )
            .unwrap();
        }
    }

    let mut group = c.benchmark_group("stream data");

    group.bench_function("single threaded", |b| {
        b.iter(|| {
            for i in 0..10 {
                blaseball_vcr::stream_data::stream_data(&db_manager, 1600113866 + i * 4);
            }
        })
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(200).measurement_time(std::time::Duration::from_secs(40)).warm_up_time(std::time::Duration::from_secs(5));
    targets = criterion_benchmark
}
criterion_main!(benches);
