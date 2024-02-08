use std::path::PathBuf;

use v4l::{buffer::Type, capability::Flags, io::mmap::Stream as MmapStream, Device};

pub struct Webcam {
    pub device: Device,
    pub path: PathBuf,
    pub name: Option<String>,
}

impl Webcam {
    pub fn all() -> Vec<Self> {
        v4l::context::enum_devices()
            .into_iter()
            .filter_map(|dev| {
                let path = dev.path().to_path_buf();
                let name = dev.name();
                let device = Device::with_path(dev.path()).expect("error loading v4l device");
                let caps = device
                    .query_caps()
                    .expect("error querying device capabilities");
                if !caps.capabilities.contains(Flags::VIDEO_CAPTURE) {
                    // This is not a webcam: it does not have the VIDEO_CAPTURE capability. It's
                    // probably one of these weird metadata devices that the Linux kernel creates for
                    // webcams. See https://unix.stackexchange.com/a/539573 for more details.
                    return None;
                }
                Some(Webcam { device, path, name })
            })
            .collect()
    }

    pub fn is_streaming(&self) -> bool {
        let buffer_count = 1; // This causes a panic if we try to set this to 0. However, I know it is
                              // ok to set this to 0 in c. TODO: file an issue with v4l
        match MmapStream::with_buffers(&self.device, Type::VideoCapture, buffer_count) {
            Ok(_) => false, // we were able to grab the webcam! it must not be in use
            Err(e) => {
                // This messy error checking can get cleaned up once `io_error_more` has
                // stabilized: https://github.com/rust-lang/rust/issues/86442
                if let Some(code) = e.raw_os_error() {
                    const EBUSY: i32 = 16;
                    if code == EBUSY {
                        // the device is busy -> it must be in use!
                        return true;
                    }
                }

                dbg!(e);
                let path = self.path.to_string_lossy();
                panic!("Could not determine state of webcam device: {path}");
            }
        }
    }
}
