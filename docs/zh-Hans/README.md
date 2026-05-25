# situs-cli

[![CI](https://github.com/toddkim95/situs/actions/workflows/ci.yml/badge.svg)](https://github.com/toddkim95/situs/actions/workflows/ci.yml)
[![Security](https://github.com/toddkim95/situs/actions/workflows/security.yml/badge.svg)](https://github.com/toddkim95/situs/actions/workflows/security.yml)

[English](../../README.md) | [한국어](../ko/README.md) | [简体中文](README.md) | [Español](../es/README.md) | [日本語](../ja/README.md)

`situs` 是一个面向 zsh 的小型 **command cwd resolver**。

它会记住某条命令曾经在哪个目录成功运行，然后让你不用手动来回 `cd`，就能
从记住的目录运行或暂存这条命令。

> 本文是简体中文译文。英文 [README.md](../../README.md) 是 source of truth。
> 命令、flag、环境变量、配置 key、shell protocol 值会故意保持英文。

```text
~/notes
> cargo build
  按 Ctrl-G

Situs 会打开 compact picker:
  cargo build --release        .../work/app        ok        2h ago
> cargo build
  esc quit  up/down select  tab cd  enter run
```

为什么需要它:

1. 你在 `/Users/me/work/app` 中成功运行了 `cargo build --release`。
2. 之后你在另一个目录输入 `cargo build --release`。
3. 按下 Situs key binding。
4. 选择之前成功过的目录。
5. Situs 把 shell line 变成 `cd -- /Users/me/work/app && cargo build --release`。

Situs 不是完整的 shell history 替代品。Atuin、McFly、fzf、HSTR 都是优秀的
history searcher；zoxide 是优秀的 directory jumper。Situs 只专注一个很窄的
问题：“这条命令以前在哪里成功过？”

## 截图

### Inline Picker

| Search | Inspect | Help |
| --- | --- | --- |
| ![在固定 query line 上方显示近期 cargo 命令的 inline search picker](../assets/screenshots/inline-search.svg) | ![显示选中命令、cwd、status、source、runs、actions 的 inline inspect view](../assets/screenshots/inline-inspect.svg) | ![显示 Situs keyboard shortcuts 的 inline help view](../assets/screenshots/inline-help.svg) |

### Fullscreen Picker

| Search | Inspect | Help |
| --- | --- | --- |
| ![在更大空间中展示同一 command cwd resolver workflow 的 fullscreen search picker](../assets/screenshots/fullscreen-search.svg) | ![显示 command metadata 的 fullscreen inspect view](../assets/screenshots/fullscreen-inspect.svg) | ![显示 keyboard shortcuts 的 fullscreen help view](../assets/screenshots/fullscreen-help.svg) |

## 功能

- 记录 command、cwd、exit status、timestamp 和 source。
- 默认优先显示成功的 command run。
- 打开 compact inline picker，并保持当前 command line 可见。
- 需要更大界面时支持 fullscreen TUI picker。
- `Tab` 会暂存选中的目录和命令，但不会运行。
- `Enter` 会 cd 到选中的目录，并运行选中的 history command。
- 可以从 exact command 扩展到有用的 partial command，例如 `cargo install`、
  `cargo install --path`、`cargo install --path .`。
- 可按 local history、Atuin history、当前目录或当前 git workspace 过滤。
- 可 read-only 导入 Atuin SQLite history。
- 为 non-TTY 和脚本场景保留 plain line-based picker。
- 支持 macOS 和 Linux 上的 `zsh`、`bash` 和 `fish` 运行环境。

## 安装

### 从 GitHub 安装

仓库公开后:

```sh
cargo install --git https://github.com/toddkim95/situs
```

如果仓库最终使用不同 owner 或名称，请替换为最终 GitHub URL。

### 从 Local Checkout 安装

```sh
git clone https://github.com/toddkim95/situs
cd situs
cargo install --path .
```

### 从 crates.io 安装

crate 发布后:

```sh
cargo install situs-cli
```

## 快速开始

把 Situs 加到 zsh:

```sh
eval "$(situs init zsh)"
```

把同一行放到 `~/.zshrc` 靠近末尾的位置，然后打开新的终端。

默认 binding 是 `Ctrl-G`。可以在加载 init script 前修改:

```sh
export SITUS_BINDKEY='^G'
eval "$(situs init zsh)"
```

运行诊断:

```sh
situs doctor
```

打印 picker shortcuts:

```sh
situs keymap
```

使用 guided setup flow:

```sh
situs setup
```

更多安装说明见 [docs/installation.md](../installation.md)。

## 日常使用

像平时一样运行命令。zsh integration 会在 interactive command 结束后记录:

```sh
cd ~/work/app
cargo test
```

之后在任何目录:

```sh
cargo test
# 按 Ctrl-G
```

在 picker 中:

| Key | Action |
| --- | --- |
| `Up` / `Down` | 选择 history row，并把 query 同步为选中的 command |
| `Left` / `Right` | 移动 query cursor |
| `Tab` | `cd` 到选中目录，并把 command 保留在 shell buffer |
| `Enter` | `cd` 到选中目录，并运行选中的 history command |
| `Ctrl-F` | 切换失败 command history |
| `Ctrl-O` | Inspect 选中的 history item |
| `F2` | 循环 source filter: all, local, Atuin |
| `F3` | 循环 context filter: all, directory, workspace |
| `Ctrl-Y` | Copy 选中的 command |
| `Ctrl-D` | Delete 选中的 Situs history row |
| `Esc` | 退出并保留原始 shell input |

完整用法见 [docs/usage.md](../usage.md)。

## Picker Modes

默认 inline picker:

```sh
situs choose --picker inline --command "cargo build"
```

Fullscreen picker:

```sh
situs choose --picker fullscreen --command "cargo build"
```

设置 fullscreen 为默认:

```sh
export SITUS_PICKER=fullscreen
```

或者运行:

```sh
situs setup
```

当多个可见 row 共享同一个 directory prefix 时，Situs 会用 `*` 隐藏公共前缀，
让真正有区别的 path segment 更容易扫描。实际选中的目录仍然是完整 path。

## Atuin

Situs 可以在不修改 Atuin database 的情况下导入 history:

```sh
situs import atuin
```

在搜索前启用自动 read-only import:

```sh
situs atuin enable
```

检查状态或禁用:

```sh
situs atuin status
situs atuin disable
```

Atuin integration 详情见 [docs/configuration.md](../configuration.md)。

## Commands

```sh
situs init zsh
situs setup
situs doctor
situs keymap
situs atuin enable
situs atuin status
situs import atuin
situs record --cwd "$PWD" --status 0 -- "cargo build"
situs choose --picker fullscreen --mode restore --command "cargo build"
situs choose --context workspace --command "cargo test"
situs choose --print-dir --command "cargo build"
situs run -- cargo build
situs stats
```

运行 `situs --help` 查看完整 command summary。

## 配置

常用环境变量:

| Variable | Purpose |
| --- | --- |
| `SITUS_BINDKEY` | zsh key binding, default `^G` |
| `SITUS_MODE` | zsh execution mode: `stay` or `restore` |
| `SITUS_PICKER` | picker mode: `inline` or `fullscreen` |
| `SITUS_INLINE_ROWS` | inline picker row count |
| `SITUS_HISTORY` | history file path override |
| `SITUS_CONFIG` | config file path override |
| `SITUS_ATUIN_SYNC` | Atuin sync override: `off`, `auto`, or `always` |
| `SITUS_LANG` | UI language: `en`, `ko`, `zh-Hans`, `es`, or `ja` |
| `SITUS_PLAIN` | 使用 simple line-based picker |

storage path、config file value、execution mode 详情见
[docs/configuration.md](../configuration.md)。

## 对比

| Tool | Main job | Situs's relationship |
| --- | --- | --- |
| Atuin | 丰富的 shell history、context、sync | Situs 可以导入 Atuin，并提供更小的 cwd resolver workflow |
| McFly | smart shell history search | Situs 解析你已经开始输入的 command 应该在哪个 cwd 运行 |
| fzf | general fuzzy finder 和 shell key bindings | Situs 有专门用途的 picker 和 shell protocol |
| zoxide | directory jumping | Situs 基于 command history 跳转，而不是 directory frequency |
| HSTR | shell history suggest box | Situs 把 command、cwd、status、action semantics 放在一起 |

更长的对比见 [docs/comparison.md](../comparison.md)。

## 开发

在本地运行完整的验证矩阵（格式化、clippy、单元/验收测试、文档翻译和 PTY 冒烟测试）：

```sh
scripts/verify-all.sh
```

您也可以运行单独的步骤：

```sh
cargo fmt -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
cargo package --locked --no-verify
scripts/verify-doc-i18n.sh
scripts/verify-picker-modes.sh
```

RustSec advisory audit 会在 GitHub Actions 中运行。要在本地检查:

```sh
cargo install cargo-audit --locked
cargo audit
```

更多贡献指南见 [CONTRIBUTING.md](../../CONTRIBUTING.md) 和
[docs/development.md](../development.md)。

新增或修改 user-facing feature 时，请更新 English、Korean、Simplified Chinese
的 i18n message coverage，或者在同一变更中记录明确的 fallback。

重新生成 README screenshots:

```sh
scripts/capture-screenshots.js
```

screenshot script 使用 `fixtures/screenshot-history.tsv` 的 mock history 运行真实
picker，并最多重试每次 capture 三次。

## 故障排查

先运行:

```sh
situs doctor
```

常见修复:

- 确认 `eval "$(situs init zsh)"` 已在 `~/.zshrc` 中加载。
- 修改 `SITUS_BINDKEY`、`SITUS_PICKER` 或 `SITUS_MODE` 后打开新的 shell。
- 使用 `cargo install --path . --force` 重新安装后，运行 `source ~/.zshrc` 或打开新
  terminal，让已经加载的 zsh widget 刷新。
- 使用 `situs stats` 确认 history 正在记录。
- 如果 Atuin 结果没有出现，运行 `situs atuin status`。
- 设置 `SITUS_PLAIN=1` 来隔离 terminal rendering 问题。

更多情况见 [docs/troubleshooting.md](../troubleshooting.md)。

## 贡献

欢迎 bug report、UX note 和小而集中的 pull request。Picker 变更需要 unit
coverage 和 zsh/PTY smoke coverage，因为很小的 terminal protocol 变化也可能破坏
真实 shell workflow。

提交 pull request 前请阅读 [CONTRIBUTING.md](../../CONTRIBUTING.md)。

## 安全

请不要为安全敏感报告创建 public issue。见 [SECURITY.md](../../SECURITY.md)。

## License

MIT。见 [LICENSE](../../LICENSE)。
