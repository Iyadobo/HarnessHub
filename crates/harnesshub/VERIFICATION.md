# HarnessHub verification — 2026-10-08

Environment: Windows x86_64, Rust 1.96.1, upstream Herdr 0.9.3.

## Observed

- Native Rust launcher built successfully, without the Ghostty dependency.
- 15 unit tests passed: adapter/protocol matching, model/endpoint validation,
  provider environment mapping, no key in arguments, authenticated single-use
  credential handoff, executable lookup, shell quoting, and session bounds.
- Launcher formatting passed. Clippy passed with warnings denied.
- Real Herdr pane displayed the native provider picker. Selecting OpenAI displayed
  the matching Codex, OpenCode, Aider, and Pi choices with current installation
  status. Escape returned to the shell.
- Native launcher created a Terminal workspace. Input executed in that process;
  recent terminal output contained the standalone `HARNESSHUB_NATIVE_OK` result.
- Native launcher started the installed Codex CLI through its Windows npm shim.
  The real Codex project trust screen appeared, and foreground process inspection
  identified its Node entry point. No trust prompt was bypassed and no AI task
  was submitted.
- Session list returned actual Herdr pane IDs and state. Stop removed the test
  Terminal and Codex panes. The disposable test runtime was stopped afterward.

## Limits

- No paid model request was made. BYOK mappings were validated in tests; provider
  billing, model availability, tool-call support, and live inference remain
  unverified for each provider/model pair.
- Account-only harnesses use their own authentication/configuration. No universal
  Chat/Responses/Messages protocol proxy is claimed.
- Full upstream `just check` was attempted and stopped at its Ghostty build:
  Zig 0.16.0 was not installed. The fork's Herdr runtime source was not changed.
  Runtime tests above used the official Herdr 0.9.3 Windows distribution.
- Windows execution was checked. Linux/macOS paths have not been run here.
- Credential handoff acceptance proves that the target runner obtained the launch
  specification, not that the harness authenticated or finished loading.
- Session restore cannot replay a spent credential ticket after a machine restart;
  BYOK launches must be started again.

The discarded browser prototype and its Node/package files are absent from the
committed implementation. HarnessHub is a terminal application.

## Saved keys, model lists, and installed harnesses follow-up

- 18 regular tests passed, plus the explicitly run real OS credential-store test.
  Windows Credential Manager saved, loaded, isolated by endpoint, and deleted an
  isolated fake key. Formatting, warnings-denied Clippy, and release build passed.
- Separate CLI processes saved a fake custom-provider key, read its status, and
  launched using that saved key after the environment variable was removed.
  `keys forget` removed it; subsequent status returned `none`.
- A local model endpoint verified Bearer authentication and returned two fixture
  models. The real terminal picker displayed both; typing `small` filtered to
  `fixture-code-small`. Escape exited. No inference request was made.
- Live model discovery supports OpenAI/OpenRouter-compatible, Anthropic, and
  Gemini response shapes; Gemini excludes embedding-only models. Anthropic and
  Gemini pagination is supported. External provider discovery was not exercised
  with real credentials. HTTP errors fall back to manual/default choices.
- Installed Pi 1.1.0 with its current official npm package and Aider 0.86.2 with
  uv's isolated Python environment. Claude Code 2.1.282 was already installed.
  All three launched inside disposable Herdr panes: Claude displayed its project
  trust prompt, Pi displayed its interactive editor, Aider displayed its git and
  provider setup prompts. No trust prompt was accepted or AI task submitted.
  The disposable runtime was stopped afterward.
- Pi's current CLI requires a model alongside a provider; its BYOK picker now
  requires model selection. Claude native login offers stable model aliases.
- Cross-platform OS credential stores were not executed here. Secure-store
  errors are reported; there is no plaintext fallback.

## OpenCode Go and auto-saved profiles

- 21 regular tests passed, one credential-store integration test remained ignored
  in the regular run. Formatting, warnings-denied Clippy, and release build passed.
- Profile storage round trip, deduplication, removal, and preservation of corrupt
  files were checked. Writes use a process lock and atomic file replacement.
- Native Terminal launches automatically saved a profile. The real startup picker
  displayed it alongside the OpenCode Go preset. Selecting it relaunched without
  setup questions. CLI `launch --profile 1` also relaunched successfully; repeated
  launches kept one entry. Profiles contained only configuration/key references.
- The installed OpenCode 1.18.34 returned its Go model catalog. HarnessHub's model
  picker displayed it, filtered `kimi-k3`, and launched a disposable Herdr pane
  whose real OpenCode TUI displayed `Build · Kimi K3 OpenCode Go`.
- Go config uses OpenCode's native `opencode-go` provider and an environment key
  reference, preserving built-in model SDK/protocol metadata. No custom provider
  or universal protocol proxy was substituted.
- A Windows PowerShell 5 argument bug was found by this runtime check: wrapping
  ConvertFrom-Json in `@(...)` nested its argument array. Fixed argument splatting;
  a real PowerShell script test verifies separate model flags and spaced arguments.
- Existing launcher processes held the old executable open. The new release was
  installed into checkout-local `.local/bin/harnesshub.exe`; both the global shim
  and root Windows launcher use it. Existing user sessions were left running.
- Test profiles and named test runtimes were removed/stopped. The Go startup
  check used a fake key, sent no AI prompt, and does not establish subscription
  authentication or paid inference. Public Go model metadata was fetched without
  credentials; the preset's actual picker uses the installed OpenCode catalog.
