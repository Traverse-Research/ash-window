use ash::{
    prelude::*,
    version::{EntryV1_0, InstanceV1_0},
    vk,
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::CStr;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
use ash::extensions::khr;

#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::extensions::mvk;

/// Create a surface from a raw surface handle.
///
/// `instance` must have created with platform specific surface extensions enabled.
pub unsafe fn create_surface<E, I>(
    entry: &E,
    instance: &I,
    window_handle: &impl HasRawWindowHandle,
    allocation_callbacks: Option<&vk::AllocationCallbacks>,
) -> VkResult<vk::SurfaceKHR>
where
    E: EntryV1_0,
    I: InstanceV1_0,
{
    match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(handle) => {
            let surface_desc = vk::Win32SurfaceCreateInfoKHR::builder()
                .hinstance(handle.hinstance)
                .hwnd(handle.hwnd);
            let surface_fn = khr::Win32Surface::new(entry, instance);
            surface_fn.create_win32_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(handle) => {
            let surface_desc = vk::WaylandSurfaceCreateInfoKHR::builder()
                .display(handle.display)
                .surface(handle.surface);
            let surface_fn = khr::WaylandSurface::new(entry, instance);
            surface_fn.create_wayland_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(handle) => {
            let surface_desc = vk::XlibSurfaceCreateInfoKHR::builder()
                .dpy(handle.display as *mut _)
                .window(handle.window);
            let surface_fn = khr::XlibSurface::new(entry, instance);
            surface_fn.create_xlib_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(handle) => {
            let surface_desc = vk::XcbSurfaceCreateInfoKHR::builder()
                .connection(handle.connection as *mut _)
                .window(handle.window);
            let surface_fn = khr::XcbSurface::new(entry, instance);
            surface_fn.create_xcb_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(handle) => {
            let surface_desc =
                vk::AndroidSurfaceCreateInfoKHR::builder().window(handle.a_native_window as _);
            let surface_fn = khr::AndroidSurface::new(entry, instance);
            surface_fn.create_android_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(handle) => {
            let surface_desc = vk::MacOSSurfaceCreateInfoMVK::builder().view(handle.ns_view);
            let surface_fn = mvk::MacOSSurface::new(entry, instance);
            surface_fn.create_mac_os_surface_mvk(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(handle) => {
            let surface_desc = vk::IOSSurfaceCreateInfoMVK::builder().view(handle.ui_view);
            let surface_fn = mvk::IOSSurface::new(entry, instance);
            surface_fn.create_ios_surface_mvk(&surface_desc, allocation_callbacks)
        }

        _ => unimplemented!(),
    }
}

/// Query the required instance extension for creating a surface from a window handle.
pub fn enumerate_required_extension(window_handle: &impl HasRawWindowHandle) -> &'static CStr {
    match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(_) => khr::Win32Surface::name(),

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(_) => khr::WaylandSurface::name(),

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(_) => khr::XlibSurface::name(),

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(_) => khr::XcbSurface::name(),

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(_) => khr::AndroidSurface::name(),

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(_) => mvk::MacOSSurface::name(),

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(_) => mvk::IOSSurface::name(),

        _ => unimplemented!(),
    }
}
