const childProcess = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");
const { writeMockHistory } = require("../lib/history-fixture");

const ROOT = path.resolve(__dirname, "../..");
const VHS_DIR = __dirname;
const FIXTURE = path.join(ROOT, "fixtures", "screenshot-history.tsv");
const MOCK_HISTORY = path.join(VHS_DIR, "mock-history.tsv");
const OUT_DIR = path.join(ROOT, "docs", "assets", "screenshots");

function main() {
  console.log("1. Preparing mock history...");
  writeMockHistory({ fixturePath: FIXTURE, outputPath: MOCK_HISTORY, root: ROOT });

  console.log("2. Building situs binary...");
  const cargo = childProcess.spawnSync("cargo", ["build"], {
    cwd: ROOT,
    stdio: "inherit",
  });
  if (cargo.status !== 0) {
    console.error("Cargo build failed");
    process.exit(1);
  }

  fs.mkdirSync(OUT_DIR, { recursive: true });

  const tapes = [
    "inline-demo.tape",
    "fullscreen-demo.tape",
    "inline-tab.tape",
  ];
  for (const tape of tapes) {
    const tapePath = path.join(VHS_DIR, tape);
    console.log(`3. Rendering ${tape}...`);
    const vhs = childProcess.spawnSync("vhs", [tapePath], {
      cwd: ROOT,
      stdio: "inherit",
    });
    if (vhs.status !== 0) {
      console.error(`VHS render failed for ${tape}`);
      process.exit(1);
    }
  }

  // Cleanup temporary mock history file
  try {
    fs.unlinkSync(MOCK_HISTORY);
  } catch (err) {}

  console.log("All VHS demos rendered successfully!");
}

main();
