mod core {
    pub mod screenshot;
}

mod utils {
    pub mod image;
    pub mod structures;
}

use crate::core::screenshot::Screenshot;
use crate::utils::structures::{Rectangle, FULLSCREEN};
use windows::Win32::UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

fn init() {
    unsafe {
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2).expect("Failed to SetProcessDpiAwarenessContext");
        FULLSCREEN.right = GetSystemMetrics(SM_CXSCREEN);
        FULLSCREEN.bottom = GetSystemMetrics(SM_CYSCREEN);
    }
}


fn main() {
    init();

    unsafe {
        let rect = Rectangle::new(100, 50, 2400, 1600);

        let screenshot = Screenshot::new();
        let image = screenshot.shot(&rect);
        image.save("screenshot.png").expect("Failed to save image");
    }
}