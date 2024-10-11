mod core {
    pub mod screenshot;
}

mod utils {
    pub mod image;
}

use crate::core::screenshot::Screenshot;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

fn init() {
    unsafe {
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2).expect("Failed to SetProcessDpiAwarenessContext");
    }
}

fn main() {
    init();

    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        let screenshot = Screenshot::new();
        let image = screenshot.shot(0, 0, width, height);
        image.save("screenshot.png").expect("Failed to save image");
    }
}