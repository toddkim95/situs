"use strict";

const fs = require("node:fs");
const path = require("node:path");

function writeMockHistory({ fixturePath, outputPath, root }) {
  const now = Math.floor(Date.now() / 1000);
  const rows = fs
    .readFileSync(fixturePath, "utf8")
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter((line) => line.length > 0 && !line.startsWith("#"))
    .map((line, index) => {
      const fields = line.split("\t");
      if (fields.length !== 5) {
        const label = root ? path.relative(root, fixturePath) : fixturePath;
        throw new Error(`${label}:${index + 1} must have 5 tab-separated fields`);
      }
      const [secondsAgo, status, cwd, command, source] = fields;
      return [now - Number.parseInt(secondsAgo, 10), Number.parseInt(status, 10), cwd, command, source];
    });

  const history = rows
    .map(([timestamp, status, cwd, command, source]) =>
      ["v2", timestamp, status, encodeField(cwd), encodeField(command), source].join("\t"),
    )
    .join("\n")
    .concat("\n");
  fs.writeFileSync(outputPath, history, "utf8");
}

function encodeField(value) {
  return Buffer.from(value, "utf8")
    .toString("hex")
    .match(/../g)
    .map((hex) => {
      const byte = Number.parseInt(hex, 16);
      const char = String.fromCharCode(byte);
      return /[A-Za-z0-9 /._:+-]/.test(char) ? char : `%${hex.toUpperCase()}`;
    })
    .join("");
}

module.exports = {
  encodeField,
  writeMockHistory,
};
