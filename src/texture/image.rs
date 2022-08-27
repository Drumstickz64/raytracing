use stb_image::image::{Image, LoadResult};

use crate::texture::Texture;

pub struct ImageTexture {
    image: Option<Image<u8>>,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub const BYTES_PER_PIXEL: usize = 3;

    pub fn new(filename: &str) -> Self {
        let image = stb_image::image::load(filename);
        let image = match image {
            LoadResult::ImageU8(image) => image,
            LoadResult::Error(err) => {
                eprintln!("ERROR: Unable to load image '{}': {}", filename, err);
                return Self::empty();
            }
            LoadResult::ImageF32(_) => {
                eprintln!(
                    "ERROR: Unable to load image '{}': Images with float-based colors unimplemented", filename
                );
                return Self::empty();
            }
        };

        Self {
            bytes_per_scanline: Self::BYTES_PER_PIXEL * image.width,
            image: Some(image),
        }
    }

    pub fn empty() -> Self {
        Self {
            image: None,
            bytes_per_scanline: 0,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: glam::DVec3) -> glam::DVec3 {
        let image = match &self.image {
            Some(image) => image,
            None => return glam::dvec3(0.0, 1.0, 1.0),
        };

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // flip Y coordinates

        let i = (u * image.width as f64) as usize;
        let j = (v * image.height as f64) as usize;

        let i = i.min(image.width);
        let j = j.min(image.height);

        const COLOR_SCALE: f64 = 1.0 / 255.0;
        let pixel = j * self.bytes_per_scanline + i * Self::BYTES_PER_PIXEL;

        glam::dvec3(
            image.data[pixel] as f64 * COLOR_SCALE,
            image.data[pixel + 1] as f64 * COLOR_SCALE,
            image.data[pixel + 2] as f64 * COLOR_SCALE,
        )
    }
}
