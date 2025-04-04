use dashmap::DashMap;
use needletail::parse_fastx_reader;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

// Lookup table for nucleotide complements
const COMPLEMENT: [u8; 256] = {
    let mut comp = [0; 256];
    comp[b'A' as usize] = b'T';
    comp[b'C' as usize] = b'G';
    comp[b'G' as usize] = b'C';
    comp[b'T' as usize] = b'A';
    comp[b'a' as usize] = b't';
    comp[b'c' as usize] = b'g';
    comp[b'g' as usize] = b'c';
    comp[b't' as usize] = b'a';
    comp[b'N' as usize] = b'N';
    comp[b'n' as usize] = b'n';
    comp
};

/// Compute the reverse complement of a DNA sequence
fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    let mut rc = Vec::with_capacity(seq.len());
    for &base in seq.iter().rev() {
        rc.push(COMPLEMENT[base as usize]);
    }
    rc
}

/// Returns canonical kmer: the lexicographically smallest of the kmer or its reverse complement
fn canonical_kmer(seq: &[u8]) -> Box<[u8]> {
    let rc = reverse_complement(seq);
    if seq <= rc.as_slice() {
        seq.to_vec().into_boxed_slice()
    } else {
        rc.into_boxed_slice()
    }
}

/// Parallel k-mer counting using DashMap for thread-safe updates
pub fn build_kmer_table_parallel(fasta_path: &str, k: usize) -> DashMap<Box<[u8]>, u32> {
    // First, load all sequences into memory
    let mut sequences = Vec::new();
    
    // Read the file
    let reader = BufReader::new(File::open(fasta_path).expect("Cannot open FASTA file"));
    let mut fastx_reader = parse_fastx_reader(reader).expect("Invalid FASTA input");
    
    // Iterate through records one at a time
    let mut record_counter = 0;
    loop {
        match fastx_reader.next() {
            Some(Ok(record)) => {
                record_counter += 1;
                sequences.push(record.seq().to_vec());
            }
            Some(Err(e)) => {
                eprintln!("Error reading record {}: {}", record_counter, e);
            }
            None => break,
        }
    }

    // Use DashMap for concurrent k-mer counting
    let kmer_map = Arc::new(DashMap::new());

    // Process sequences in parallel
    sequences.par_iter().for_each(|seq| {
        if seq.len() < k {
            return;
        }

        for i in 0..=seq.len() - k {
            let kmer = &seq[i..i+k];
            let canon = canonical_kmer(kmer);
            let mut entry = kmer_map.entry(canon).or_insert(0);
            *entry += 1;
        }
    });

    Arc::try_unwrap(kmer_map).expect("DashMap is still referenced")
}
