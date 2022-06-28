#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(clippy::all)]
mod sys {
    // to make clippy happy
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use sys::*;

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, ImageBuffer, Luma, Rgb32FImage, RgbImage};
    use std::os::raw::c_int;
    use std::{env, iter, ptr};

    fn compare_images(image: &RgbImage, path: &str) {
        // i'm hoping that floating point optimizations and whatnot do not affect the results enough that a bitwise comparison fails
        let path = env::current_dir()
            .unwrap()
            .join("test_resources")
            .join(path)
            .with_extension("png");
        let truth = image::open(path).expect("Unable to open test resource");
        let truth = truth.into_rgb8(); // make sure we're working in rgb8

        if image.len() != truth.len() {
            panic!("Image sizes do not match.");
        }

        for pixel in iter::zip(image.pixels(), truth.pixels()) {
            if pixel.0 != pixel.1 {
                panic!("Images do not match.");
            }
        }
    }

    #[test]
    fn can_generate_sdf_from_shape() {
        unsafe {
            let shape_desc = "{ 1471,0; 1149,0; 1021,333; 435,333; 314,0; 0,0; 571,1466; 884,1466; # }{ 926,580; 724,1124; 526,580; # }";
            let mut shape = msdfgen_Shape::new();
            let mut colors_specified = false;

            msdfgen_readShapeDescription1(
                shape_desc.as_ptr() as *const _,
                &mut shape,
                &mut colors_specified,
            );

            shape.normalize();
            shape.inverseYAxis = true;

            msdfgen_edgeColoringSimple(&mut shape, 3.0, 0);

            let image = ImageBuffer::<Luma<f32>, Vec<f32>>::new(16, 16);

            let msdf = msdfgen_Bitmap {
                pixels: image.as_flat_samples().samples.as_ptr() as *mut f32,
                w: image.width() as c_int,
                h: image.height() as c_int,
                _phantom_0: Default::default(),
            };

            let scale = msdfgen_Vector2::new1(0.01, 0.01);
            let translation = msdfgen_Vector2::new1(100.0, 100.0);
            let projection = msdfgen_Projection::new1(&scale, &translation);

            let config = msdfgen_GeneratorConfig {
                overlapSupport: true,
            };

            msdfgen_generateSDF(
                &msdf as *const msdfgen_Bitmap<f32> as *const _,
                &shape,
                &projection,
                200.0,
                &config,
            );

            let image: DynamicImage = image.into();
            let image = image.into_rgb8();

            compare_images(&image, "sdf");
        }
    }

    #[test]
    fn can_generate_msdf_from_shape() {
        unsafe {
            let shape_desc = "{ 1471,0; 1149,0; 1021,333; 435,333; 314,0; 0,0; 571,1466; 884,1466; # }{ 926,580; 724,1124; 526,580; # }";
            let mut shape = msdfgen_Shape::new();
            let mut colors_specified = false;

            msdfgen_readShapeDescription1(
                shape_desc.as_ptr() as *const _,
                &mut shape,
                &mut colors_specified,
            );

            shape.normalize();
            shape.inverseYAxis = true;

            msdfgen_edgeColoringSimple(&mut shape, 3.0, 0);

            let image = Rgb32FImage::new(16, 16);

            let msdf = msdfgen_Bitmap {
                pixels: image.as_flat_samples().samples.as_ptr() as *mut f32,
                w: image.width() as c_int,
                h: image.height() as c_int,
                _phantom_0: Default::default(),
            };

            let scale = msdfgen_Vector2::new1(0.01, 0.01);
            let translation = msdfgen_Vector2::new1(100.0, 100.0);
            let projection = msdfgen_Projection::new1(&scale, &translation);

            let config = msdfgen_MSDFGeneratorConfig {
                _base: msdfgen_GeneratorConfig {
                    overlapSupport: true,
                },
                errorCorrection: msdfgen_ErrorCorrectionConfig {
                    mode: 0,
                    distanceCheckMode: 0,
                    minDeviationRatio: 0.0,
                    minImproveRatio: 0.0,
                    buffer: ptr::null_mut(),
                },
            };

            msdfgen_generateMSDF(
                &msdf as *const msdfgen_Bitmap<f32> as *const _,
                &shape,
                &projection,
                200.0,
                &config,
            );

            let image: DynamicImage = image.into();
            let image = image.into_rgb8();

            compare_images(&image, "msdf");
        }
    }
}
