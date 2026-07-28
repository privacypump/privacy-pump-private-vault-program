import { lstatSync, readFileSync, readdirSync } from "node:fs";
import { extname, join, relative, sep } from "node:path";

const root = process.cwd();
const ignored = new Set([
  ".git",
  "node_modules",
  "target",
  "dist",
  "build",
  "coverage",
]);
const forbiddenSegments = new Set([".vercel", "keys", ".keys"]);
const binaryExtensions = new Set([
  ".png",
  ".jpg",
  ".jpeg",
  ".webp",
  ".gif",
  ".zip",
  ".gz",
  ".so",
  ".dll",
  ".dylib",
]);
const findings = [];

function walk(directory) {
  for (const name of readdirSync(directory)) {
    if (ignored.has(name)) continue;
    const absolute = join(directory, name);
    const path = relative(root, absolute).split(sep).join("/");
    const stat = lstatSync(absolute);
    if (stat.isSymbolicLink()) {
      findings.push(path + ": symbolic links are not allowed");
      continue;
    }
    if (stat.isDirectory()) {
      if (forbiddenSegments.has(name)) findings.push(path + ": forbidden path");
      walk(absolute);
      continue;
    }
    if (
      forbiddenSegments.has(name) ||
      /\.(pem|p12|jwk|jwt|proof|witness|so|map)$/i.test(name)
    ) {
      findings.push(path + ": forbidden file type");
    }
    if (
      binaryExtensions.has(extname(name).toLowerCase()) ||
      path === "scripts/policy-scan.mjs"
    )
      continue;
    let text;
    try {
      text = readFileSync(absolute, "utf8");
    } catch {
      continue;
    }
    const checks = [
      [/[A-Za-z]:\\Users\\/g, "absolute Windows user path"],
      [/\/home\/[A-Za-z0-9._-]+\//g, "absolute home path"],
      [/BEGIN (?:RSA |EC |OPENSSH |DSA )?PRIVATE KEY/g, "private key marker"],
      [/AKIA[0-9A-Z]{16}/g, "AWS access key"],
      [/gh[pousr]_[A-Za-z0-9_]{20,}/g, "GitHub token"],
      [/\[(?:\s*\d+\s*,){31,}\s*\d+\s*\]/g, "keypair-like numeric array"],
      [
        /privacyPumpPrivateVaultRouteRuntime|pendingPackageEnvelope|SUPABASE_SERVICE_ROLE_KEY|AWS_KMS_KEY_ARN/g,
        "production-only implementation reference",
      ],
    ];
    for (const [pattern, label] of checks) {
      if (pattern.test(text)) findings.push(path + ": " + label);
    }
  }
}

walk(root);
if (findings.length) {
  console.error(findings.join("\n"));
  process.exit(1);
}
console.log("Policy scan passed.");
