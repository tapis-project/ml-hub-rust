#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");

const [, , release] = process.argv;
const supportedReleases = new Set(["minor", "patch"]);
const sdkRoot = path.resolve(__dirname, "..");
const servicesDirectory = path.join(sdkRoot, "services");

if (!supportedReleases.has(release) || process.argv.length !== 3) {
  console.error("Usage: npm run bump-version -- <minor|patch>");
  process.exit(1);
}

const configPaths = fs
  .readdirSync(servicesDirectory, { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => path.join(servicesDirectory, entry.name, "config.json"))
  .filter((configPath) => fs.existsSync(configPath))
  .sort();

if (configPaths.length === 0) {
  throw new Error(`No services/*/config.json files found in ${servicesDirectory}`);
}

for (const configPath of configPaths) {
  bumpVersion(configPath, release);
}

function bumpVersion(configPath, release) {
  const contents = fs.readFileSync(configPath, "utf8");
  const config = JSON.parse(contents);
  const currentVersion = config.npmVersion;

  if (typeof currentVersion !== "string") {
    throw new Error(`${configPath} does not define a string npmVersion`);
  }

  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(currentVersion);

  if (match === null) {
    throw new Error(`${configPath} has an invalid npmVersion: ${currentVersion}`);
  }

  const major = Number(match[1]);
  const minor = Number(match[2]);
  const patch = Number(match[3]);
  const nextVersion =
    release === "minor"
      ? `${major}.${minor + 1}.0`
      : `${major}.${minor}.${patch + 1}`;
  const versionPattern = /(\"npmVersion\"\s*:\s*\")[^\"]+(\")/;
  const nextContents = contents.replace(
    versionPattern,
    `$1${nextVersion}$2`,
  );

  if (nextContents === contents) {
    throw new Error(`Unable to update npmVersion in ${configPath}`);
  }

  fs.writeFileSync(configPath, nextContents);

  console.log(`${path.normalize(configPath)}: ${currentVersion} -> ${nextVersion}`);
}
