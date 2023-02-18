use slog::Logger;
use smithay::reexports::{
    calloop::{EventLoop, LoopSignal},
    wayland_server::{
        protocol::wl_surface::{self, WlSurface},
        Display,
    },
};
use smithay::{
    desktop::{Space, Window},
    input::{Seat, SeatHandler, SeatState},
    wayland::compositor::CompositorHandler,
    wayland::socket,
    wayland::{compositor::CompositorState, seat, shell::xdg::XdgShellState},
};

pub struct Corrosion {
    pub seat: Seat<Self>,
    pub compositor_state: CompositorState,
    pub seat_state: SeatState<Self>,
    pub space: Space<Window>,
    pub shell_state: XdgShellState,
    pub loop_signal: LoopSignal,
}

impl Corrosion {
    pub fn new(
        display: Display<Self>,
        logger: Option<Logger>,
        event_loop: EventLoop<crate::CalloopData>,
    ) -> Corrosion {
        let dh = &display.handle();

        let mut seat_state = SeatState::new();
        let mut seat: Seat<Self> = seat_state.new_wl_seat(dh, "seat-0", logger.clone());

        let compositor_state = CompositorState::new::<Self, _>(dh, logger.clone());

        let space = Space::new(logger.clone());

        let shell_state = XdgShellState::new::<Self, _>(dh, logger.clone());

        let loop_signal = event_loop.get_signal();

        Corrosion {
            seat,
            compositor_state,
            seat_state,
            space,
            shell_state,
            loop_signal,
        }
    }
}
