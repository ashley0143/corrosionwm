use state::Corrosion;
use wayland_server::Display;
mod state;

fn main() {
    let display = Display::<Corrosion>::new();
}
