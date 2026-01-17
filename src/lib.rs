// This crate builds from the ethproofs/ethereum-prover fork
// The actual implementation is in the fork's proof_verifier_js/wasm crate
// This file re-exports with the simplified API

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;

// Re-export from the fork's crate
pub use proof_verifier_wasm::{
    init_defaults,
    init_with,
    deserialize_proof_bytes,
    verify_proof,
    ProofHandle,
    VerifyResult,
};

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
    init_defaults().expect("failed to initialize verifier");
}

#[wasm_bindgen]
pub fn verify_stark(proof_bytes: &[u8]) -> Result<bool, JsValue> {
    let handle = deserialize_proof_bytes(proof_bytes)?;
    let result = verify_proof(&handle);
    Ok(result.success())
}
