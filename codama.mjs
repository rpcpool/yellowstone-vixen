import fs from "fs";
import path from "node:path";
import { rootNode } from "@codama/nodes";
import { rootNodeFromAnchor } from "@codama/nodes-from-anchor";
import { readJson } from "@codama/renderers-core";
import { visit } from "@codama/visitors-core";

import { renderVisitor as renderRustVisitor } from "@codama/renderers-rust";
import { renderVisitor as renderVixenVisitor } from "@codama/renderers-vixen-parser";

function generateProject(file, node) {
  const project = path.parse(file).name;
  const packageName = `${project}-parser`;
  const rootDir = process.cwd();

  // #Renderers-rust
  visit(
    node,
    renderRustVisitor(
      path.join(rootDir, "crates", packageName, "src", "generated_sdk"),
      {
        crateFolder: path.join(rootDir, "crates", packageName),
        formatCode: true,
      }
    )
  );

  //  #Render Vixen Parser
  visit(
    node,
    renderVixenVisitor(
      path.join(rootDir, "crates", packageName, "src", "generated_parser"),
      {
        sdkName: "crate",
        crateFolder: path.join(rootDir, "crates", packageName),
        formatCode: true,
      }
    )
  );
}

function main() {
  // const anchorIdls = fs.readdirSync("idls/anchor");

  // anchorIdls.forEach((file) => {
  //   console.log(`#### ${file}:`);
  //   const idl = readJson(`./idls/anchor/${file}`);
  //   const node = rootNodeFromAnchor(idl);

  //   generateProject(file, node);
  // });

  // const shankIdls = fs.readdirSync("idls/shank");

  // shankIdls.forEach((file) => {
  //   console.log(`#### ${file}:`);
  //   const idl = readJson(`./idls/shank/${file}`);
  //   const node = rootNode(idl.program);

  //   generateProject(file, node);
  // });

  const file = "idls/failing/token.json";
  const idl = readJson(`./idls/failing/token.json`);
  const node = rootNodeFromAnchor(idl);

  generateProject(file, node);
}

main();
