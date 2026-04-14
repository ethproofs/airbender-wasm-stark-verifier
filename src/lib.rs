// This crate builds from the ethproofs/ethereum-prover fork
// The actual implementation is in the fork's proof_verifier_js/wasm crate
// This file re-exports with the simplified API

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
pub fn verify_stark(
    proof_bytes: &[u8],
    vk_bytes: Option<js_sys::Array>,
) -> Result<bool, JsValue> {
    match vk_bytes {
        Some(arr) => {
            if arr.length() != 2 {
                return Err(JsValue::from_str("vk_bytes must contain [setup_bin, layout_bin]"));
            }
            let setup: js_sys::Uint8Array = arr.get(0).dyn_into()
                .map_err(|_| JsValue::from_str("vk_bytes[0] (setup_bin) must be a Uint8Array"))?;
            let layout: js_sys::Uint8Array = arr.get(1).dyn_into()
                .map_err(|_| JsValue::from_str("vk_bytes[1] (layout_bin) must be a Uint8Array"))?;
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
