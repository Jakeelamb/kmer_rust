mod io_utils;
mod kmer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage:\n{} fasta2fasta <input> <output>\n{} kmer <input> <k>", args[0], args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "fasta2fasta" => io_utils::fastq_to_fasta(&args[2], &args[3]),
        "kmer" => {
            let k = args[3].parse::<usize>().expect("Invalid k value");
            let table = kmer::build_kmer_table_parallel(&args[2], k);
            println!("Parallel K-mer table contains {} entries", table.len());
        }
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}

