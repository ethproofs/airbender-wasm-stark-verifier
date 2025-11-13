use cli_lib::prover_utils::{ProgramProof, generate_oracle_data_from_metadata_and_proof_list};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(long)]
    input_file: String,
}

pub fn main() {
    let cli = Cli::parse();
    println!("Input file: {}", cli.input_file);
    // Read binary data from file
    let data = std::fs::read(cli.input_file).expect("Failed to read input file");

    let program_proof: ProgramProof =
        bincode::serde::decode_from_slice(&data, bincode::config::standard())
            .expect("Failed to deserialize ProgramProof")
            .0;

    let end_params = program_proof.end_params;

    let (metadata, proof_list) = program_proof.to_metadata_and_proof_list();

    let oracle_data = generate_oracle_data_from_metadata_and_proof_list(&metadata, &proof_list);
    let it = oracle_data.into_iter();

    verifier_common::prover::nd_source_std::set_iterator(it);

    // Assume that program proof has only recursion proofs.
    println!("Running continue recursive");
    assert!(metadata.reduced_proof_count > 0);
    let output = full_statement_verifier::verify_recursion_log_23_layer();

    assert!(
        verifier_common::prover::nd_source_std::try_read_word().is_none(),
        "Expected that all words from CSR were consumed"
    );

    println!("End params (recursion verification key): {:?}", end_params);
    println!("Basic program (verification key): {:?}", &output[8..16]);
    println!("Public input (per block): {:?}", &output[0..8]);
}
