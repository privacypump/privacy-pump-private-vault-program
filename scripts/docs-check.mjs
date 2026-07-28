import { readFileSync } from "node:fs";

const required = [
  "README.md",
  "ARCHITECTURE.md",
  "SECURITY.md",
  "CONTRIBUTING.md",
  "CHANGELOG.md",
  "AGENTS.md",
  "LICENSE_STATUS.md",
  "CODE_OF_CONDUCT.md",
];

for (const file of required) {
  const content = readFileSync(file, "utf8");
  if (!content.trim()) throw new Error(file + " is empty");
}

const readme = readFileSync("README.md", "utf8");
for (const link of readme.matchAll(/\[[^\]]+\]\(([^)]+)\)/g)) {
  const target = link[1];
  if (/^[a-z]+:/i.test(target) || target.startsWith("#")) continue;
  readFileSync(target, "utf8");
}

console.log("Documentation checks passed.");
