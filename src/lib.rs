use execution_utils::unrolled::{UnrolledProgramProof, UnrolledProgramSetup};

use wasm_bindgen::prelude::*;

// Set up panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn verify_stark(data: &[u8]) -> js_sys::Array {
    let (verification_key, block_hash) = verify_proof(data.to_vec());
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from_str(&verification_key));
    arr.push(&JsValue::from_str(&block_hash));
    arr
}

pub fn verify_proof(data: Vec<u8>) -> (String, String) {
    let layouts_bytes = include_bytes!("../setups/compiled_layouts.bin");
    let setup_bytes = include_bytes!("../setups/setup.bin");

    // Load compiled_layouts from file if output_layouts_dir is provided
    let compiled_layouts: execution_utils::setups::CompiledCircuitsSet =
        bincode::serde::decode_from_slice(layouts_bytes, bincode::config::standard())
            .expect("Failed to deserialize compiled layouts")
            .0;

    //Load setup from file if output_layouts_dir is provided
    let setup: UnrolledProgramSetup =
        bincode::serde::decode_from_slice(setup_bytes, bincode::config::standard())
            .unwrap()
            .0;

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
    let verification_key = hex_output;

    let mut hex_output = String::new();
    for entry in &output[0..8] {
        hex_output.push_str(&format!("{:08x}", entry.swap_bytes()));
    }
    println!("Public input (block hash) as hex: {}", hex_output);
    let block_hash = hex_output;
    return (verification_key, block_hash);
}
