use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use flate2::read::MultiGzDecoder;
use needletail::{parse_fastx_reader};

pub fn fastq_to_fasta(input_path: &str, output_path: &str) {
    // Open input file and detect if it's gzipped
    let file = File::open(input_path).expect("Unable to open input file");
    let is_gzipped = input_path.ends_with(".gz");
    
    // Create appropriate reader based on file type
    let reader: Box<dyn Read + Send> = if is_gzipped {
        Box::new(MultiGzDecoder::new(file))
    } else {
        Box::new(file)
    };
    
    let reader = BufReader::new(reader);
    let mut out = BufWriter::new(File::create(output_path).expect("Cannot create output file"));

    // Parse FASTQ and write FASTA
    let mut records = parse_fastx_reader(reader).expect("Invalid FASTQ input");
    while let Some(record) = records.next() {
        let record = record.expect("Error parsing FASTQ");
        let id = std::str::from_utf8(record.id()).unwrap();
        let seq_data = record.seq();
        let seq = std::str::from_utf8(&seq_data).unwrap();
        writeln!(out, ">{}\n{}", id, seq).expect("Failed to write FASTA");
    }
}
