// This crate builds from the ethproofs/ethereum-prover fork
// The actual implementation is in the fork's proof_verifier_js/wasm crate
// This file re-exports with the simplified API

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;

// Re-export from the fork's crate
pub use proof_verifier_wasm::{
    deserialize_proof_bytes, ProofHandle, SecurityLevel, VerifyResult, WasmVerifier,
};

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
}

#[wasm_bindgen]
pub fn verify_stark(proof_bytes: &[u8], vk_bytes: &[u8]) -> Result<bool, JsValue> {
    let verifier = WasmVerifier::from_key(vk_bytes)?;
    let handle = deserialize_proof_bytes(proof_bytes)?;
    let result = verifier.verify_proof(&handle);
    Ok(result.success())
}
