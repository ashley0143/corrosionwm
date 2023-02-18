use slog::{o, Drain};
use smithay::reexports::{calloop::EventLoop, wayland_server::Display};
use state::Corrosion;
mod handlers;
mod state;

pub struct CalloopData {
    state: Corrosion,
    display: Display<Corrosion>,
}
// The &String in Result<> is a placeholder for now
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let logger = slog::Logger::root(slog_stdlog::StdLog.fuse(), o!());
    let mut event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;

    let display = Display::<Corrosion>::new().unwrap();
    Ok(())
}
