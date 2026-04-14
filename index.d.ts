export { ProofHandle, VerifyResult, deserialize_proof_bytes, init_defaults, init_with, main, verify_proof } from './pkg/airbender_wasm_stark_verifier.js';

export function verify_stark(proof_bytes: Uint8Array, vk_bytes?: Uint8Array | null): boolean;

export default function init(): Promise<void>;
