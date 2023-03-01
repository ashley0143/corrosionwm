// modules
mod handlers;
mod grabs;
mod input;
mod state;
mod winit;

// imports
use smithay::reexports::{calloop::EventLoop, wayland_server::Display};
pub use state::Corrosion;

pub struct CalloopData {
    state: Corrosion,
    display: Display<Corrosion>,
}

// HACK: this is temporary just to find a term
fn find_term() -> &'static str {
    let terms = ["kitty", "wezterm", "urxvt", "weston-terminal"];
    for term in terms.iter() {
        if which::which(term).is_ok() {
            tracing::info!("Found terminal!: {}", term);
            return term;
        }
    }
    tracing::error!("No terminal found, please install one of the following: kitty, wezterm, urxvt, weston-terminal, or use the -c flag to specify a terminal");
    ""
}

fn init_logging() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_env("CORROSIONWM_LOG").unwrap_or_default();
    tracing::info!("logging initialized with env filter: {}", env_filter);
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

fn run_command_on_startup(command: Option<String>) {
    if let Some(command) = command {
        std::process::Command::new(command).spawn().ok();
    }
}

fn start_terminal() {
    let term = find_term();
    if !term.is_empty() {
        std::process::Command::new(term).spawn().ok();
    }
}

fn print_help() {
    println!("Usage: corrosionwm [OPTION]...");
    println!("A Wayland compositor written in Rust");
    println!("--command <command> or -c <command> to run a command on startup");
}

fn parse_args() -> (Option<String>, Option<String>) {
    let mut args = std::env::args().skip(1);
    let flag = args.next();
    let arg = args.next();
    (flag, arg)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    tracing::info!("Starting corrosionWM");

    let mut event_loop = EventLoop::try_new()?;
    let mut display = Display::new()?;
    let state = Corrosion::new(&mut event_loop, &mut display);
    let data = CalloopData { state, display };

    crate::winit::init_winit(&mut event_loop, &mut data)?;

    let (flag, arg) = parse_args();
    match (flag.as_deref(), arg) {
        (Some("-h") | Some("--help"), _) => {
            print_help();
        }
        (Some("-c") | Some("--command"), command) => {
            run_command_on_startup(command);
        }
        _ => {
            start_terminal();
        }
    }

    tracing::info!("Starting corrosionWM event loop");
    event_loop.run(None, &mut data, |_| {
        // corrosionWM is running
    })?;

    Ok(())
}
