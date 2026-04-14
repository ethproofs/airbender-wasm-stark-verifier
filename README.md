# Airbender WASM STARK Verifier

WebAssembly bindings for the [Airbender](https://github.com/matter-labs/zksync-airbender) STARK verifier. Verify Ethereum STF ZK proofs in browsers and Node.js.

## Installation

```bash
npm install @ethproofs/airbender-wasm-stark-verifier
```

## Usage

### Browser / Bundler (React, Next.js, Vite, etc.)

```typescript
import init, { main, verify_stark } from '@ethproofs/airbender-wasm-stark-verifier';

// Initialize WASM module
await init();

// Initialize the verifier (sets up panic hook and default config)
main();

// Verify a proof with default config - returns true if valid
const isValid = verify_stark(proofBytes);

// Or verify with custom setup/layout binaries (length-prefixed)
// First 4 bytes = big-endian u32 length of setupBin, followed by setupBin, then layoutBin
const vkBytes = new Uint8Array(4 + setupBin.byteLength + layoutBin.byteLength);
new DataView(vkBytes.buffer).setUint32(0, setupBin.byteLength);
vkBytes.set(setupBin, 4);
vkBytes.set(layoutBin, 4 + setupBin.byteLength);
const isValid = verify_stark(proofBytes, vkBytes);
```

### Node.js

```javascript
import { main, verify_stark } from '@ethproofs/airbender-wasm-stark-verifier/pkg-node/airbender_wasm_stark_verifier.js';

// Initialize the verifier
main();

// Verify a proof with default config
const isValid = verify_stark(proofBytes);

// Or verify with custom setup/layout binaries (length-prefixed)
const vkBytes = new Uint8Array(4 + setupBin.byteLength + layoutBin.byteLength);
new DataView(vkBytes.buffer).setUint32(0, setupBin.byteLength);
vkBytes.set(setupBin, 4);
vkBytes.set(layoutBin, 4 + setupBin.byteLength);
const isValid = verify_stark(proofBytes, vkBytes);
```

### Advanced Usage

For more control over the verification process:

```typescript
import init, {
  main,
  deserialize_proof_bytes,
  verify_proof,
  init_with,
} from '@ethproofs/airbender-wasm-stark-verifier';

await init();
main();

// Deserialize and verify in separate steps
const handle = deserialize_proof_bytes(proofBytes);
const result = verify_proof(handle);

if (result.success) {
  console.log('Proof is valid');
} else {
  console.error('Verification failed:', result.error());
}

// Or use custom setup/layout for non-default circuit versions
init_with(new Uint8Array(setupBin), new Uint8Array(layoutBin));
```

## API Reference

- `main()` - initializes the panic hook and default verifier configuration. Call this once before verifying proofs.

- `verify_stark(proofBytes: Uint8Array, vkBytes?: Uint8Array | null): boolean` - verifies a proof and returns `true` if valid. Optionally pass a length-prefixed `Uint8Array` containing setup and layout binaries for non-default circuit versions. The format is: 4 bytes (big-endian u32) for the setup length, followed by the setup bytes, followed by the layout bytes.

- `deserialize_proof_bytes(proofBytes: Uint8Array): ProofHandle` - deserializes proof bytes into a handle for verification.

- `verify_proof(handle: ProofHandle): VerifyResult` - verifies a deserialized proof. Returns an object with `success` boolean and `error()` method.

- `init_with(setupBin: Uint8Array, layoutBin: Uint8Array)` - initializes the verifier with custom setup and layout binaries for non-default circuit versions.

- `init_defaults()` - initializes the verifier with default configuration (called automatically by `main()`).

## Building

```bash
# Build for bundlers (default)
npm run build

# Build for Node.js
npm run build:node

# Build all targets
npm run build:all
```

## Testing

```bash
# Run Node.js test with a proof file
npm run test:node -- path/to/proof.bin
```

## License

MIT
