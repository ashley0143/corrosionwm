// src/main.rs
// Main file
// Imports
use slog::{o, Drain};
use smithay::reexports::{calloop::EventLoop, wayland_server::Display};
use state::Corrosion;
mod handlers;
mod state;

// Define the CalloopData struct
pub struct CalloopData {
    state: Corrosion,
    display: Display<Corrosion>,
}

// The &String in Result<> is a placeholder for now
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _logger = slog::Logger::root(slog_stdlog::StdLog.fuse(), o!());
    let _event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;

    let _display = Display::<Corrosion>::new().unwrap();
    Ok(())
}
