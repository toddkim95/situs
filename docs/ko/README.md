# situs-cli

[![CI](https://github.com/toddkim95/situs/actions/workflows/ci.yml/badge.svg)](https://github.com/toddkim95/situs/actions/workflows/ci.yml)
[![Security](https://github.com/toddkim95/situs/actions/workflows/security.yml/badge.svg)](https://github.com/toddkim95/situs/actions/workflows/security.yml)

[English](../../README.md) | [한국어](README.md) | [简体中文](../zh-Hans/README.md) | [Español](../es/README.md) | [日本語](../ja/README.md)

`situs`은 zsh를 위한 작은 **command cwd resolver**입니다.

명령이 예전에 어디에서 성공했는지 기억해 두었다가, 수동으로 여기저기
`cd`하지 않고도 그 디렉터리에서 명령을 실행하거나 shell 입력줄에 준비할
수 있게 해줍니다.

> 이 문서는 한국어 번역본입니다. 영어 [README.md](../../README.md)가
> source of truth입니다. 명령어, flag, 환경 변수, 설정 key, shell protocol
> 값은 의도적으로 번역하지 않습니다.

```text
~/notes
> cargo build
  Ctrl-G 누르기

Situs이 compact picker를 엽니다:
  cargo build --release        .../work/app        ok        2h ago
> cargo build
  esc quit  up/down select  tab cd  enter run
```

왜 필요한가요:

1. `/Users/me/work/app`에서 `cargo build --release`를 성공적으로 실행합니다.
2. 나중에 다른 디렉터리에서 `cargo build --release`를 입력합니다.
3. Situs key binding을 누릅니다.
4. 이전에 성공했던 디렉터리를 고릅니다.
5. Situs이 shell line을 `cd -- /Users/me/work/app && cargo build --release`로 바꿉니다.

Situs은 전체 shell history 대체품이 아닙니다. Atuin, McFly, fzf, HSTR은
훌륭한 history searcher이고, zoxide는 훌륭한 directory jumper입니다. Situs은
하나의 좁은 문제에 집중합니다: "이 명령이 예전에 어디에서 동작했지?"

## 스크린샷

### Inline Picker

| Search | Inspect | Help |
| --- | --- | --- |
| ![고정 query line 위에 최근 cargo 명령들이 보이는 inline search picker](../assets/screenshots/inline-search.svg) | ![선택된 명령, cwd, status, source, runs, actions를 보여주는 inline inspect view](../assets/screenshots/inline-inspect.svg) | ![Situs keyboard shortcuts를 보여주는 inline help view](../assets/screenshots/inline-help.svg) |

### Fullscreen Picker

| Search | Inspect | Help |
| --- | --- | --- |
| ![더 넓은 화면에서 같은 command cwd resolver workflow를 보여주는 fullscreen search picker](../assets/screenshots/fullscreen-search.svg) | ![command metadata를 보여주는 fullscreen inspect view](../assets/screenshots/fullscreen-inspect.svg) | ![keyboard shortcuts를 보여주는 fullscreen help view](../assets/screenshots/fullscreen-help.svg) |

## 기능

- command, cwd, exit status, timestamp, source를 기억합니다.
- 기본적으로 성공한 command run을 우선합니다.
- 현재 command line을 유지하는 compact inline picker를 엽니다.
- 더 넓은 화면이 필요할 때 fullscreen TUI picker를 지원합니다.
- `Tab`으로 선택한 디렉터리와 명령을 실행하지 않고 shell buffer에 준비합니다.
- `Enter`로 선택한 디렉터리로 `cd`한 뒤 선택한 history command를 실행합니다.
- `cargo install`, `cargo install --path`, `cargo install --path .`처럼 유용한
  partial command로 matching을 넓힙니다.
- local history, Atuin history, 현재 디렉터리, 현재 git workspace로 필터링합니다.
- Atuin SQLite history를 read-only로 import할 수 있습니다.
- non-TTY와 scripting을 위한 plain line-based picker도 유지합니다.
- macOS 및 Linux 환경의 `zsh`, `bash`, `fish` 쉘을 지원합니다.

## 설치

### GitHub에서 설치

이 repository가 공개된 뒤:

```sh
cargo install --git https://github.com/toddkim95/situs
```

repository가 다른 owner/name으로 공개되면 최종 GitHub URL로 바꾸세요.

### Local Checkout에서 설치

```sh
git clone https://github.com/toddkim95/situs
cd situs
cargo install --path .
```

### crates.io에서 설치

crate가 공개된 뒤:

```sh
cargo install situs-cli
```

## 빠른 시작

zsh에 Situs을 추가합니다:

```sh
eval "$(situs init zsh)"
```

같은 줄을 `~/.zshrc` 끝부분 근처에 넣고 새 터미널을 여세요.

기본 binding은 `Ctrl-G`입니다. init script를 로드하기 전에 바꿀 수 있습니다:

```sh
export SITUS_BINDKEY='^G'
eval "$(situs init zsh)"
```

진단 실행:

```sh
situs doctor
```

picker shortcut 출력:

```sh
situs keymap
```

guided setup flow:

```sh
situs setup
```

자세한 설치 내용은 [docs/installation.md](../installation.md)에 있습니다.

## 일상 사용

평소처럼 명령을 실행하면 됩니다. zsh integration은 interactive command가
끝난 뒤 기록합니다:

```sh
cd ~/work/app
cargo test
```

나중에 어느 디렉터리에서든:

```sh
cargo test
# Ctrl-G 누르기
```

Picker 안에서:

| Key | Action |
| --- | --- |
| `Up` / `Down` | history row를 선택하고 query를 선택한 command와 동기화 |
| `Left` / `Right` | query cursor 이동 |
| `Tab` | 선택한 디렉터리로 `cd`하고 command를 shell buffer에 유지 |
| `Enter` | 선택한 디렉터리로 `cd`하고 선택한 history command 실행 |
| `Alt-Enter` / `Alt-Y` | 디렉터리를 바꾸거나 실행하지 않고 선택한 command를 shell buffer에 붙여넣기 |
| `Ctrl-F` | 실패한 command history toggle |
| `Ctrl-O` | 선택한 history item inspect |
| `F2` | source filter 순환: all, local, Atuin |
| `F3` | context filter 순환: all, directory, workspace |
| `Ctrl-Y` | 선택한 command copy |
| `Ctrl-D` | 선택한 Situs history row delete |
| `Esc` | 종료하고 원래 shell input 유지 |

전체 사용법은 [docs/usage.md](../usage.md)에 있습니다.

## Picker Modes

기본값인 inline picker:

```sh
situs choose --picker inline --command "cargo build"
```

Fullscreen picker:

```sh
situs choose --picker fullscreen --command "cargo build"
```

fullscreen을 기본값으로 만들기:

```sh
export SITUS_PICKER=fullscreen
```

또는:

```sh
situs setup
```

여러 row가 같은 directory prefix를 공유하면 Situs은 그 공통 prefix를 `*`로
표시해서 중요한 path segment를 더 쉽게 볼 수 있게 합니다. 실제 선택되는
디렉터리는 여전히 전체 path입니다.

## Atuin

Situs은 Atuin database를 수정하지 않고 history를 가져올 수 있습니다:

```sh
situs import atuin
```

검색 전 자동 read-only import 활성화:

```sh
situs atuin enable
```

상태 확인 또는 비활성화:

```sh
situs atuin status
situs atuin disable
```

Atuin integration detail은 [docs/configuration.md](../configuration.md)에 있습니다.

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

전체 command summary는 `situs --help`를 실행하세요.

## 설정

자주 쓰는 환경 변수:

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
| `SITUS_PLAIN` | simple line-based picker 사용 |

storage path, config file value, execution mode detail은
[docs/configuration.md](../configuration.md)를 보세요.

## 비교

| Tool | Main job | Situs's relationship |
| --- | --- | --- |
| Atuin | 풍부한 shell history, context, sync | Situs은 Atuin을 import할 수 있고 더 작은 cwd resolver workflow를 제공합니다 |
| McFly | smart shell history search | Situs은 이미 입력하기 시작한 command의 cwd를 해결합니다 |
| fzf | general fuzzy finder와 shell key bindings | Situs은 목적이 분명한 picker와 shell protocol을 갖습니다 |
| zoxide | directory jumping | Situs은 directory frequency가 아니라 command history 기반으로 이동합니다 |
| HSTR | shell history suggest box | Situs은 command, cwd, status, action semantics를 함께 유지합니다 |

더 긴 비교는 [docs/comparison.md](../comparison.md)에 있습니다.

## 개발

로컬에서 전체 검증 매트릭스(포맷 체크, clippy, 단위/인수 테스트, 번역, PTY 연동 테스트)를 한 번에 실행합니다:

```sh
scripts/verify-all.sh
```

개별 단계를 실행할 수도 있습니다:

```sh
cargo fmt -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
cargo package --locked --no-verify
scripts/verify-doc-i18n.sh
scripts/verify-picker-modes.sh
```

RustSec advisory audit는 GitHub Actions에서 실행됩니다. 로컬에서 확인하려면:

```sh
cargo install cargo-audit --locked
cargo audit
```

기여 가이드는 [CONTRIBUTING.md](../../CONTRIBUTING.md)와
[docs/development.md](../development.md)에 있습니다.

user-facing feature를 추가하거나 바꾸면 English, Korean, Simplified Chinese의
i18n message coverage를 갱신하거나, 같은 변경에서 명시적인 fallback을 문서화하세요.

README screenshot 재생성:

```sh
scripts/capture-screenshots.js
```

screenshot script는 `fixtures/screenshot-history.tsv`의 mock history로 실제 picker를
실행하며, 각 capture를 최대 세 번 재시도합니다.

## 문제 해결

먼저:

```sh
situs doctor
```

자주 필요한 조치:

- `eval "$(situs init zsh)"`가 `~/.zshrc`에 로드되어 있는지 확인하세요.
- `SITUS_BINDKEY`, `SITUS_PICKER`, `SITUS_MODE`를 바꾼 뒤에는 새 shell을 여세요.
- `cargo install --path . --force`로 재설치한 뒤에는 현재 terminal의 zsh widget이
  갱신되도록 `source ~/.zshrc`를 실행하거나 새 terminal을 여세요.
- history가 기록되는지 `situs stats`로 확인하세요.
- Atuin 결과가 보이지 않으면 `situs atuin status`를 실행하세요.
- terminal rendering 문제를 분리하려면 `SITUS_PLAIN=1`을 설정하세요.

더 많은 사례는 [docs/troubleshooting.md](../troubleshooting.md)에 있습니다.

## 기여

bug report, UX note, 작고 집중된 pull request를 환영합니다. Picker 변경은
terminal protocol의 작은 변화도 실제 shell workflow를 깨뜨릴 수 있으므로 unit
coverage와 zsh/PTY smoke coverage가 모두 필요합니다.

pull request를 열기 전에 [CONTRIBUTING.md](../../CONTRIBUTING.md)를 읽어주세요.

## 보안

보안 민감 이슈는 public issue로 열지 마세요. [SECURITY.md](../../SECURITY.md)를
참고하세요.

## 라이선스

MIT. [LICENSE](../../LICENSE)를 참고하세요.
