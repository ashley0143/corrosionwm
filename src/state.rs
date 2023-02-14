use smithay::reexports::wayland_server::protocol::wl_surface::{self, WlSurface};
use smithay::{
    input::{Seat, SeatHandler, SeatState},
    wayland::compositor::CompositorHandler,
    wayland::socket,
    wayland::{compositor::CompositorState, seat},
};

pub struct Corrosion {
    pub seat: Seat<Self>,
    pub compositor_state: CompositorState,
    pub seat_state: SeatState<Corrosion>,
}

impl CompositorHandler for Corrosion {
    fn compositor_state(&mut self) -> &mut smithay::wayland::compositor::CompositorState {
        &mut self.compositor_state
    }

    fn commit(&mut self, surface: &wl_surface::WlSurface) {
        // Stuff goes here
    }
}

impl SeatHandler for Corrosion {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    fn seat_state(&mut self) -> &mut smithay::input::SeatState<Self> {
        &mut self.seat_state
    }
}
