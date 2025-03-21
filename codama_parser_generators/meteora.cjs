const path = require("node:path");
const { rootNode } = require("@codama/nodes");
const { rootNodeFromAnchor } = require("@codama/nodes-from-anchor");
const { readJson } = require("@codama/renderers-core");
const { visit } = require("@codama/visitors-core");

const { renderVisitor: renderRustVisitor } = require("@codama/renderers-rust");
const {
  renderVisitor: renderVixenVisitor,
} = require("@codama/renderers-vixen-parser");

const project = "meteora";
// const project = "pump_fun";

const sdkName = project + "_sdk";
const rootDir = process.cwd();

const idl = readJson(
  path.join(rootDir, "crates/parser/src", project, "idl.json")
);

// const node = rootNode(idl.program);
const node = rootNodeFromAnchor(idl);

// #Renderers-rust
visit(
  node,
  renderRustVisitor(
    path.join(rootDir, "crates/codama_sdks", sdkName, "src", "generated"),
    {
      crateFolder: path.join(rootDir, "crates/codama_sdks", sdkName),
      formatCode: true,
    }
  )
);

//  #Render Vixen Parser
visit(
  node,
  renderVixenVisitor(
    path.join(rootDir, "crates/parser/src", project, "generated"),
    {
      sdkName,
      crateFolder: path.join(rootDir, "crates/parser/src", project),
      formatCode: true,
    }
  )
);
