use std::path::Path;

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    RGB, RGBA,
}

#[derive(Debug, Clone, Copy)]
pub enum PixelArrayRef<'a> {
    RGB(&'a [u8]),
    RGBA(&'a [u8]),
}

impl PixelArrayRef<'_> {

    /// Returns the number of pixels in the pixel array.
    pub fn len(&self) -> usize {
        match self {
            PixelArrayRef::RGB(data) => data.len() / 3,
            PixelArrayRef::RGBA(data) => data.len() / 4,
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        match self {
            PixelArrayRef::RGB(data) => data.as_ptr(),
            PixelArrayRef::RGBA(data) => data.as_ptr(),
        }
    }

    pub fn to_owned(&self) -> PixelArray {
        match self {
            PixelArrayRef::RGB(data) => PixelArray::RGB(data.to_vec()),
            PixelArrayRef::RGBA(data) => PixelArray::RGBA(data.to_vec()),
        }
    }

    pub fn get_pixel(&self, index: usize) -> [u8; 4] {
        match self {
            PixelArrayRef::RGB(data) => {
                let i = index * 3;
                [data[i], data[i + 1], data[i + 2], 255]
            }

            PixelArrayRef::RGBA(data) => {
                let i = index * 4;
                [data[i], data[i + 1], data[i + 2], data[i + 3]]
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PixelArray {
    RGB(Vec<u8>),
    RGBA(Vec<u8>),
}

impl PixelArray {
    pub fn len(&self) -> usize {
        match self {
            PixelArray::RGB(data) => data.len() / 3,
            PixelArray::RGBA(data) => data.len() / 4,
        }
    }

    pub fn as_ref(&self) -> PixelArrayRef {
        match self {
            PixelArray::RGB(data) => PixelArrayRef::RGB(data),
            PixelArray::RGBA(data) => PixelArrayRef::RGBA(data),
        }
    }

    pub fn get_pixel(&self, index: usize) -> [u8; 4] {
        match self {
            PixelArray::RGB(data) => {
                let i = index * 3;
                [data[i], data[i + 1], data[i + 2], 255]
            }

            PixelArray::RGBA(data) => {
                let i = index * 4;
                [data[i], data[i + 1], data[i + 2], data[i + 3]]
            }
        }
    }
}

impl From<Image> for PixelArray {
    fn from(image: Image) -> PixelArray {
        image.pixel_array
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImageRef<'a> {
    size: (u32, u32),
    pixel_array: PixelArrayRef<'a>,
}

impl<'a> ImageRef<'a> {
    pub fn new(size: (u32, u32), pixel_array: PixelArrayRef<'a>) -> ImageRef<'a> {
        let expected_len = (size.0 * size.1) as usize;
        assert_eq!(pixel_array.len(), expected_len, "Pixel array length does not match image size.");
        
        ImageRef {
            size,
            pixel_array,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn create_texture(&self) -> Result<u32, Error> {
        use crate::gl_utils::{create_texture_rgb, create_texture_rgba};

        match self.pixel_array {
            PixelArrayRef::RGB(data)
                => create_texture_rgb(self.size, data),
            PixelArrayRef::RGBA(data)
                => create_texture_rgba(self.size, data),
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> [u8; 4] {
        let index = y * self.size.0 as usize + x;
        self.pixel_array.get_pixel(index)
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    size: (u32, u32),
    pixel_array: PixelArray,
}

impl Image {
    pub fn new(size: (u32, u32), pixel_array: PixelArray) -> Image {
        Image {
            size,
            pixel_array,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn as_ref(&self) -> ImageRef {
        ImageRef::new(self.size, self.pixel_array.as_ref())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Image, Error> {
        use stb_image::image::{load, LoadResult};
        match load(path) {
            LoadResult::Error(e) => Err(e.into()),

            LoadResult::ImageF32(_) => {
                let message = "Image format is not supported at this time!";
                Err(message.into())
            }

            LoadResult::ImageU8(img) => {
                match img.depth {
                    3 => {
                        let pixel_array = PixelArray::RGB(img.data);
                        let size = (img.width as u32, img.height as u32);
                        Ok(Image::new(size, pixel_array))
                    }

                    4 => {
                        let pixel_array = PixelArray::RGBA(img.data);
                        let size = (img.width as u32, img.height as u32);
                        Ok(Image::new(size, pixel_array))
                    }

                    _ => {
                        let message = "Invalid pixel depth. Must be 3 or 4.";
                        Err(message.into())
                    }
                }
            }
        }
    }

    pub fn create_texture(&self) -> Result<u32, Error> {
        self.as_ref().create_texture()
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> [u8; 4] {
        let index = y * self.size.0 as usize + x;
        self.pixel_array.get_pixel(index)
    }
}


