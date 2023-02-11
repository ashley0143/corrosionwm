mod compositor;
mod xdg_shell;

use crate::Corrosion;

//
// Wl Seat
//

use smithay::input::{SeatHandler, SeatState};
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::wayland::data_device::{ClientDndGrabHandler, DataDeviceHandler, ServerDndGrabHandler};
use smithay::{delegate_data_device, delegate_output, delegate_seat};

impl SeatHandler for Corrosion {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Corrosion> {
        &mut self.seat_state
    }

    fn cursor_image(
        &mut self,
        _seat: &smithay::input::Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }
    fn focus_changed(&mut self, _seat: &smithay::input::Seat<Self>, _focused: Option<&WlSurface>) {}
}

delegate_seat!(Corrosion);

//
// Wl Data Device
//

impl DataDeviceHandler for Corrosion {
    fn data_device_state(&self) -> &smithay::wayland::data_device::DataDeviceState {
        &self.data_device_state
    }
}

impl ClientDndGrabHandler for Corrosion {}
impl ServerDndGrabHandler for Corrosion {}

delegate_data_device!(Corrosion);

//
// Wl Output & Xdg Output
//

delegate_output!(Corrosion);
