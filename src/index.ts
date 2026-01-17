import {
  createVerifier,
  type Verifier,
  type VerifierOptions,
  type VerificationResult,
  type ProofHandle,
} from "@matterlabs/ethproofs-airbender-verifier";

// Re-export all types and the createVerifier function for advanced usage
export {
  createVerifier,
  type Verifier,
  type VerifierOptions,
  type VerificationResult,
  type ProofHandle,
};

// Cached verifier instance for the simple API
let cachedVerifier: Verifier | null = null;

/**
 * Gets or creates a verifier instance with default options.
 * The verifier is cached for subsequent calls.
 */
async function getVerifier(): Promise<Verifier> {
  if (!cachedVerifier) {
    cachedVerifier = await createVerifier();
  }
  return cachedVerifier;
}

/**
 * Simple verification API that takes proof bytes and returns a boolean.
 *
 * This is a convenience wrapper around the full verifier API.
 * For more control, use `createVerifier()` directly.
 *
 * @param proofBytes - The raw proof bytes to verify
 * @returns Promise<boolean> - true if the proof is valid, false otherwise
 */
export async function verify_stark(proofBytes: Uint8Array): Promise<boolean> {
  const verifier = await getVerifier();
  const handle = verifier.deserializeProofBytes(proofBytes);
  const result = verifier.verifyProof(handle);
  return result.success;
}

/**
 * Verification API that returns the full result including error details.
 *
 * @param proofBytes - The raw proof bytes to verify
 * @returns Promise<VerificationResult> - Object with success boolean and optional error string
 */
export async function verify_stark_with_result(
  proofBytes: Uint8Array
): Promise<VerificationResult> {
  const verifier = await getVerifier();
  const handle = verifier.deserializeProofBytes(proofBytes);
  return verifier.verifyProof(handle);
}

/**
 * Resets the cached verifier instance.
 * Call this if you need to reinitialize with different options.
 */
export function resetVerifier(): void {
  cachedVerifier = null;
}
