use image::{GenericImage, ImageBuffer, Luma, Pixel, Rgb32FImage, Rgba32FImage};
use msdf_sys::*;
use std::os::raw::c_int;

pub type GrayFImage = ImageBuffer<Luma<f32>, Vec<f32>>;

fn get_msdfgen_bitmap<T: Pixel>(image: &ImageBuffer<T, Vec<T::Subpixel>>) -> msdfgen_Bitmap<f32> {
    msdfgen_Bitmap {
        pixels: image.as_flat_samples().samples.as_ptr() as *mut f32,
        w: image.width() as c_int,
        h: image.height() as c_int,
        _phantom_0: Default::default(),
    }
}

/// Trait for various types of SDFs. Can be converted to an image, or rendered with
/// [SDFTrait::render] / [SDFTrait::render_colored].
pub trait SDFTrait {
    type Image: GenericImage;
    type ColoredOutput: GenericImage;

    /// Create an SDF from an image.
    fn from_image(image: Self::Image, range: f64, mid_value: f32) -> Self;
    /// Convert an SDF to an image.
    fn to_image(self) -> Self::Image;

    /// Get SDF image.
    fn image(&self) -> &Self::Image;
    /// Get SDF range.
    fn range(&self) -> f64;
    /// Get SDF mid-value.
    fn mid_value(&self) -> f32;

    /// Render this SDF to a single-channel float image of specified size.
    fn render(&self, width: u32, height: u32) -> GrayFImage;
    /// Render this SDF to a multi-channel float image of specified size. Colors represent MSDF
    /// edge colors.
    fn render_colored(&self, width: u32, height: u32) -> Self::ColoredOutput;
}

/// A conventional single-channel signed distance field. Backed by a single-channel [f32] image.
pub struct SDF {
    image: GrayFImage,
    range: f64,
    mid_value: f32,
}

/// A multi-channel signed distance field. Backed by a three-channel [f32] image.
pub struct MSDF {
    image: Rgb32FImage,
    range: f64,
    mid_value: f32,
}

/// A multi-channel signed distance field with true distance. Backed by a four-channel [f32] image.
pub struct MTSDF {
    image: Rgba32FImage,
    range: f64,
    mid_value: f32,
}

impl SDFTrait for SDF {
    type Image = GrayFImage;
    type ColoredOutput = Rgb32FImage;

    fn from_image(image: Self::Image, range: f64, mid_value: f32) -> Self {
        SDF {
            image,
            range,
            mid_value,
        }
    }

    fn to_image(self) -> Self::Image {
        self.image
    }

    fn image(&self) -> &Self::Image {
        &self.image
    }

    fn range(&self) -> f64 {
        self.range
    }

    fn mid_value(&self) -> f32 {
        self.mid_value
    }

    fn render(&self, width: u32, height: u32) -> GrayFImage {
        let image = GrayFImage::new(width, height);

        let input = get_msdfgen_bitmap(&self.image);
        let output = get_msdfgen_bitmap(&image);

        unsafe {
            msdfgen_renderSDF(
                &output as *const msdfgen_Bitmap<_> as *const _,
                &input as *const msdfgen_Bitmap<_> as *const _,
                self.range(),
                self.mid_value(),
            );
        }

        image
    }

    fn render_colored(&self, width: u32, height: u32) -> Self::ColoredOutput {
        let image = Self::ColoredOutput::new(width, height);

        let input = get_msdfgen_bitmap(&self.image);
        let output = get_msdfgen_bitmap(&image);

        unsafe {
            msdfgen_renderSDF1(
                &output as *const msdfgen_Bitmap<_> as *const _,
                &input as *const msdfgen_Bitmap<_> as *const _,
                self.range(),
                self.mid_value(),
            );
        }

        image
    }
}

impl SDFTrait for MSDF {
    type Image = Rgb32FImage;
    type ColoredOutput = Rgb32FImage;

    fn from_image(image: Self::Image, range: f64, mid_value: f32) -> Self {
        MSDF {
            image,
            range,
            mid_value,
        }
    }

    fn to_image(self) -> Self::Image {
        self.image
    }

    fn image(&self) -> &Self::Image {
        &self.image
    }

    fn range(&self) -> f64 {
        self.range
    }

    fn mid_value(&self) -> f32 {
        self.mid_value
    }

    fn render(&self, width: u32, height: u32) -> GrayFImage {
        let image = GrayFImage::new(width, height);

        let input = get_msdfgen_bitmap(&self.image);
        let output = get_msdfgen_bitmap(&image);

        unsafe {
            msdfgen_renderSDF2(
                &output as *const msdfgen_Bitmap<_> as *const _,
                &input as *const msdfgen_Bitmap<_> as *const _,
                self.range(),
                self.mid_value(),
            );
        }

        image
    }

    fn render_colored(&self, width: u32, height: u32) -> Self::ColoredOutput {
        let image = Self::ColoredOutput::new(width, height);

        let input = get_msdfgen_bitmap(&self.image);
        let output = get_msdfgen_bitmap(&image);

        unsafe {
            msdfgen_renderSDF3(
                &output as *const msdfgen_Bitmap<_> as *const _,
                &input as *const msdfgen_Bitmap<_> as *const _,
                self.range(),
                self.mid_value(),
            );
        }

        image
    }
}

impl SDFTrait for MTSDF {
    type Image = Rgba32FImage;
    type ColoredOutput = Rgba32FImage;

    fn from_image(image: Self::Image, range: f64, mid_value: f32) -> Self {
        MTSDF {
            image,
            range,
            mid_value,
        }
    }

    fn to_image(self) -> Self::Image {
        self.image
    }

    fn image(&self) -> &Self::Image {
        &self.image
    }

    fn range(&self) -> f64 {
        self.range
    }

    fn mid_value(&self) -> f32 {
        self.mid_value
    }

    fn render(&self, width: u32, height: u32) -> GrayFImage {
        let image = GrayFImage::new(width, height);

        let input = get_msdfgen_bitmap(&self.image);
        let output = get_msdfgen_bitmap(&image);

        unsafe {
            msdfgen_renderSDF4(
                &output as *const msdfgen_Bitmap<_> as *const _,
                &input as *const msdfgen_Bitmap<_> as *const _,
                self.range(),
                self.mid_value(),
            );
        }

        image
    }

    fn render_colored(&self, width: u32, height: u32) -> Self::ColoredOutput {
        let image = Self::ColoredOutput::new(width, height);

        let input = get_msdfgen_bitmap(&self.image);
        let output = get_msdfgen_bitmap(&image);

        unsafe {
            msdfgen_renderSDF5(
                &output as *const msdfgen_Bitmap<_> as *const _,
                &input as *const msdfgen_Bitmap<_> as *const _,
                self.range(),
                self.mid_value(),
            );
        }

        image
    }
}
