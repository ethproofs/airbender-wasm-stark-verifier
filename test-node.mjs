import { main, verify_stark } from "./pkg-node/airbender_wasm_stark_verifier.js";
import { readFile } from "fs/promises";

async function test() {
  console.log("Testing @ethproofs/airbender-wasm-stark-verifier\n");

  const proofPath = process.argv[2];

  if (!proofPath) {
    console.log("No proof file provided. Testing module initialization only.\n");
    console.log("Initializing verifier...");
    main();
    console.log("✓ Verifier initialized successfully\n");
    console.log("Usage: node test-node.mjs <path-to-proof-file>");
    return;
  }

  console.log(`Loading proof from: ${proofPath}`);
  const proofBytes = new Uint8Array(await readFile(proofPath));
  console.log(`Proof size: ${proofBytes.length} bytes\n`);

  console.log("Initializing verifier...");
  main();
  console.log("✓ Verifier initialized\n");

  console.log("Verifying proof...");
  const startTime = performance.now();

  const result = verify_stark(proofBytes);

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
