import { existsSync, walkSync } from "https://deno.land/std/fs/mod.ts";

function listMatchingFiles(pattern: RegExp, path: string): string[] {
  if (!existsSync(path)) {
    return [];
  }
  let results = [];
  for (
    const entry of walkSync(path, { match: [pattern], includeDirs: false })
  ) {
    results.push(entry.path);
  }
  return results;
}

function getWasmCode(): Uint8Array {
  let results = optWasmPaths();

  if (results.length === 0) {
    throw Error(`no matching file found within ${path}`);
  }
  if (results.length > 1) {
    throw Error(`multiple matching file found within ${path}: ${results}`);
  }

  return Deno.readFileSync(results[0]);
}

function optWasmPaths(): string[] {
  const directoryPath = "./target/wasm32-unknown-unknown/release";
  const wasmFilePattern = /.*\.opt\.wasm$/;

  let results = listMatchingFiles(wasmFilePattern, directoryPath);
  return results;
}

export async function code() {
  return getWasmCode();
}