use crate::utils::image::Image;
use crate::utils::structures::Rectangle;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, SelectObject, HBITMAP, SRCCOPY};
use windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow;

pub struct Screenshot;

impl Screenshot {
    pub fn shot(&self, rect: &Rectangle) -> Image {
        unsafe {
            let hwnd: HWND = GetDesktopWindow();
            let hdc = GetDC(hwnd);
            let hdc_mem = CreateCompatibleDC(hdc);

            let hbm: HBITMAP = CreateCompatibleBitmap(hdc, rect.width() as i32, rect.height() as i32);
            SelectObject(hdc_mem, hbm);

            BitBlt(hdc_mem, 0, 0, rect.width() as i32, rect.height() as i32, hdc, rect.left, rect.top, SRCCOPY).expect("BitBlt failed");

            let image = Image::from_bitmap(hdc_mem, rect.dimension());

            let _ = DeleteObject(hbm);
            let _ = DeleteDC(hdc_mem);
            let _ = DeleteDC(hdc);

            image
        }
    }
}