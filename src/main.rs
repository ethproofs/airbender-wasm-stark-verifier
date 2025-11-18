use clap::Parser;
use execution_utils::{
    setups::pad_binary,
    unrolled::UnrolledProgramProof,
    verifier_binaries::{RECURSION_UNROLLED_BIN, RECURSION_UNROLLED_TXT},
};
use verifier_common::prover::risc_v_simulator::cycle::IWithoutByteAccessIsaConfigWithDelegation;

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

    let (binary, binary_u32) = pad_binary(RECURSION_UNROLLED_BIN.to_vec());
    let (text, _) = pad_binary(RECURSION_UNROLLED_TXT.to_vec());

    println!("Computing setups");
    let setup = execution_utils::unrolled::compute_setup_for_machine_configuration::<
        IWithoutByteAccessIsaConfigWithDelegation,
    >(&binary, &text);
    let compiled_layouts =
        execution_utils::setups::get_unrolled_circuits_artifacts_for_machine_type::<
            IWithoutByteAccessIsaConfigWithDelegation,
        >(&binary_u32);

    let proof: UnrolledProgramProof =
        bincode::serde::decode_from_slice(&data, bincode::config::standard())
            .expect("Failed to deserialize ProgramProof")
            .0;

    let output = execution_utils::unrolled::verify_unrolled_layer_proof(
        &proof,
        &setup,
        &compiled_layouts,
        false,
    )
    .expect("is valid proof");

    let mut hex_output = String::new();
    for entry in &output[8..16] {
        hex_output.push_str(&format!("{:08x}", entry.swap_bytes()));
    }
    println!("Basic program (verification key): {}", hex_output);

    let mut hex_output = String::new();
    for entry in &output[0..8] {
        hex_output.push_str(&format!("{:08x}", entry.swap_bytes()));
    }
    println!("Public input (block hash) as hex: {}", hex_output);
}
