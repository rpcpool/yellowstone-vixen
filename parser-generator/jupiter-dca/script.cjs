const path = require("node:path");
const { rootNodeFromAnchor } = require("@codama/nodes-from-anchor");
const { readJson } = require("@codama/renderers-core");
const { visit } = require("@codama/visitors-core");
const { renderVisitor } = require("@codama/renderers-vixen-parser");

const scriptDir = __dirname;
const idl = readJson(path.join(scriptDir, "idl.json"));
const projectFolder = path.join(
  scriptDir,
  "..",
  "..",
  "crates",
  "jupiter-dca-parser"
);

const node = rootNodeFromAnchor(idl); // for Anchor

visit(
  node,
  renderVisitor({
    projectFolder,
    projectName: "jupiter-dca",
  })
);
