const fs = require('fs');
const yaml = require('js-yaml');
const { execSync } = require('child_process');

console.log("MLHub Deployments API transform");

try {
  // Convert json
  const specPath = "/src/specs/deployments/openapi.json";
  const sdkServiceDir = "/src/sdks/typescript-sdk/services/deployments"
  const sdkYamlSpecPath = `${sdkServiceDir}/spec.yml`
  fs.writeFileSync(
    sdkYamlSpecPath,
    yaml.dump(JSON.parse(fs.readFileSync(specPath, 'utf-8')))  
  )
  
  // Downgrade spec
  const sdkDowngradedJsonSpecPath = `${sdkServiceDir}/downgraded_spec.json`
  execSync(`npx openapi-down-convert --allOf --input ${sdkYamlSpecPath} --output ${sdkDowngradedJsonSpecPath}`, { encoding: 'utf-8' });
  
  const sdkTransformedJsonSpecPath = `${sdkServiceDir}/transformed_spec.json`
  const sdkScriptsDir = "/src/sdks/typescript-sdk/scripts"
  const fixNullPropScript = `${sdkScriptsDir}/fix_null_properties.py`
  execSync(`python3 ${fixNullPropScript} ${sdkDowngradedJsonSpecPath} ${sdkTransformedJsonSpecPath}`, { encoding: 'utf-8' });

  // Convert final to yaml
  const sdkTransformedYamlSpecPath = `${sdkServiceDir}/transformed_spec.yml`
  fs.writeFileSync(
    sdkTransformedYamlSpecPath,
    yaml.dump(JSON.parse(fs.readFileSync(sdkTransformedJsonSpecPath, 'utf-8')))  
  )
} catch (error) {
  console.error(error);
}
