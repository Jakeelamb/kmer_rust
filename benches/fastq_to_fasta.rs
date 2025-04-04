use criterion::{criterion_group, criterion_main, Criterion};
use kmerr::io_utils::fastq_to_fasta;

fn bench_conversion(c: &mut Criterion) {
    c.bench_function("fastq to fasta", |b| {
        b.iter(|| {
            fastq_to_fasta("test_data/test.fastq.gz", "test_data/bench_output.fasta");
        });
    });
}

criterion_group!(benches, bench_conversion);
criterion_main!(benches);
