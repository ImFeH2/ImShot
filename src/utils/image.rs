use crate::utils::structures::Dimensions;
use image::{ImageBuffer, ImageResult, Rgba};
use windows::Win32::Graphics::Gdi::{GetPixel, HDC};

pub struct Image {
    pub buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Image {
    pub fn new(buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Image {
        Self {
            buffer,
        }
    }
    pub fn from_bitmap(bitmap: HDC, dim: Dimensions) -> Image {
        let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(dim.width, dim.height);

        for y in 0..dim.height {
            for x in 0..dim.width {
                let color = unsafe { GetPixel(bitmap, x as i32, y as i32).0 };

                let r = (color & 0x000000FF_u32) as u8;
                let g = ((color & 0x0000FF00_u32) >> 8) as u8;
                let b = ((color & 0x00FF0000_u32) >> 16) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }

        Image::new(img)
    }

    pub fn save(&self, path: &str) -> ImageResult<()> {
        self.buffer.save(path)
    }
}