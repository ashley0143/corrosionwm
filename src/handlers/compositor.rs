use crate::state::Corrosion;
use smithay::{
    backend::renderer::utils,
    delegate_compositor,
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    wayland::compositor::{self, CompositorHandler, CompositorState},
};

impl CompositorHandler for Corrosion {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        utils::on_commit_buffer_handler(surface);
        if !compositor::is_sync_subsurface(surface) {
            let mut root = surface.clone();
            while let Some(parent) = compositor::get_parent(&root) {
                root = parent;
            }

            if let Some(window) = self
                .space
                .elements()
                .find(|w| w.toplevel().wl_surface() == &root)
            {
                window.on_commit();
            };
        }
    }
}
delegate_compositor!(Corrosion);
