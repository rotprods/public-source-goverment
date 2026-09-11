import { readFile, readdir } from "node:fs/promises";
import path from "node:path";

const root = process.cwd();

async function walkJsonFiles(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const files = [];
  for (const entry of entries) {
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory()) {
      files.push(...(await walkJsonFiles(fullPath)));
    } else if (entry.isFile() && entry.name.endsWith(".json")) {
      files.push(fullPath);
    }
  }
  return files;
}

const jsonContracts = await walkJsonFiles(path.join(root, "contracts"));
for (const file of jsonContracts) {
  const source = await readFile(file, "utf8");
  const parsed = JSON.parse(source);
  if (file.endsWith(".schema.json") && typeof parsed.$schema !== "string") {
    throw new Error(`${path.relative(root, file)} is a schema without $schema`);
  }
}

for (const stateFile of ["STATE.json", "GOAL_STATE.json"]) {
  JSON.parse(await readFile(path.join(root, stateFile), "utf8"));
}

const openapi = await readFile(path.join(root, "contracts/openapi/openapi.yaml"), "utf8");
if (!openapi.includes("openapi: 3.1.0") || !openapi.includes("/healthz:")) {
  throw new Error("bootstrap OpenAPI must be 3.1 and document /healthz");
}

const apiSource = await readFile(path.join(root, "services/api/src/main.rs"), "utf8");
if (!apiSource.includes('route("/healthz"')) {
  throw new Error("OpenAPI documents /healthz but the Rust bootstrap route is absent");
}

console.log(`contracts ok: ${jsonContracts.length} JSON contract(s) + bootstrap OpenAPI/runtime parity`);
