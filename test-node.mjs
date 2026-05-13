import { verify_stark } from "./pkg-node/airbender_wasm_stark_verifier.js";
import { readFile } from "fs/promises";

async function test() {
  console.log("Testing @ethproofs/airbender-wasm-stark-verifier\n");

  const proofPath = process.argv[2];
  const vkPath = process.argv[3];

  if (!proofPath || !vkPath) {
    console.log("Usage: node test-node.mjs <path-to-proof-file> <path-to-vk-file>");
    return;
  }

  console.log(`Loading proof from: ${proofPath}`);
  const proofBytes = new Uint8Array(await readFile(proofPath));
  console.log(`Proof size: ${proofBytes.length} bytes`);

  console.log(`Loading verification key from: ${vkPath}`);
  const vkBytes = new Uint8Array(await readFile(vkPath));
  console.log(`VK size: ${vkBytes.length} bytes\n`);

  console.log("Verifying proof...");
  const startTime = performance.now();

  const result = verify_stark(proofBytes, vkBytes);

  const elapsed = (performance.now() - startTime).toFixed(2);

  if (result) {
    console.log(`✓ Proof is VALID (${elapsed}ms)`);
  } else {
    console.log(`✗ Proof is INVALID (${elapsed}ms)`);
  }
}

test().catch((err) => {
  console.error("Error:", err.message);
  process.exit(1);
});
