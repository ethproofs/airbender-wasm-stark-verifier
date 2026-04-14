// This crate builds from the ethproofs/ethereum-prover fork
// The actual implementation is in the fork's proof_verifier_js/wasm crate
// This file re-exports with the simplified API

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;

// Re-export from the fork's crate
pub use proof_verifier_wasm::{
    deserialize_proof_bytes, init_defaults, init_with, verify_proof, ProofHandle, VerifyResult,
};

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
    init_defaults().expect("failed to initialize verifier");
}

#[wasm_bindgen]
pub fn verify_stark(
    proof_bytes: &[u8],
    vk_bytes: Option<js_sys::Uint8Array>,
) -> Result<bool, JsValue> {
    match vk_bytes {
        Some(arr) => {
            let bytes = arr.to_vec();
            if bytes.len() < 4 {
                return Err(JsValue::from_str(
                    "vk_bytes too short to contain length prefix",
                ));
            }
            let setup_len = u32::from_be_bytes(bytes[..4].try_into().unwrap()) as usize;
            if bytes.len() < 4 + setup_len {
                return Err(JsValue::from_str("vk_bytes setup length exceeds buffer"));
            }
            let setup = &bytes[4..4 + setup_len];
            let layout = &bytes[4 + setup_len..];
            init_with(&setup.to_vec(), &layout.to_vec())?;
        }
        None => {
            init_defaults()?;
        }
    }

    let handle = deserialize_proof_bytes(proof_bytes)?;
    let result = verify_proof(&handle);
    Ok(result.success())
}
