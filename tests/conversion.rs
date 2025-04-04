use kmerr::io_utils::fastq_to_fasta;
use std::fs;

#[test]
fn test_fastq_to_fasta_conversion() {
    let input = "test_data/test.fastq";
    let output = "test_data/test_output.fasta";
    let expected = fs::read_to_string("test_data/expected.fasta").unwrap();

    fastq_to_fasta(input, output);

    let actual = fs::read_to_string(output).unwrap();
    assert_eq!(actual, expected);
}
