// src/udev.rs
// handles udev backend
// imports
use std::path::PathBuf;

use smithay::backend::udev::{self, UdevBackend};


// define the udevdata struct
pub struct UdevData {
    backend: UdevBackend, // The udev backend of UdevData
    primary_gpu: PathBuf, // the primary gpu of the system
}

pub fn initialize_backend() -> std::io::Result<UdevBackend> {
    tracing::info!("Initializing udev backend....");
    let backend = match UdevBackend::new("seat0") {
        // create a new udev backend (seat0 is the default seat name)
        Ok(backend) => {
            backend // if no error, return the backend
        }
        Err(error) => {
            tracing::error!("Failed to initialize udev backend: {}", error);
            return Err(error); // if this happens, something fucked up
        }
    };
    tracing::info!("Udev backend initialized successfully!");
    Ok(backend)
}

// implementation for the udevdata struct
impl UdevData {
    pub fn new() -> Self {
        let backend = crate::udev::initialize_backend().unwrap();
        let primary_gpu = udev::primary_gpu("seat0").unwrap().unwrap();
        UdevData {
            backend,
            primary_gpu,
        }
    }
    // initialize udev backend
}
