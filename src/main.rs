use smithay::reexports::wayland_server::Display;
use state::Corrosion;
mod state;

fn main() {
    let display = Display::<Corrosion>::new();
}
