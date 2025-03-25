import fs from "fs";
import path from "node:path";
// import { rootNode } from "@codama/nodes";
import { rootNodeFromAnchor } from "@codama/nodes-from-anchor";
import { readJson } from "@codama/renderers-core";
import { visit } from "@codama/visitors-core";

import { renderVisitor as renderRustVisitor } from "@codama/renderers-rust";
import { renderVisitor as renderVixenVisitor } from "@codama/renderers-vixen-parser";

function generateProject(project) {
  const packageName = `yellowstone-vixen-${project}-parser`;
  const rootDir = process.cwd();

  const idl = readJson(`./idls/${project}.json`);

  // const node = rootNode(idl.program);
  const node = rootNodeFromAnchor(idl);

  // #Renderers-rust
  visit(
    node,
    renderRustVisitor(
      path.join(
        rootDir,
        "crates/parsers_codama",
        packageName,
        "src",
        "generated_sdk"
      ),
      {
        crateFolder: path.join(rootDir, "crates/parsers_codama", packageName),
        formatCode: true,
      }
    )
  );

  //  #Render Vixen Parser
  visit(
    node,
    renderVixenVisitor(
      path.join(
        rootDir,
        "crates/parsers_codama",
        packageName,
        "src",
        "generated_parser"
      ),
      {
        sdkName: "crate",
        crateFolder: path.join(rootDir, "crates/parsers_codama", packageName),
        formatCode: true,
      }
    )
  );
}

function main() {
  const files = fs.readdirSync("idls");

  files.forEach((file) => {
    const fileNameWithoutExtension = path.parse(file).name;
    generateProject(fileNameWithoutExtension);
  });
}

main();
