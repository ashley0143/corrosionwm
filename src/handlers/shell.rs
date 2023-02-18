use crate::state::Corrosion;
use smithay::delegate_xdg_shell;
use smithay::{
    reexports::wayland_server::protocol::wl_seat::WlSeat,
    utils::Serial,
    wayland::shell::xdg::{
        PopupConfigure, PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler,
        XdgShellState,
    },
};

impl XdgShellHandler for Corrosion {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.shell_state
    }

    fn new_popup(&mut self, surface: PopupSurface, positioner: PositionerState) {
        &surface.send_configure();
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        surface.send_configure();
    }

    fn grab(&mut self, surface: PopupSurface, seat: WlSeat, serial: Serial) {
        todo!();
    }
}

delegate_xdg_shell!(Corrosion);
