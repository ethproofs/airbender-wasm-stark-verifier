import { verify_stark_with_result, createVerifier } from "./dist/index.js";
import { readFile } from "fs/promises";
import { fileURLToPath } from "url";

// Polyfill fetch for Node.js to handle file:// URLs for WASM loading
const originalFetch = globalThis.fetch;
globalThis.fetch = async (input, init) => {
  if (typeof input === "string" && input.startsWith("file://")) {
    const filePath = fileURLToPath(input);
    const buffer = await readFile(filePath);
    return new Response(buffer, {
      headers: { "Content-Type": "application/wasm" },
    });
  }
  if (input instanceof URL && input.protocol === "file:") {
    const buffer = await readFile(input);
    return new Response(buffer, {
      headers: { "Content-Type": "application/wasm" },
    });
  }
  return originalFetch(input, init);
};

async function main() {
  console.log("Testing @ethproofs/airbender-wasm-stark-verifier\n");

  // Check if a proof file was provided as argument
  const proofPath = process.argv[2];

  if (!proofPath) {
    console.log("No proof file provided. Testing module initialization only.\n");

    // Test that createVerifier works
    console.log("Initializing verifier...");
    const verifier = await createVerifier();
    console.log("✓ Verifier initialized successfully\n");

    console.log("Available methods:");
    console.log("  - verifier.deserializeProofBytes(proofBytes)");
    console.log("  - verifier.verifyProof(handle)\n");

    console.log("Usage: node test-node.mjs <path-to-proof-file>");
    return;
  }

  // Load and verify the proof
  console.log(`Loading proof from: ${proofPath}`);
  const proofBytes = new Uint8Array(await readFile(proofPath));
  console.log(`Proof size: ${proofBytes.length} bytes\n`);

  console.log("Verifying proof...");
  const startTime = performance.now();

  const result = await verify_stark_with_result(proofBytes);

  const elapsed = (performance.now() - startTime).toFixed(2);

  if (result.success) {
    console.log(`✓ Proof is VALID (${elapsed}ms)`);
  } else {
    console.log(`✗ Proof is INVALID (${elapsed}ms)`);
    console.log(`  Error: ${result.error}`);
  }
}

main().catch((err) => {
  console.error("Error:", err.message);
  process.exit(1);
});
