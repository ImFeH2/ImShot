mod image_saver;

use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, SelectObject, HBITMAP, SRCCOPY};
use windows::Win32::UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2};
use windows::Win32::UI::WindowsAndMessaging::{GetDesktopWindow, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};


fn main() {
    unsafe {
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2).expect("Failed to SetProcessDpiAwarenessContext");
    }

    let hwnd: HWND = unsafe { GetDesktopWindow() };
    let hdc = unsafe { GetDC(hwnd) };
    let hdc_mem = unsafe { CreateCompatibleDC(hdc) };

    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

    let hbm: HBITMAP = unsafe { CreateCompatibleBitmap(hdc, width, height) };
    unsafe { SelectObject(hdc_mem, hbm) };

    unsafe {
        BitBlt(hdc_mem, 0, 0, width, height, hdc, 0, 0, SRCCOPY).expect("BitBlt failed");
    }

    let image_saver = image_saver::ImageSaver::from_bitmap(hdc_mem, width as u32, height as u32);
    match image_saver.save("screenshot.png") {
        Ok(_) => println!("Screenshot saved"),
        Err(e) => println!("Failed to save screenshot: {:?}", e),
    }


    unsafe {
        let _ = DeleteObject(hbm);
        let _ = DeleteDC(hdc_mem);
        let _ = DeleteDC(hdc);
    }
}