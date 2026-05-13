export { ProofHandle, SecurityLevel, VerifyResult, WasmVerifier, deserialize_proof_bytes, main } from './pkg/airbender_wasm_stark_verifier.js';

export function verify_stark(proof_bytes: Uint8Array, vk_bytes: Uint8Array): boolean;

export default function init(): Promise<void>;
