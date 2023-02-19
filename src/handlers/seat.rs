// src/handlers/seat.rs
// Handles the seat
// Imports
use crate::state::Corrosion;
use smithay::{
    delegate_seat,
    input::{SeatHandler, SeatState},
    reexports::wayland_server::protocol::wl_surface::WlSurface,
};

// Implement the SeatHandler trait for Corrosion
impl SeatHandler for Corrosion {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
}

delegate_seat!(Corrosion);
