// Global setup for WASM tests - initializes the WASM module
import { fileURLToPath } from "url";
import { dirname } from "path";
import { readFileSync } from "fs";

// Polyfill __dirname and require for ESM + wasm-pack compatibility
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Make __dirname and require available globally for the wasm module
(global as any).__dirname = __dirname;
(global as any).require = (module: string) => {
  if (module === "fs") {
    return { readFileSync };
  }
  throw new Error(`Module ${module} not found`);
};

// Pre-import the WASM module to ensure it's initialized before tests run
await import("kreuzcrawl");
