//! Rust bindings for [msdfgen](https://github.com/Chlumsky/msdfgen). This crate attempts to bind
//! msdfgen in a safe and idiomatic way.
//!
//! # Examples
//! ## Generate SDFs
//! ```rust
//! # mod test_helpers;
//! # use test_helpers::compare_images;
//! # use std::env;
//! # use std::fs::File;
//! # use std::io::{BufReader, Read};
//! # use std::os::raw::c_uint;
//! # use image::DynamicImage;
//! # use ttf_parser::Face;
//! # use msdf::SDFTrait;
//! #
//! # fn main() {
//! # use mint::Vector2;
//! use msdf::{GlyphLoader, Projection};
//! # let path = env::current_dir()
//! #     .unwrap()
//! #     .join("test_resources")
//! #     .join("Roboto-Medium.ttf");
//! # let file = File::open(path).unwrap();
//! # let mut reader = BufReader::new(file);
//! #
//! # let mut font_data_vec = vec![];
//! #
//! # reader.read_to_end(&mut font_data_vec).unwrap();
//! #
//! # let font_data = font_data_vec.as_slice();
//!
//! // Load a font from ttf data.
//! let face: Face;
//! # let face = Face::from_slice(font_data, 0).unwrap();
//! #
//! let glyph_index = face.glyph_index('W').unwrap();
//!
//! // Load a glyph into a shape using a ttf glyph index.
//! let shape = face.load_shape(glyph_index).unwrap();
//!
//! // Not a required step for SDF and Psuedo-SDF generation. Other coloring options exist.
//! let colored_shape = shape.color_edges_simple(3.0);
//!
//! // Project glyph down by a factor of 64x.
//! let projection = Projection {
//!     scale: Vector2 { x: 1.0 / 64.0, y: 1.0 / 64.0 },
//!     translation: Vector2 { x: 0.0, y: 0.0 },
//! };
//!
//! // Using default configuration.
//! let sdf_config = Default::default();
//! let msdf_config = Default::default();
//!
//! // Generate all types of SDF. Plain SDFs and Psuedo-SDFs do not require edge coloring.
//! let sdf   = colored_shape.generate_sdf(32, 32, 10.0 * 64.0, &projection, &sdf_config);
//! let psdf  = colored_shape.generate_psuedo_sdf(32, 32, 10.0 * 64.0, &projection, &sdf_config);
//! let msdf  = colored_shape.generate_msdf(32, 32, 10.0 * 64.0, &projection, &msdf_config);
//! let mtsdf = colored_shape.generate_mtsdf(32, 32, 10.0 * 64.0, &projection, &msdf_config);
//!
//! // Do something with these SDFs.
//! // let image: DynamicImage = DynamicImage::from(msdf.to_image());
//! // image.into_rgba8().save("mysdf.png").unwrap();
//! #
//! # let sdf: DynamicImage = DynamicImage::from(sdf.to_image());
//! # let psdf: DynamicImage = DynamicImage::from(psdf.to_image());
//! # let msdf: DynamicImage = DynamicImage::from(msdf.to_image());
//! # let mtsdf: DynamicImage = DynamicImage::from(mtsdf.to_image());
//! # let sdf = sdf.into_rgba8();
//! # let psdf = psdf.into_rgba8();
//! # let msdf = msdf.into_rgba8();
//! # let mtsdf = mtsdf.into_rgba8();
//! #
//! # compare_images(&sdf, "doctest_sdf");
//! # compare_images(&psdf, "doctest_psdf");
//! # compare_images(&msdf, "doctest_msdf");
//! # compare_images(&mtsdf, "doctest_mtsdf");
//! # }
//! ```
//! ## Render SDFs to images
//! ```rust
//! # mod test_helpers;
//! # use test_helpers::compare_images;
//! # use image::io::Reader as ImageReader;
//! # use std::env;
//! # use msdf::{MSDF, SDFTrait};
//! #
//! # fn main() {
//! # use image::DynamicImage;
//! # let path = env::current_dir()
//! #     .unwrap()
//! #     .join("test_resources")
//! #     .join("doctest_msdf.png");
//! # let image = ImageReader::open(path).unwrap().decode().unwrap();
//! # let image = image.into_rgb32f();
//! #
//! // Load MSDF from an image::Rgb32FImage.
//! let msdf = MSDF::from_image(image, 10.0, 0.5);
//!
//! // Render to a 1024x1024 image.
//! let rendered = msdf.render(1024, 1024);
//!
//! // Render to a 1024x1024 image with edge colors.
//! let rendered_colored = msdf.render_colored(1024, 1024);
//!
//! // Do something with these images.
//! // let image: DynamicImage = DynamicImage::from(rendered);
//! // image.into_rgba8().save("myrenderedsdf.png").unwrap();
//! #
//! # let rendered: DynamicImage = DynamicImage::from(rendered);
//! # let rendered_colored: DynamicImage = DynamicImage::from(rendered_colored);
//! # let rendered = rendered.into_rgba8();
//! # let rendered_colored = rendered_colored.into_rgba8();
//! #
//! # compare_images(&rendered, "doctest_msdf_render");
//! # compare_images(&rendered_colored, "doctest_msdf_render_colored");
//! # }
//! ```

use std::os::raw::c_int;

use image::{ImageBuffer, Luma, Rgb32FImage, Rgba32FImage};

use msdf_sys::*;

#[cfg(test)]
mod test_helpers;
#[cfg(test)]
pub mod tests;

mod bitmap;
mod config;
mod loader;

pub use bitmap::*;
pub use config::*;
pub use loader::*;

#[derive(Debug)]
/// Type for errors emitted by the generator.
pub enum MsdfError {
    FreetypeInitializationFailure,
    FontLoadingFailure,
    GlyphLoadingFailure,
}

/// An msdfgen shape. Can be used to generate an SDF or Psuedo-SDF. Must be colored first using a
/// coloring function to generate a MSDF or MTSDF.
pub struct Shape {
    shape: msdfgen_Shape,
}

impl Shape {
    /// Assigns colors to edges of the shape in accordance to the multi-channel distance field
    /// technique. May split some edges if necessary. `angle` specifies the maximum angle (in
    /// radians) to be considered a corner, for example 3 (~172 degrees). Values below 1/2 PI will
    /// be treated as the external angle.
    pub fn color_edges_simple(mut self, angle: f64) -> ColoredShape {
        unsafe {
            msdfgen_edgeColoringSimple(&mut self.shape, angle, 0);
        } // hardcode seed for the time being

        ColoredShape(self)
    }

    /// The alternative "ink trap" coloring strategy is designed for better results with typefaces
    /// that use ink traps as a design feature. It guarantees that even if all edges that are
    /// shorter than both their neighboring edges are removed, the coloring remains consistent with
    /// the established rules.
    pub fn color_edges_ink_trap(mut self, angle: f64) -> ColoredShape {
        unsafe {
            msdfgen_edgeColoringInkTrap(&mut self.shape, angle, 0);
        }

        ColoredShape(self)
    }

    /// The alternative coloring by distance tries to use different colors for edges that are close
    /// together. This should theoretically be the best strategy on average. However, since it needs
    /// to compute the distance between all pairs of edges, and perform a graph optimization task,
    /// it is much slower than the rest.
    pub fn color_edges_by_distance(mut self, angle: f64) -> ColoredShape {
        unsafe {
            msdfgen_edgeColoringByDistance(&mut self.shape, angle, 0);
        }

        ColoredShape(self)
    }

    /// Generates a conventional single-channel signed distance field.
    pub fn generate_sdf(
        &self,
        width: u32,
        height: u32,
        range: f64,
        projection: &Projection,
        config: &SDFConfig,
    ) -> SDF {
        let image = ImageBuffer::<Luma<f32>, Vec<f32>>::new(width, height);

        let msdf = msdfgen_Bitmap {
            pixels: image.as_flat_samples().samples.as_ptr() as *mut f32,
            w: width as c_int,
            h: height as c_int,
            _phantom_0: Default::default(),
        };

        let projection = projection.as_msdfgen_projection();

        let config = msdfgen_GeneratorConfig {
            overlapSupport: config.overlap_support,
        };

        unsafe {
            msdfgen_generateSDF(
                &msdf as *const msdfgen_Bitmap<_> as *const _,
                &self.shape,
                &projection,
                range,
                &config,
            );
        }

        SDF::from_image(image, range, 0.5)
    }

    /// Generates a single-channel signed pseudo-distance field.
    pub fn generate_psuedo_sdf(
        &self,
        width: u32,
        height: u32,
        range: f64,
        projection: &Projection,
        config: &SDFConfig,
    ) -> SDF {
        let image = ImageBuffer::<Luma<f32>, Vec<f32>>::new(width, height);

        let msdf = msdfgen_Bitmap {
            pixels: image.as_flat_samples().samples.as_ptr() as *mut f32,
            w: width as c_int,
            h: height as c_int,
            _phantom_0: Default::default(),
        };

        let projection = projection.as_msdfgen_projection();

        let config = msdfgen_GeneratorConfig {
            overlapSupport: config.overlap_support,
        };

        unsafe {
            msdfgen_generatePseudoSDF(
                &msdf as *const msdfgen_Bitmap<_> as *const _,
                &self.shape,
                &projection,
                range,
                &config,
            );
        }

        SDF::from_image(image, range, 0.5)
    }
}

/// A shape that has been colored by one of the coloring functions. A shape must be colored first
/// before it can be used to generate an MSDF or MTSDF.
pub struct ColoredShape(Shape);

impl ColoredShape {
    /// Generates a conventional single-channel signed distance field.
    pub fn generate_sdf(
        &self,
        width: u32,
        height: u32,
        range: f64,
        projection: &Projection,
        config: &SDFConfig,
    ) -> SDF {
        self.0
            .generate_sdf(width, height, range, projection, config)
    }

    /// Generates a single-channel signed pseudo-distance field.
    pub fn generate_psuedo_sdf(
        &self,
        width: u32,
        height: u32,
        range: f64,
        projection: &Projection,
        config: &SDFConfig,
    ) -> SDF {
        self.0
            .generate_psuedo_sdf(width, height, range, projection, config)
    }

    /// Generates a multi-channel signed distance field.
    pub fn generate_msdf(
        &self,
        width: u32,
        height: u32,
        range: f64,
        projection: &Projection,
        config: &MSDFConfig,
    ) -> MSDF {
        let image = Rgb32FImage::new(width, height);

        let msdf = msdfgen_Bitmap {
            pixels: image.as_flat_samples().samples.as_ptr() as *mut f32,
            w: width as c_int,
            h: height as c_int,
            _phantom_0: Default::default(),
        };

        let projection = projection.as_msdfgen_projection();

        let config = config.as_msdfgen_config();

        unsafe {
            msdfgen_generateMSDF(
                &msdf as *const msdfgen_Bitmap<_> as *const _,
                &self.0.shape,
                &projection,
                range,
                &config,
            );
        }

        MSDF::from_image(image, range, 0.5)
    }

    /// Generates a multi-channel signed distance field with true distance in the alpha channel.
    pub fn generate_mtsdf(
        &self,
        width: u32,
        height: u32,
        range: f64,
        projection: &Projection,
        config: &MSDFConfig,
    ) -> MTSDF {
        let image = Rgba32FImage::new(width, height);

        let msdf = msdfgen_Bitmap {
            pixels: image.as_flat_samples().samples.as_ptr() as *mut f32,
            w: width as c_int,
            h: height as c_int,
            _phantom_0: Default::default(),
        };

        let projection = projection.as_msdfgen_projection();

        let config = config.as_msdfgen_config();

        unsafe {
            msdfgen_generateMTSDF(
                &msdf as *const msdfgen_Bitmap<_> as *const _,
                &self.0.shape,
                &projection,
                range,
                &config,
            );
        }

        MTSDF::from_image(image, range, 0.5)
    }
}
