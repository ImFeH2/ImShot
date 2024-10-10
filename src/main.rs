use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDeviceCaps,
    SelectObject, SRCCOPY, GetPixel,
};
use windows::Win32::UI::WindowsAndMessaging::{GetDesktopWindow, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use windows::Win32::UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2};
use image::{ImageBuffer, Rgba};

fn main() {
    unsafe {
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2).expect("Failed to SetProcessDpiAwarenessContext");
    }

    let hwnd: HWND = unsafe { GetDesktopWindow() };
    let hdc = unsafe { GetDC(hwnd) };
    let hdc_mem = unsafe { CreateCompatibleDC(hdc) };

    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

    let hbm = unsafe { CreateCompatibleBitmap(hdc, width, height) };
    unsafe { SelectObject(hdc_mem, hbm) };

    unsafe {
        BitBlt(hdc_mem, 0, 0, width, height, hdc, 0, 0, SRCCOPY).expect("BitBlt failed");
    }

    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let color = unsafe { GetPixel(hdc_mem, x, y).0 };

            let r = (color & 0x000000FF_u32) as u8;
            let g = ((color & 0x0000FF00_u32) >> 8) as u8;
            let b = ((color & 0x00FF0000_u32) >> 16) as u8;
            img.put_pixel(x as u32, y as u32, Rgba([r, g, b, 255]));
        }
    }

    img.save("screenshot.png").unwrap();

    unsafe {
        let _ = DeleteObject(hbm);
        let _ = DeleteDC(hdc_mem);
        let _ = DeleteDC(hdc);
    }
}