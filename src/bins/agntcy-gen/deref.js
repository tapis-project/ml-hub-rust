const $RefParser = require("@apidevtools/json-schema-ref-parser");
const fs = require("fs");

async function derefSchema() {
  const schema = await $RefParser.dereference("agntcy0.7.0-for-js.json");
  fs.writeFileSync("agntcy0.7.0-derefed-node.json", JSON.stringify(schema, null, 2));
}

derefSchema();