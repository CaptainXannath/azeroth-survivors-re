
 //! The host OS pasteboard, the one place benilla talks to the platform clipboard.

use std::ffi::c_void;

use bevy::prelude::*;
use bevy::window::RawHandleWrapper;

/// One platform pasteboard.
trait Pasteboard {
    fn read_text(&mut self) -> Result<Option<String>, String>;
    fn write_text(&mut self, text: &str) -> Result<(), String>;
    fn name(&self) -> &'static str;
}

#[cfg(not(target_arch = "wasm32"))]
impl Pasteboard for arboard::Clipboard {
    fn read_text(&mut self) -> Result<Option<String>, String> {
        match self.get_text() {
            Ok(text) if text.is_empty() => Ok(None),
            Ok(text) => Ok(Some(text)),
            Err(arboard::Error::ContentNotAvailable) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    fn write_text(&mut self, text: &str) -> Result<(), String> {
        self.set_text(text.to_string())
            .map_err(|e| e.to_string())
    }

    fn name(&self) -> &'static str {
        if cfg!(target_os = "macos") {
            "arboard/NSPasteboard"
        } else if cfg!(windows) {
            "arboard/win32"
        } else {
            "arboard/x11"
        }
    }
}

/// Linux Wayland clipboard implementation.
#[cfg(target_os = "linux")]
mod wl {
    use super::Pasteboard;

    impl Pasteboard for smithay_clipboard::Clipboard {
        fn read_text(&mut self) -> Result<Option<String>, String> {
            match self.load() {
                Ok(text) if text.is_empty() => Ok(None),
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(e.to_string()),
            }
        }

        fn write_text(&mut self, text: &str) -> Result<(), String> {
            self.store(text.to_string());
            Ok(())
        }

        fn name(&self) -> &'static str {
            "smithay/wl_data_device"
        }
    }
}

/// Get the Wayland display on Linux; other platforms return None.
#[cfg(target_os = "linux")]
pub(crate) fn wayland_display(
    handle: Option<&RawHandleWrapper>,
) -> Option<*mut c_void> {
    match handle?.get_display_handle() {
        raw_window_handle::RawDisplayHandle::Wayland(wl) => {
            Some(wl.display.as_ptr())
        }
        _ => None,
    }
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn wayland_display(
    _handle: Option<&RawHandleWrapper>,
) -> Option<*mut c_void> {
    None
}

#[cfg(target_os = "linux")]
fn open_wayland(display: *mut c_void) -> Box<dyn Pasteboard> {
    // SAFETY: The caller must provide a live Wayland display.
    Box::new(unsafe { smithay_clipboard::Clipboard::new(display) })
}

#[cfg(not(target_os = "linux"))]
fn open_wayland(_display: *mut c_void) -> Box<dyn Pasteboard> {
    unreachable!("Wayland display is only available on Linux")
}

fn session_note() -> String {
    if !cfg!(unix) || cfg!(target_os = "macos") {
        return String::new();
    }

    let var = |key: &str| {
        std::env::var(key).unwrap_or_else(|_| "unset".to_string())
    };

    format!(
        " [XDG_SESSION_TYPE={}, WAYLAND_DISPLAY={}, DISPLAY={}]",
        var("XDG_SESSION_TYPE"),
        var("WAYLAND_DISPLAY"),
        var("DISPLAY"),
    )
}

/// The process-wide handle on the OS pasteboard.
#[derive(Default)]
pub(crate) struct HostClipboard {
    backend: Option<Box<dyn Pasteboard>>,
    opened: bool,
}

impl HostClipboard {
    fn ensure(&mut self, wl_display: Option<*mut c_void>) {
        if self.opened {
            return;
        }

        self.opened = true;

        self.backend = match wl_display {
            Some(display) => Some(open_wayland(display)),

            #[cfg(not(target_arch = "wasm32"))]
            None => match arboard::Clipboard::new() {
                Ok(clipboard) => Some(Box::new(clipboard)),
                Err(e) => {
                    warn!("clipboard: unavailable — {e}{}", session_note());
                    None
                }
            },

            #[cfg(target_arch = "wasm32")]
            None => None,
        };

        if let Some(backend) = &self.backend {
            info!("clipboard: {}{}", backend.name(), session_note());
        }
    }

    pub(crate) fn read(
        &mut self,
        wl_display: Option<*mut c_void>,
    ) -> Option<String> {
        self.ensure(wl_display);

        let backend = self.backend.as_mut()?;

        match backend.read_text() {
            Ok(text) => text,
            Err(e) => {
                warn!(
                    "clipboard: read failed via {} — {e}{}",
                    backend.name(),
                    session_note()
                );
                None
            }
        }
    }

    pub(crate) fn write(
        &mut self,
        wl_display: Option<*mut c_void>,
        text: &str,
    ) {
        self.ensure(wl_display);

        let Some(backend) = self.backend.as_mut() else {
            return;
        };

        if let Err(e) = backend.write_text(text) {
            warn!(
                "clipboard: write failed via {} — {e}{}",
                backend.name(),
                session_note()
            );
        }
    }
}
