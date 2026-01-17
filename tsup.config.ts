import { defineConfig } from "tsup";
import { copyFileSync } from "fs";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["esm", "cjs"],
  dts: true,
  clean: true,
  sourcemap: true,
  noExternal: ["@matterlabs/ethproofs-airbender-verifier"],
  onSuccess: async () => {
    // Copy WASM file to dist
    const wasmSrc = "node_modules/@matterlabs/ethproofs-airbender-verifier/wasm/pkg/proof_verifier_wasm_bg.wasm";
    const wasmDest = "dist/proof_verifier_wasm_bg.wasm";
    copyFileSync(wasmSrc, wasmDest);
    console.log("Copied WASM file to dist/");
  },
});
