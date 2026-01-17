# Airbender WASM STARK Verifier

A wrapper around [@matterlabs/ethproofs-airbender-verifier](https://www.npmjs.com/package/@matterlabs/ethproofs-airbender-verifier) providing a simple API for STARK proof verification.

## Installation

```bash
npm install @ethproofs/airbender-wasm-stark-verifier
```

## Usage

### Simple API

```typescript
import { verify_stark } from '@ethproofs/airbender-wasm-stark-verifier';

// Verify a proof - returns true if valid
const isValid = await verify_stark(proofBytes);
```

### With Error Details

```typescript
import { verify_stark_with_result } from '@ethproofs/airbender-wasm-stark-verifier';

const result = await verify_stark_with_result(proofBytes);
if (!result.success) {
  console.error('Verification failed:', result.error);
}
```

### Advanced Usage

For more control, use the full verifier API:

```typescript
import { createVerifier } from '@ethproofs/airbender-wasm-stark-verifier';

// Create verifier with default options
const verifier = await createVerifier();

// Or with custom setup/layout for non-default circuit versions
const verifier = await createVerifier({
  setupBin: setupBytes,
  layoutBin: layoutBytes,
});

// Deserialize and verify
const handle = verifier.deserializeProofBytes(proofBytes);
const result = verifier.verifyProof(handle);
```

## API Reference

- `verify_stark(proofBytes: Uint8Array): Promise<boolean>` - simple verification that returns `true` if the proof is valid.

- `verify_stark_with_result(proofBytes: Uint8Array): Promise<VerificationResult>` - returns an object with `success` boolean and optional `error` string.

- `createVerifier(options?: VerifierOptions): Promise<Verifier>` - creates a verifier instance for advanced usage. Optionally accepts custom `setupBin` and `layoutBin` for non-default circuit versions.

- `resetVerifier(): void` - resets the cached verifier instance used by the simple API.

## License

MIT
