use clap::Parser;
use std::process::Command;
use std::{fs, thread};
use std::time::{Duration, Instant};
use sysinfo::{ProcessRefreshKind, System};

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    /// Duration in milliseconds before the process watcher times out
    #[arg(short, long, default_value_t = 30000)]
    timeout: u64,

    /// Delay in milliseconds between process checks
    #[arg(short, long, default_value_t = 100)]
    interval: u64,

    /// Name of the program to launch
    program_path: String,

    /// Name of the process to watch
    process_name: String,

    /// Additional program arguments
    arguments: Vec<String>,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let program = fs::canonicalize(&cli.program_path)
        .map_err(|err| format!("Could not find {}: {err}", cli.program_path))?;

    Command::new(program).args(cli.arguments).spawn()
        .map_err(|err| format!("Could not start {}: {err}", cli.program_path))?;

    println!("Waiting for process {}...", cli.process_name);

    let deadline = Instant::now() + Duration::from_millis(cli.timeout);
    let poll = Duration::from_millis(cli.interval);

    let mut sys = System::new();

    let (pid, start_time) = loop {
        sys.refresh_processes_specifics(ProcessRefreshKind::new());

        if let Some(p) = sys.processes().values().find(|p| p.name() == cli.process_name) {
            break (p.pid(), p.start_time());
        }

        if Instant::now() >= deadline {
            return Err(format!("Process {} not found", cli.process_name));
        }

        thread::sleep(poll);
    };

    println!("Process {} (ID: {}) found, waiting to exit...", pid, cli.process_name);

    while sys.process(pid).is_some_and(|p| p.start_time() == start_time) {
        sys.refresh_processes_specifics(ProcessRefreshKind::new());
        thread::sleep(poll);
    }

    Ok(())
}
