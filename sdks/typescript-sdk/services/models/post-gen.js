const fs = require("fs");

// Replace every instance of url encoding of a path-like path parameter with a non-urlencoded string
let contents = fs.readFileSync("../../gen/models/src/apis/PlatformsApi.ts", "utf8");

let modifiedContents = contents.replace(
    "encodeURIComponent(String(requestParameters.modelId))",
    "String(requestParameters.modelId)"
);

fs.writeFileSync("../../gen/models/src/apis/PlatformsApi.ts", modifiedContents);

