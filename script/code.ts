import { walkSync } from "https://deno.land/std/fs/mod.ts";

function listMatchingFiles(pattern: RegExp, path: string): string[] {
  let results = [];
  for (
    const entry of walkSync(path, { match: [pattern], includeDirs: false })
  ) {
    results.push(entry.path);
  }
  return results;
}

function getWasmCode(): string {
  const directoryPath = "./target/wasm32-unknown-unknown/release";
  const wasmFilePattern = /.*\.opt\.wasm$/;

  let results = listMatchingFiles(wasmFilePattern, directoryPath);

  if (results.length === 0) {
    throw Error(`no matching file found within ${path}`);
  }
  if (results.length > 1) {
    throw Error(`multiple matching file found within ${path}: ${results}`);
  }

  return Deno.readFileSync(results[0]);
}

export const code = getWasmCode();
