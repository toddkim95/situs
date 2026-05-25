#!/usr/bin/env node

const childProcess = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const { writeMockHistory } = require("./lib/history-fixture");

const ROOT = path.resolve(__dirname, "..");
const OUT_DIR = path.join(ROOT, "docs", "assets", "screenshots");
const FIXTURE = path.join(ROOT, "fixtures", "screenshot-history.tsv");
const BIN = process.env.SITUS_BIN || path.join(ROOT, "target", "debug", "situs");
const CAPTURE_RETRIES = positiveInteger(process.env.SITUS_SCREENSHOT_RETRIES, 3);
const COLS = 132;
const ROWS = 18;
const FONT_SIZE = 14;
const CHAR_WIDTH = 8.45;
const LINE_HEIGHT = 20;
const PADDING_X = 16;
const PADDING_Y = 14;
const DEFAULT_BG = "#282e3a";
const DEFAULT_FG = "#e2e8f0";

const shots = [
  {
    name: "inline-search",
    title: "Inline search",
    picker: "inline",
    keys: [],
  },
  {
    name: "inline-inspect",
    title: "Inline inspect",
    picker: "inline",
    keys: ["\\017"],
    waitFor: "Inspect",
  },
  {
    name: "inline-help",
    title: "Inline help",
    picker: "inline",
    keys: ["\\033OP"],
    waitFor: "Keyboard",
  },
  {
    name: "fullscreen-search",
    title: "Fullscreen search",
    picker: "fullscreen",
    keys: [],
  },
  {
    name: "fullscreen-inspect",
    title: "Fullscreen inspect",
    picker: "fullscreen",
    keys: ["\\017"],
    waitFor: "Inspect",
  },
  {
    name: "fullscreen-help",
    title: "Fullscreen help",
    picker: "fullscreen",
    keys: ["\\033OP"],
    waitFor: "Keyboard",
  },
];

async function main() {
  ensureBinary();
  fs.mkdirSync(OUT_DIR, { recursive: true });
  const temp = fs.mkdtempSync(path.join(os.tmpdir(), "situs-screenshots-"));
  const history = path.join(temp, "history.tsv");
  writeMockHistory({ fixturePath: FIXTURE, outputPath: history, root: ROOT });

  for (const shot of shots) {
    await retry(`capture ${shot.name}`, CAPTURE_RETRIES, async (attempt) => {
      const rawPath = path.join(temp, `${shot.name}.${attempt}.ansi`);
      captureAnsi({ ...shot, history, rawPath, attempt });
      const raw = stripCleanup(stripExpectSpawnLine(fs.readFileSync(rawPath, "utf8")));
      let screen = parseAnsi(raw, COLS, ROWS);
      if (raw.length === 0 || screen.every(isBlankRow)) {
        throw new Error(`PTY capture was empty for ${shot.name}`);
      }
      trimBlankRows(screen);
      const svg = renderSvg(screen, shot.title);
      const svgPath = path.join(OUT_DIR, `${shot.name}.svg`);
      const pngPath = path.join(OUT_DIR, `${shot.name}.png`);
      fs.writeFileSync(svgPath, svg, "utf8");
      await renderPng(svg, pngPath);
      console.log(`wrote ${path.relative(ROOT, pngPath)}`);
    });
  }
}

function positiveInteger(value, fallback) {
  const parsed = Number.parseInt(value || "", 10);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback;
}

async function retry(label, maxAttempts, operation) {
  let lastError;
  for (let attempt = 1; attempt <= maxAttempts; attempt += 1) {
    try {
      await operation(attempt);
      if (attempt > 1) {
        console.warn(`${label} succeeded on attempt ${attempt}/${maxAttempts}`);
      }
      return;
    } catch (error) {
      lastError = error;
      if (attempt >= maxAttempts) {
        break;
      }
      console.warn(`${label} failed on attempt ${attempt}/${maxAttempts}: ${error.message}`);
      await sleep(200 * attempt);
    }
  }
  throw lastError;
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function ensureBinary() {
  if (!fs.existsSync(BIN)) {
    console.warn(`missing situs binary at ${BIN}; running cargo build`);
    const result = childProcess.spawnSync("cargo", ["build"], {
      cwd: ROOT,
      encoding: "utf8",
      stdio: "inherit",
    });
    if (result.status !== 0) {
      throw new Error("cargo build failed");
    }
  }
}

function captureAnsi({ picker, keys, waitFor, history, rawPath, attempt }) {
  const expectPath = path.join(path.dirname(rawPath), `${picker}-${path.basename(rawPath)}.expect`);
  const sendKeys = keys.map((key) => `send "${key}"\nafter 250`).join("\n");
  const waitForView = waitFor
    ? `expect {${quoteTcl(waitFor)} {} timeout {puts stderr "missing ${waitFor}"; exit 3}}`
    : "";
  const script = `
set timeout 5
set stty_init "rows ${ROWS} columns ${COLS}"
spawn env -u NO_COLOR ${quoteTcl(`SITUS_HISTORY=${history}`)} {SITUS_ATUIN_SYNC=off} ${quoteTcl(`SITUS_PICKER=${picker}`)} {SITUS_INLINE_ROWS=14} {TERM=xterm-256color} {COLORTERM=truecolor} sh -c {SITUS_TTY=$(tty); export SITUS_TTY; exec "$1" choose --picker "$2" --command cargo} situs-screenshot ${quoteTcl(BIN)} ${quoteTcl(picker)}
expect {{cargo install --path . --force} {} timeout {puts stderr "missing loaded picker"; exit 2}}
${sendKeys}
${waitForView}
after 300
send "\\003"
expect eof
`;
  fs.writeFileSync(expectPath, script, "utf8");
  const result = childProcess.spawnSync("expect", [expectPath], {
    cwd: ROOT,
    encoding: "utf8",
  });
  if (result.status !== 0) {
    const failedPrefix = `${rawPath}.failed-${attempt}`;
    fs.writeFileSync(`${failedPrefix}.stdout`, result.stdout || "", "utf8");
    fs.writeFileSync(`${failedPrefix}.stderr`, result.stderr || "", "utf8");
    throw new Error(`expect failed for ${picker}: ${result.stderr || result.stdout}`);
  }
  fs.writeFileSync(rawPath, result.stdout, "utf8");
}

function stripExpectSpawnLine(raw) {
  return raw.replace(/^spawn .*\r?\n/, "");
}

function stripCleanup(raw) {
  const matches = [...raw.matchAll(/\x1b\[[0-9]+M/g)];
  if (matches.length === 0) {
    return raw;
  }
  return raw.slice(0, matches[matches.length - 1].index);
}

function quoteTcl(value) {
  return `{${String(value).replace(/[{}]/g, "")}}`;
}

function blankCell() {
  return {
    ch: " ",
    fg: DEFAULT_FG,
    bg: DEFAULT_BG,
    bold: false,
    dim: false,
  };
}

function makeScreen(cols, rows) {
  return Array.from({ length: rows }, () => Array.from({ length: cols }, blankCell));
}

function parseAnsi(input, cols, rows) {
  let screen = makeScreen(cols, rows);
  let row = 0;
  let col = 0;
  let style = blankCell();

  for (let index = 0; index < input.length; index += 1) {
    const char = input[index];
    if (char === "\x1b") {
      const parsed = parseEscape(input, index);
      if (parsed) {
        ({ screen, row, col, style } = applyEscape(parsed, screen, row, col, style, cols, rows));
        index = parsed.end;
      }
      continue;
    }
    if (char === "\r") {
      col = 0;
      continue;
    }
    if (char === "\n") {
      row = Math.min(rows - 1, row + 1);
      continue;
    }
    if (char < " ") {
      continue;
    }
    if (row >= 0 && row < rows && col >= 0 && col < cols) {
      screen[row][col] = { ...style, ch: char };
    }
    col += 1;
    if (col >= cols) {
      col = 0;
      row = Math.min(rows - 1, row + 1);
    }
  }

  return screen;
}

function parseEscape(input, start) {
  if (input[start + 1] !== "[") {
    return { command: input[start + 1], params: "", end: start + 1 };
  }
  let end = start + 2;
  while (end < input.length && !/[A-Za-z]/.test(input[end])) {
    end += 1;
  }
  if (end >= input.length) {
    return null;
  }
  return {
    command: input[end],
    params: input.slice(start + 2, end),
    end,
  };
}

function applyEscape(parsed, screen, row, col, style, cols, rows) {
  const command = parsed.command;
  const params = parsed.params.replace(/^\?/, "");
  const numbers = params
    .split(";")
    .filter((value) => value.length > 0)
    .map((value) => Number.parseInt(value, 10));

  if (command === "m") {
    style = applySgr(numbers, style);
  } else if (command === "A") {
    row = Math.max(0, row - (numbers[0] || 1));
  } else if (command === "B") {
    row = Math.min(rows - 1, row + (numbers[0] || 1));
  } else if (command === "C") {
    col = Math.min(cols - 1, col + (numbers[0] || 1));
  } else if (command === "D") {
    col = Math.max(0, col - (numbers[0] || 1));
  } else if (command === "G") {
    col = Math.min(cols - 1, Math.max(0, (numbers[0] || 1) - 1));
  } else if (command === "H" || command === "f") {
    row = Math.min(rows - 1, Math.max(0, (numbers[0] || 1) - 1));
    col = Math.min(cols - 1, Math.max(0, (numbers[1] || 1) - 1));
  } else if (command === "J") {
    if ((numbers[0] || 0) === 2 || numbers.length === 0) {
      screen = makeScreen(cols, rows);
    }
  } else if (command === "K") {
    screen[row] = Array.from({ length: cols }, blankCell);
  } else if (command === "L") {
    const count = numbers[0] || 1;
    for (let i = 0; i < count; i += 1) {
      screen.splice(row, 0, Array.from({ length: cols }, blankCell));
      screen.pop();
    }
  } else if (command === "M") {
    const count = numbers[0] || 1;
    for (let i = 0; i < count; i += 1) {
      screen.splice(row, 1);
      screen.push(Array.from({ length: cols }, blankCell));
    }
  } else if (command === "h" && params === "1049") {
    screen = makeScreen(cols, rows);
    row = 0;
    col = 0;
  }

  return { screen, row, col, style };
}

function applySgr(codes, style) {
  if (codes.length === 0) {
    return blankCell();
  }
  let next = { ...style };
  for (let index = 0; index < codes.length; index += 1) {
    const code = codes[index];
    if (code === 0) next = blankCell();
    else if (code === 1) next.bold = true;
    else if (code === 2) next.dim = true;
    else if (code === 22) {
      next.bold = false;
      next.dim = false;
    } else if (code === 32) next.fg = "#22c55e";
    else if (code === 33) next.fg = "#eab308";
    else if (code === 39) next.fg = DEFAULT_FG;
    else if (code === 38 && codes[index + 1] === 2) {
      next.fg = rgb(codes[index + 2], codes[index + 3], codes[index + 4]);
      index += 4;
    } else if (code === 48 && codes[index + 1] === 2) {
      next.bg = rgb(codes[index + 2], codes[index + 3], codes[index + 4]);
      index += 4;
    }
  }
  return next;
}

function rgb(r, g, b) {
  return `#${[r, g, b]
    .map((value) => Math.max(0, Math.min(255, value || 0)).toString(16).padStart(2, "0"))
    .join("")}`;
}

function trimBlankRows(screen) {
  while (screen.length > 8 && isBlankRow(screen[screen.length - 1])) {
    screen.pop();
  }
}

function isBlankRow(row) {
  return row.every((cell) => cell.ch === " " && cell.bg === DEFAULT_BG);
}

function renderSvg(screen, title) {
  const width = Math.round(PADDING_X * 2 + COLS * CHAR_WIDTH);
  const height = Math.round(PADDING_Y * 2 + screen.length * LINE_HEIGHT);
  const bodyY = PADDING_Y + 14;
  const parts = [
    `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}">`,
    `<rect width="100%" height="100%" rx="12" fill="#111827"/>`,
    `<rect x="8" y="8" width="${width - 16}" height="${height - 16}" rx="8" fill="${DEFAULT_BG}" stroke="#334155"/>`,
  ];

  for (let r = 0; r < screen.length; r += 1) {
    const row = screen[r];
    for (const run of styleRuns(row, "bg")) {
      if (run.value !== DEFAULT_BG) {
        parts.push(
          `<rect x="${PADDING_X + run.start * CHAR_WIDTH}" y="${bodyY + r * LINE_HEIGHT - 14}" width="${Math.ceil((run.end - run.start) * CHAR_WIDTH)}" height="${LINE_HEIGHT}" fill="${run.value}"/>`,
        );
      }
    }
    for (const run of textRuns(row)) {
      const text = row
        .slice(run.start, run.end)
        .map((cell) => cell.ch)
        .join("");
      if (text.trim().length === 0) continue;
      const opacity = run.style.dim ? "0.68" : "1";
      const weight = run.style.bold ? "700" : "500";
      parts.push(
        `<text xml:space="preserve" x="${PADDING_X + run.start * CHAR_WIDTH}" y="${bodyY + r * LINE_HEIGHT}" fill="${run.style.fg}" opacity="${opacity}" font-family="Menlo, Monaco, Consolas, monospace" font-size="${FONT_SIZE}" font-weight="${weight}">${escapeXml(text)}</text>`,
      );
    }
  }

  parts.push("</svg>");
  return parts.join("\n");
}

function styleRuns(row, key) {
  const runs = [];
  let start = 0;
  let value = row[0][key];
  for (let i = 1; i <= row.length; i += 1) {
    if (i === row.length || row[i][key] !== value) {
      runs.push({ start, end: i, value });
      start = i;
      value = row[i]?.[key];
    }
  }
  return runs;
}

function textRuns(row) {
  const runs = [];
  let start = 0;
  let style = textStyle(row[0]);
  for (let i = 1; i <= row.length; i += 1) {
    if (i === row.length || !sameTextStyle(style, textStyle(row[i]))) {
      runs.push({ start, end: i, style });
      start = i;
      style = textStyle(row[i]);
    }
  }
  return runs;
}

function textStyle(cell = blankCell()) {
  return {
    fg: cell.fg,
    bold: cell.bold,
    dim: cell.dim,
  };
}

function sameTextStyle(left, right) {
  return left.fg === right.fg && left.bold === right.bold && left.dim === right.dim;
}

function escapeXml(value) {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

async function renderPng(svg, pngPath) {
  let sharp;
  try {
    sharp = require("sharp");
  } catch {
    console.warn("sharp is not available; wrote SVG only");
    return;
  }
  await sharp(Buffer.from(svg)).png().toFile(pngPath);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
