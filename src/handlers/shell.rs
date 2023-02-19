// src/handlers/shell.rs
// Handles xdg-shell
// Imports
use crate::state::Corrosion;
use smithay::delegate_xdg_shell;
use smithay::{
    reexports::wayland_server::protocol::wl_seat::WlSeat,
    utils::Serial,
    wayland::shell::xdg::{
        PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
    },
};

// Implement the XdgShellHandler trait for Corrosion
impl XdgShellHandler for Corrosion {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.shell_state
    }

    fn new_popup(&mut self, surface: PopupSurface, _positioner: PositionerState) {
        let _ = surface.send_configure();
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        surface.send_configure();
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: WlSeat, _serial: Serial) {
        todo!();
    }
}

delegate_xdg_shell!(Corrosion);

// TODO: Implement decoration handling so you can disable decorations in the config file
