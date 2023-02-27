#![allow(irrefutable_let_patterns)]

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        // change this by changing the RUST_LOG environment variable
        tracing::info!("logging initialized with env filter: {}", env_filter);
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
        tracing::info!("logging initialized with default filter");
    }
    tracing::info!("logging initialized");
    tracing::info!("Starting corrosionWM");

    let mut event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;

    let mut display: Display<Corrosion> = Display::new()?;
    let state = Corrosion::new(&mut event_loop, &mut display);

    let mut data = CalloopData { state, display };

    crate::winit::init_winit(&mut event_loop, &mut data)?;

    let mut args = std::env::args().skip(1);
    let flag = args.next();
    let arg = args.next();

    match (flag.as_deref(), arg) {
        (Some("-h") | Some("--help"), _) => {
            println!("Usage: corrosionwm [OPTION]...");
            println!("A Wayland compositor written in Rust");
            println!("--command <command> or -c <command> to run a command on startup");
        }
        (Some("-c") | Some("--command"), Some(command)) => {
            std::process::Command::new(command).spawn().ok();
        }
        _ => {
            // TODO: Make this configurable
            // TODO: remove this completely as this shit is just for debugging
            std::process::Command::new("kitty").spawn().expect("You may not have kitty installed, if not, please install it, or use the --command/-c flag to specify a different program to run.");
        }
    }

    tracing::info!("Starting corrosionWM event loop");
    event_loop.run(None, &mut data, move |_| {
        // corrosionWM is running
    })?;

    Ok(())
}
