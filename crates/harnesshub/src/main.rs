mod catalog;
mod handoff;
mod platform;
mod ui;

use catalog::{HARNESSES, PROVIDERS};
use serde_json::Value;
use std::io::{self, IsTerminal};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

const HELP:&str="HarnessHub — provider-first terminal launcher on Herdr\n\n  harnesshub                          Pick provider, then harness; attach to Herdr\n  harnesshub launch [OPTIONS]         Launch a harness in its own project workspace\n  harnesshub providers               List model providers and API formats\n  harnesshub harnesses                List harnesses, installation, and BYOK support\n  harnesshub list                     List live panes and agent state\n  harnesshub attach                   Open the full Herdr TUI\n  harnesshub stop PANE_ID             Close that pane and stop its process\n\nLaunch options:\n  --provider ID    --harness ID    --model MODEL    --cwd PATH\n  --base-url URL   --key-env NAME  --session NAME   --no-attach\n\nKeys come from the selected provider environment variable, or masked terminal\nentry. There is no --api-key option and no credential file. API formats must\nmatch: Codex uses Responses; Claude uses Messages; chat endpoints use OpenCode\nor Aider. HARNESSHUB_HERDR selects a Herdr runtime outside PATH.\n";

#[derive(Default)]
struct Options {
    provider: Option<String>,
    harness: Option<String>,
    model: Option<String>,
    cwd: Option<String>,
    endpoint: Option<String>,
    key_env: Option<String>,
    session: String,
    no_attach: bool,
}
fn options(args: &[String]) -> Result<Options, String> {
    let mut o = Options {
        session: "harnesshub".into(),
        ..Options::default()
    };
    let mut i = 0;
    while i < args.len() {
        let flag = args[i].as_str();
        if flag == "--no-attach" {
            o.no_attach = true;
            i += 1;
            continue;
        }
        let target = match flag {
            "--provider" => &mut o.provider,
            "--harness" => &mut o.harness,
            "--model" => &mut o.model,
            "--cwd" => &mut o.cwd,
            "--base-url" => &mut o.endpoint,
            "--key-env" => &mut o.key_env,
            "--session" => {
                i += 1;
                o.session = args.get(i).ok_or("Missing session name.")?.clone();
                i += 1;
                continue;
            }
            _ => return Err(format!("Unknown option: {flag}")),
        };
        i += 1;
        *target = Some(
            args.get(i)
                .ok_or_else(|| format!("Missing value for {flag}"))?
                .clone(),
        );
        i += 1;
    }
    if o.session.is_empty()
        || o.session.len() > 64
        || !o
            .session
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err("Session name must be 1–64 letters, digits, underscores, or hyphens.".into());
    }
    Ok(o)
}
fn runtime() -> io::Result<PathBuf> {
    if let Some(path) = std::env::var_os("HARNESSHUB_HERDR") {
        return platform::executable(&path.to_string_lossy())
            .ok_or_else(|| io::Error::other("HARNESSHUB_HERDR is not an executable file."));
    }
    if let Some(path) = platform::executable("herdr") {
        return Ok(path);
    }
    let current = std::env::current_exe()?;
    for parent in current.ancestors().skip(1) {
        if let Some(path) = platform::executable(&parent.join("herdr").to_string_lossy()) {
            return Ok(path);
        }
        if let Some(path) =
            platform::executable(&parent.join(".local/herdr/herdr").to_string_lossy())
        {
            return Ok(path);
        }
    }
    Err(io::Error::other(
        "Install Herdr, or set HARNESSHUB_HERDR to its executable path.",
    ))
}
fn command(runtime: &PathBuf, session: &str) -> Command {
    let mut c = Command::new(runtime);
    c.args(["--session", session]);
    c.env_remove("HERDR_SOCKET_PATH")
        .env_remove("HERDR_CLIENT_SOCKET_PATH");
    c
}
fn api(runtime: &PathBuf, session: &str, args: &[&str]) -> io::Result<Value> {
    let mut c = command(runtime, session);
    c.args(args);
    platform::background(&mut c);
    let result = c.output()?;
    if !result.status.success() {
        return Err(io::Error::other(
            String::from_utf8_lossy(&result.stderr).trim().to_string(),
        ));
    }
    // Raw pane input commands acknowledge success with an empty stdout.
    if result.stdout.iter().all(u8::is_ascii_whitespace) {
        return Ok(Value::Null);
    }
    let response: Value = serde_json::from_slice(&result.stdout)
        .map_err(|_| io::Error::other("Herdr returned invalid API output."))?;
    if let Some(error) = response.get("error") {
        return Err(io::Error::other(
            error["message"]
                .as_str()
                .unwrap_or("Herdr rejected the action."),
        ));
    }
    Ok(response["result"].clone())
}
fn start(runtime: &PathBuf, session: &str) -> io::Result<()> {
    if api(runtime, session, &["api", "snapshot"]).is_ok() {
        return Ok(());
    }
    let mut c = command(runtime, session);
    c.arg("server")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    platform::background(&mut c);
    let mut child = c.spawn()?;
    for _ in 0..40 {
        std::thread::sleep(Duration::from_millis(200));
        if api(runtime, session, &["api", "snapshot"]).is_ok() {
            return Ok(());
        }
        if child.try_wait()?.is_some() {
            return Err(io::Error::other(
                "Herdr server exited before becoming ready. Check its named-session log.",
            ));
        }
    }
    Err(io::Error::other(
        "Herdr did not become ready. Check its named-session log.",
    ))
}
fn attach(runtime: &PathBuf, session: &str) -> io::Result<i32> {
    Ok(command(runtime, session).status()?.code().unwrap_or(1))
}
fn launch(o: Options) -> io::Result<i32> {
    let provider = if let Some(id) = &o.provider {
        PROVIDERS
            .iter()
            .find(|p| p.id == id)
            .ok_or_else(|| io::Error::other("Unknown provider. Run harnesshub providers."))?
    } else {
        let rows: Vec<_> = PROVIDERS
            .iter()
            .map(|p| {
                format!(
                    "{}{}",
                    p.name,
                    if p.key_env.is_empty() {
                        String::new()
                    } else {
                        format!("  [{}]", p.key_env)
                    }
                )
            })
            .collect();
        let Some(index) = ui::choose(
            " Model provider ",
            "Choose the provider whose key you want to use.",
            &rows,
        )?
        else {
            return Ok(0);
        };
        &PROVIDERS[index]
    };
    let available: Vec<_> = HARNESSES
        .iter()
        .filter(|h| h.providers.contains(&provider.id))
        .collect();
    let harness = if let Some(id) = &o.harness {
        HARNESSES
            .iter()
            .find(|h| h.id == id)
            .ok_or_else(|| io::Error::other("Unknown harness. Run harnesshub harnesses."))?
    } else {
        let rows: Vec<_> = available
            .iter()
            .map(|h| {
                format!(
                    "{}  {}",
                    h.name,
                    if h.id == "terminal" || platform::executable(h.command).is_some() {
                        "installed"
                    } else {
                        "not installed"
                    }
                )
            })
            .collect();
        let Some(index) = ui::choose(
            " Harness ",
            &format!(
                "Provider: {}. Only matching API adapters appear.",
                provider.name
            ),
            &rows,
        )?
        else {
            return Ok(0);
        };
        available[index]
    };
    if !harness.providers.contains(&provider.id) {
        return Err(io::Error::other(
            "This harness does not support the chosen API format.",
        ));
    }
    let harness_command = if harness.id == "terminal" {
        platform::shell()
    } else {
        harness.command.into()
    };
    if platform::executable(&harness_command).is_none() {
        return Err(io::Error::other(format!(
            "{} is not installed. Setup: {}",
            harness.name, harness.install
        )));
    }
    let interactive = io::stdin().is_terminal();
    let current = std::env::current_dir()?.to_string_lossy().into_owned();
    let cwd = match o.cwd {
        Some(p) => p,
        None if interactive => ui::text("Project folder", &current)?,
        None => current,
    };
    if !PathBuf::from(&cwd).is_dir() {
        return Err(io::Error::other("Project folder does not exist."));
    }
    let model = match o.model {
        Some(m) => m,
        None if interactive && harness.model_flag => {
            ui::text("Model ID (blank uses harness default)", "")?
        }
        None => String::new(),
    };
    let endpoint = match o.endpoint {
        Some(url) => url,
        None if interactive && ["compatible", "responses", "gateway"].contains(&provider.id) => {
            ui::text("API endpoint URL", "")?
        }
        None => provider.endpoint.into(),
    };
    let key_env = o.key_env.as_deref().unwrap_or(provider.key_env);
    let key = if provider.id == "native" {
        None
    } else {
        match std::env::var(key_env) {
            Ok(key) if !key.is_empty() => Some(key),
            _ => Some(ui::secret(key_env)?),
        }
    };
    let mut spec = catalog::build(
        harness,
        provider,
        &model,
        &endpoint,
        key_env,
        key,
        cwd.clone(),
    )
    .map_err(io::Error::other)?;
    spec.command = harness_command;
    let runtime = runtime()?;
    start(&runtime, &o.session)?;
    let handoff = handoff::serve(spec)?;
    let label = format!("{} / {}", harness.name, provider.id);
    let created = api(
        &runtime,
        &o.session,
        &[
            "workspace",
            "create",
            "--cwd",
            &cwd,
            "--label",
            &label,
            "--focus",
        ],
    )?;
    let pane = created["root_pane"]["pane_id"]
        .as_str()
        .ok_or_else(|| io::Error::other("Herdr did not return a pane ID."))?;
    let workspace = created["workspace"]["workspace_id"]
        .as_str()
        .ok_or_else(|| io::Error::other("Herdr did not return a workspace ID."))?;
    let launched = (|| {
        let mut ready = false;
        for _ in 0..40 {
            let info = api(
                &runtime,
                &o.session,
                &["pane", "process-info", "--pane", pane],
            )?;
            let shell = info["process_info"]["shell_pid"].as_u64();
            if let Some(processes) = info["process_info"]["foreground_processes"].as_array() {
                if !processes.is_empty() && processes.iter().all(|p| p["pid"].as_u64() == shell) {
                    ready = true;
                    break;
                }
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        if !ready {
            return Err(io::Error::other("New pane shell did not become ready."));
        }
        let executable = std::env::current_exe()?.to_string_lossy().into_owned();
        api(
            &runtime,
            &o.session,
            &[
                "pane",
                "run",
                pane,
                &platform::runner_command(&executable, &handoff.address, &handoff.token),
            ],
        )?;
        if !handoff
            .claimed
            .recv_timeout(Duration::from_secs(40))
            .unwrap_or(false)
        {
            return Err(io::Error::other(
                "Harness did not claim its launch ticket. Check the pane shell.",
            ));
        }
        println!(
            "Launched {} with {} in {pane}.",
            harness.name, provider.name
        );
        println!("Attach: harnesshub attach --session {}", o.session);
        Ok(())
    })();
    if let Err(error) = launched {
        let _ = api(&runtime, &o.session, &["workspace", "close", workspace]);
        return Err(error);
    }
    if o.no_attach {
        Ok(0)
    } else {
        attach(&runtime, &o.session)
    }
}
fn run() -> io::Result<i32> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|s| s == "__run") {
        if args.len() != 3 {
            return Err(io::Error::other("Invalid runner invocation."));
        }
        let spec = handoff::receive(&args[1], &args[2])?;
        return platform::spawn_spec(&spec);
    }
    let action = args.first().map(String::as_str).unwrap_or("launch");
    match action {
        "--help" | "-h" | "help" => {
            print!("{HELP}");
            Ok(0)
        }
        "--version" => {
            println!("HarnessHub {}", env!("CARGO_PKG_VERSION"));
            Ok(0)
        }
        "providers" => {
            for p in PROVIDERS {
                println!("{:<13} {:<43} {}", p.id, p.name, p.key_env);
            }
            Ok(0)
        }
        "harnesses" => {
            for h in HARNESSES {
                println!(
                    "{:<10} {:<20} {:<13} {}",
                    h.id,
                    h.name,
                    if h.id == "terminal" || platform::executable(h.command).is_some() {
                        "installed"
                    } else {
                        "not installed"
                    },
                    h.providers.join(", ")
                );
            }
            Ok(0)
        }
        "launch" => launch(
            options(if args.is_empty() { &[] } else { &args[1..] }).map_err(io::Error::other)?,
        ),
        "attach" => {
            let o = options(&args[1..]).map_err(io::Error::other)?;
            let runtime = runtime()?;
            start(&runtime, &o.session)?;
            attach(&runtime, &o.session)
        }
        "list" => {
            let o = options(&args[1..]).map_err(io::Error::other)?;
            let snapshot = api(&runtime()?, &o.session, &["api", "snapshot"])?;
            println!("{:<12} {:<12} PROJECT / HARNESS", "PANE", "STATE");
            for pane in snapshot["snapshot"]["panes"]
                .as_array()
                .into_iter()
                .flatten()
            {
                let workspace = snapshot["snapshot"]["workspaces"]
                    .as_array()
                    .and_then(|w| w.iter().find(|w| w["workspace_id"] == pane["workspace_id"]));
                println!(
                    "{:<12} {:<12} {}",
                    pane["pane_id"].as_str().unwrap_or("?"),
                    pane["agent_status"].as_str().unwrap_or("unknown"),
                    workspace
                        .and_then(|w| w["label"].as_str())
                        .unwrap_or("Terminal")
                );
            }
            Ok(0)
        }
        "stop" => {
            let pane = args.get(1).ok_or_else(|| {
                io::Error::other("Usage: harnesshub stop PANE_ID [--session NAME]")
            })?;
            let o = options(&args[2..]).map_err(io::Error::other)?;
            api(&runtime()?, &o.session, &["pane", "close", pane])?;
            println!("Stopped {pane}.");
            Ok(0)
        }
        _ => Err(io::Error::other("Unknown command. Run harnesshub --help.")),
    }
}
fn main() {
    match run() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            eprintln!("HarnessHub: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unknown_or_secret_options_are_rejected() {
        assert!(options(&["--api-key".into(), "fake".into()]).is_err());
        assert!(options(&["--provider".into()]).is_err());
    }
    #[test]
    fn session_names_are_bounded() {
        assert!(options(&["--session".into(), "../default".into()]).is_err());
        assert!(options(&["--session".into(), "test-session".into()]).is_ok());
    }
}
