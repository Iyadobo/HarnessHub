# HarnessHub

A native terminal launch desk for popular coding harnesses, built on the Herdr
runtime. Start by choosing a **model provider**, then choose a compatible harness.
Work runs in real Herdr panes, with the original terminal UI, status detection,
workspaces, detach/reattach, and session controls.

There is no web interface, Electron application, or browser service.

## Start

Install [Herdr](https://herdr.dev/docs/install) and Rust, then build this fork's
native launcher:

```sh
cargo build -p harnesshub --release --locked
cargo run -p harnesshub --release --locked
```

Or run `target/release/harnesshub` (`harnesshub.exe` on Windows) in a terminal.
On Windows, `HarnessHub.cmd` opens the same native launcher and builds it first
when the binary is missing.
`HARNESSHUB_HERDR` can point at a Herdr executable outside PATH. The launcher also
finds Herdr beside itself or in a checkout-local `.local/herdr/` directory.

The first screen selects a provider: existing harness sign-in, Anthropic, OpenAI,
Google AI Studio, OpenRouter, or a custom Chat Completions, Responses, or Anthropic
Messages endpoint. The next screen lists matching harnesses and their installation
status. Choose your project, then select a model from the provider's live catalog.
Type to filter long lists. Manual model entry and the harness default are also
available. If lookup fails, manual entry remains available. Existing harness login
keeps that harness's own model setup. The launcher then opens the Herdr terminal UI.

Saved profiles appear at the top of the first picker, followed by the **OpenCode
Go** preset and other providers. Every successful launch automatically saves its
provider, harness, model, endpoint, key-variable reference, and project folder.
Selecting a profile launches that setup again without repeating those questions.
Identical setups reuse one profile; the most recently used profile appears first.
Profiles contain no API keys. They live in `%LOCALAPPDATA%\HarnessHub\profiles.json`
on Windows or `$XDG_CONFIG_HOME/harnesshub/profiles.json` (`~/.config/harnesshub`
by default) on Unix.

The OpenCode Go preset selects OpenCode and its native `opencode-go` provider.
It uses `OPENCODE_API_KEY`, the official Go endpoint, and the installed OpenCode
model catalog. OpenCode keeps its built-in model-specific API formats and session
headers. A Go subscription/key is required; HarnessHub does not purchase a plan.

Run another `harnesshub` from another terminal to add another harness. Detach with
Herdr's `Ctrl+B`, then `q`; the named `harnesshub` session keeps running.

## Commands

```sh
harnesshub keys save --provider anthropic
harnesshub keys status --provider anthropic
harnesshub keys forget --provider anthropic
harnesshub providers
harnesshub profiles
harnesshub launch --profile 1
harnesshub profiles forget 1
harnesshub launch --provider opencode-go --model kimi-k3
harnesshub harnesses
harnesshub launch --provider openai --harness codex --cwd /path/to/project --model YOUR_MODEL
harnesshub launch --provider openrouter --harness opencode --model vendor/model --no-attach
harnesshub launch --provider compatible --harness aider --base-url https://provider.example/v1 --model YOUR_MODEL
harnesshub list
harnesshub attach
harnesshub stop w1:p1
```

Use `--session NAME` to isolate groups. Pane IDs come from `harnesshub list`.
`--profile ID` can also override `--model` or `--cwd`, creating another saved setup.
Removing a profile keeps its saved provider key.
`stop` ends the chosen pane process; it does not delete project files. Noninteractive
launches use an environment key or an already saved key, and use
`--no-attach` when no terminal is available.

## Harnesses and BYOK

| Harness | Provider adapter |
| --- | --- |
| Claude Code | Anthropic, Anthropic-compatible Messages gateway |
| Codex | OpenAI, Responses-compatible endpoint |
| OpenCode | Anthropic, OpenAI, OpenRouter, custom Chat or Responses endpoint |
| Gemini CLI | Google AI Studio |
| Aider | Anthropic, OpenAI, OpenRouter, custom Chat endpoint; model required |
| Pi | Anthropic, OpenAI, Google AI Studio, OpenRouter |
| Goose, Amp, GitHub Copilot CLI, Cursor Agent | Harness's own login/provider configuration |
| Terminal | Local shell for project work and setup |

Every harness also supports its existing authentication setup. Provider keys are
not interchangeable API protocols: an OpenAI Chat endpoint cannot be sent directly
to Claude Code or Codex. Unsupported combinations are rejected. This version uses
native provider settings; it does not claim a universal protocol-conversion proxy.
Custom OpenCode providers require a model ID. Aider models are provider-relative:
select Anthropic and pass `--model claude-model`; HarnessHub adds Aider's provider
prefix. OpenRouter model IDs retain their vendor prefix, such as `vendor/model`.

Default key variables: `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `GEMINI_API_KEY`,
`OPENROUTER_API_KEY`, or `HARNESSHUB_API_KEY`. Override the reference with
`--key-env NAME`. Environment keys take priority over saved keys. Masked entry
saves the key in the OS credential store (Windows Credential Manager, macOS
Keychain, or Linux Secret Service). `keys save` also imports an environment key.
Custom keys are scoped to provider, endpoint, and variable. `keys forget` removes
only HarnessHub's saved key. Keys are never accepted as command-line arguments or
stored in plaintext profiles, launch commands, or temporary files. A short-lived,
authenticated, single-use loopback connection passes it to the target terminal's
child environment. This is a local credential handoff, not a web service. A local
process running as your user can still inspect your processes and environment;
this is not a sandbox boundary. Harnesses retain their own security prompts and
may persist their own authentication or project state.

Detach preserves live processes. After a runtime/machine restart, the launcher
must launch BYOK sessions again; expired handoff tickets cannot restore keys.

Adapter references: [Claude gateways](https://code.claude.com/docs/en/llm-gateway),
[Codex provider configuration](https://developers.openai.com/codex/config-reference/),
[OpenCode providers](https://opencode.ai/docs/providers),
[Gemini authentication](https://geminicli.com/docs/get-started/authentication/),
[Aider keys](https://aider.chat/docs/config/api-keys.html),
[Pi](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent).

## Validation

```sh
cargo fmt --package harnesshub --check
cargo test -p harnesshub --locked
cargo clippy -p harnesshub --all-targets --locked -- -D warnings
```

The new launcher can build independently of the Ghostty terminal library.
Rebuilding the complete Herdr runtime also requires its original Zig 0.16.0
toolchain and platform build prerequisites. Upstream instructions remain below.

HarnessHub is an Apache-2.0 fork of [herdrdev/herdr](https://github.com/herdrdev/herdr).
The original runtime, license, and attribution are retained.

---

## Upstream Herdr


<p align="center">
  <img src="assets/logo.png" alt="herdr" width="100" />
</p>

<p align="center">
  <a href="https://herdr.dev">herdr.dev</a> · <a href="#install">install</a> · <a href="https://herdr.dev/docs/quick-start/">quick start</a> · <a href="https://herdr.dev/docs/">docs</a>
</p>

<p align="center">
  English · <a href="README.zh-CN.md">简体中文</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-666666?labelColor=333333" alt="Apache 2.0 license" /></a>
  <a href="https://github.com/herdrdev/herdr/releases"><img src="https://img.shields.io/github/downloads/herdrdev/herdr/total?labelColor=333333&color=666666" alt="total GitHub release downloads" /></a>
  <a href="https://github.com/herdrdev/herdr/stargazers"><img src="https://img.shields.io/github/stars/herdrdev/herdr?labelColor=333333&color=666666&logo=github" alt="GitHub stars" /></a>
  <a href="https://github.com/herdrdev/herdr/releases/latest"><img src="https://img.shields.io/github/v/release/herdrdev/herdr?label=release&labelColor=333333&color=666666" alt="latest stable release" /></a>
  <a href="https://formulae.brew.sh/formula/herdr"><img src="https://img.shields.io/homebrew/v/herdr?label=homebrew&labelColor=333333&color=666666" alt="Homebrew version" /></a>
  <a href="https://x.com/herdrdev"><img src="https://img.shields.io/badge/follow-%40herdrdev-000000?logo=x&logoColor=white" alt="follow @herdrdev on X" /></a>
</p>

---

https://github.com/user-attachments/assets/043ec09f-4bdd-41d5-aee0-8fda6b83e267

**the runtime your coding agents live on.**

- **detach without stopping work** — herdr keeps terminals running in a background server when you close the client or lose your SSH connection. after a server or machine restart, herdr restores the saved layout and can resume supported agent sessions; the original processes do not survive. [session state →](https://herdr.dev/docs/session-state/)
- **several machines, one window** — keep local work and saved ssh machines together, with a combined agent list and independent reconnects. [remote machines →](https://herdr.dev/docs/connecting-machines/)
- **never hunt for the stuck one** — every pane is marked working, blocked, or idle. when an agent stops and needs an answer, herdr says so.
- **agent-native** — agents drive herdr through the cli and socket api: they can spawn panes, prompt each other, and wait until another agent is genuinely blocked. [agent skill →](https://herdr.dev/docs/agent-skill/)
- **runs what you already run** — claude code, codex, cursor, opencode, grok and the rest. herdr doesn't wrap or replace them; it owns their terminals. building an agent? [add herdr support →](https://herdr.dev/docs/add-herdr-support/)
- **keyboard and mouse, both first-class** — tmux-style prefix keys *and* click, drag, split. pick per moment, not per tool.
- **plugins** — extend panes and workflows. [browse the marketplace →](https://herdr.dev/plugins/)
- **one rust binary, no electron** — runs in whatever terminal you already use.

---

## install

```bash
curl -fsSL https://herdr.dev/install.sh | sh
```

or `brew install herdr` · `mise use -g herdr` · windows: `powershell -ExecutionPolicy Bypass -c "irm https://herdr.dev/install.ps1 | iex"` · [endpoint-protected Windows](https://herdr.dev/docs/windows-beta/) · [binaries](https://github.com/herdrdev/herdr/releases)

then start it where the work lives:

```bash
herdr
```

run your agents, split panes, walk away. `ctrl+b q` detaches, `herdr` reattaches. [quick start →](https://herdr.dev/docs/quick-start/)

## docs

everything lives at [herdr.dev/docs](https://herdr.dev/docs/): [quick start](https://herdr.dev/docs/quick-start/) · [concepts](https://herdr.dev/docs/concepts/) · [supported agents](https://herdr.dev/docs/agents/) · [keyboard](https://herdr.dev/docs/keyboard/) · [configuration](https://herdr.dev/docs/configuration/) · [session state](https://herdr.dev/docs/session-state/) · [connecting machines](https://herdr.dev/docs/connecting-machines/) · [remote](https://herdr.dev/docs/persistence-remote/) · [integrations](https://herdr.dev/docs/integrations/) · [add herdr support to your agent](https://herdr.dev/docs/add-herdr-support/) · [plugins](https://herdr.dev/docs/plugins/) · [socket api](https://herdr.dev/docs/socket-api/)

## thanks

every past sponsor and backer is listed in [SPONSORS.md](./SPONSORS.md) — thank you 🐑

enterprise / partnership: hey@herdr.dev

## agent instructions

if you are an ai agent helping with this repository, read [`AGENTS.md`](./AGENTS.md) before making changes and read [`CONTRIBUTING.md`](./CONTRIBUTING.md) before opening issues or PRs.

## development

```bash
git clone https://github.com/herdrdev/herdr
cd herdr
cargo build --release

just test        # unit tests
just check       # formatting, tests, and maintenance checks
```

## license

Herdr is licensed under the [Apache License 2.0](LICENSE).
