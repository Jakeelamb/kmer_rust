# rust-kmer-assembler

A high-performance, memory-safe k-mer assembler in Rust. Inspired by Trinity but fully reimagined from the ground up.

## 🔍 Project Goals

- Build a complete assembler in Rust
- Fully understand de Bruijn graph-based transcriptome assembly
- Optimize for speed and memory safety
- Step-by-step recreation of Trinity:
  1. FASTQ → FASTA
  2. K-mer cataloger
  3. Normalization
  4. Inchworm (contig construction)
  5. Chrysalis
  6. Butterfly

## 🚀 Usage

```bash
cargo run -- input.fastq[.gz] output.fasta
