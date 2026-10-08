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
