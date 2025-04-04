use criterion::{criterion_group, criterion_main, Criterion};
use kmerr::kmer::build_kmer_table_parallel;

fn bench_kmer(c: &mut Criterion) {
    c.bench_function("parallel kmer count", |b| {
        b.iter(|| {
            build_kmer_table_parallel("test_data/expected.fasta", 15);
        });
    });
}

criterion_group!(benches, bench_kmer);
criterion_main!(benches);
