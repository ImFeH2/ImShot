use crate::utils::image::Image;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, SelectObject, HBITMAP, SRCCOPY};
use windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow;

pub struct Screenshot;

impl Screenshot {
    pub fn new() -> Self {
        Self
    }

    pub fn shot(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Image {
        unsafe {
            let hwnd: HWND = GetDesktopWindow();
            let hdc = GetDC(hwnd);
            let hdc_mem = CreateCompatibleDC(hdc);

            assert!(x1 < x2 && y1 < y2, "Invalid coordinates");

            let width = x2 - x1;
            let height = y2 - y1;

            let hbm: HBITMAP = CreateCompatibleBitmap(hdc, width, height);
            SelectObject(hdc_mem, hbm);

            BitBlt(hdc_mem, 0, 0, width, height, hdc, x1, y1, SRCCOPY).expect("BitBlt failed");

            let image = Image::from_bitmap(hdc_mem, width as u32, height as u32);

            let _ = DeleteObject(hbm);
            let _ = DeleteDC(hdc_mem);
            let _ = DeleteDC(hdc);

            image
        }
    }
}