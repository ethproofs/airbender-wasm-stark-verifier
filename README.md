# Airbender WASM STARK Verifier

WebAssembly bindings for the [Airbender](https://github.com/matter-labs/zksync-airbender) STARK verifier. Verify Ethereum STF ZK proofs in browsers and Node.js.

## Installation

```bash
npm install @ethproofs/airbender-wasm-stark-verifier
```

## Usage

### Browser / Bundler (React, Next.js, Vite, etc.)

```typescript
import init, { verify_stark } from '@ethproofs/airbender-wasm-stark-verifier';

// Initialize WASM module
await init();

// Verify a proof with a single-file verification key - returns true if valid
const isValid = verify_stark(proofBytes, vkBytes);
```

### Node.js

```javascript
import { verify_stark } from '@ethproofs/airbender-wasm-stark-verifier/pkg-node/airbender_wasm_stark_verifier.js';

// Verify a proof with a single-file verification key
const isValid = verify_stark(proofBytes, vkBytes);
```

### Advanced Usage

For more control over the verification process, or to verify against legacy split (setup + layout) verification keys:

```typescript
import init, {
  WasmVerifier,
  deserialize_proof_bytes,
} from '@ethproofs/airbender-wasm-stark-verifier';

await init();

// Single-file verification key (recommended)
const verifier = WasmVerifier.fromKey(vkBytes);

// Or, for legacy 80-bit deployments, use the two-file format
// const verifier = WasmVerifier.fromLegacyKey(setupBytes, layoutBytes);

const handle = deserialize_proof_bytes(proofBytes);
const result = verifier.verifyProof(handle);

if (result.success) {
  console.log('Proof is valid');
} else {
  console.error('Verification failed:', result.error());
}
```

## API Reference

- `verify_stark(proofBytes: Uint8Array, vkBytes: Uint8Array): boolean` - verifies a proof against a single-file verification key and returns `true` if valid. The verification key carries its security level (80-bit or 100-bit) explicitly; mismatches with the proof's security level are rejected.

- `WasmVerifier.fromKey(vkBytes: Uint8Array): WasmVerifier` - constructs a verifier from a single-file verification key.

- `WasmVerifier.fromLegacyKey(setupBytes: Uint8Array, layoutBytes: Uint8Array): WasmVerifier` - constructs a verifier from the legacy two-file format. Only supports 80-bit security; new integrations should use `fromKey`.

- `WasmVerifier.verifyProof(handle: ProofHandle): VerifyResult` - verifies a deserialized proof. Returns an object with a `success` boolean and an `error()` method.

- `deserialize_proof_bytes(proofBytes: Uint8Array): ProofHandle` - deserializes gzipped proof bytes into a handle for verification.

- `SecurityLevel` - enum with `Security80` and `Security100` variants.

- `main()` - initializes the panic hook. Called automatically when the WASM module loads, so explicit calls are usually unnecessary.

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
# Run Node.js test with a proof file and verification key
npm run test:node -- path/to/proof.bin path/to/vk.bin
```

## License

MIT
