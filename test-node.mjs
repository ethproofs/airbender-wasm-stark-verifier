import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import { main, verify_stark } from './pkg-node/ethproofs_verifier_lib.js';

async function testVerification() {
  console.log('🧪 Testing Airbender WASM STARK Verifier\n');

  try {
    main();
    console.log('✅ WASM module initialized\n');

    // Load proof
    const proofPath = path.join(__dirname, 'data', 'airbender-proof.bin');

    console.log('\nLoading proof...');
    const proofBytes = fs.readFileSync(proofPath);

    console.log(`  Proof size: ${proofBytes.length} bytes`);

    // Test verification
    console.log('\n🔍 Verifying Airbender block proof...');
    const start = performance.now();
    const result = verify_stark(proofBytes);
    const end = performance.now();
    console.log(`✅ Verification result: ${result}`);

    console.log('\n📊 Verification Summary:');
    console.log(`  STARK proof: ${result ? '✅ VALID' : '❌ INVALID'}`);
    console.log(`  Time taken: ${end - start} milliseconds`);
  } catch (error) {
    console.error('❌ Error during verification:', error.message);
    console.error('Stack trace:', error.stack);
    process.exit(1);
  }
}

testVerification();
