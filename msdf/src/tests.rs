use crate::{GlyphLoader, Projection, SDFTrait, Shape, MSDF, MTSDF, SDF};
use image::DynamicImage;
use std::default::Default;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use ttf_parser::Face;

use crate::test_helpers::compare_images;
use image::io::Reader as ImageReader;
use mint::Vector2;

fn with_glyph<F: FnOnce(Shape, Projection)>(glyph: char, size: u32, callback: F) {
    let path = env::current_dir()
        .unwrap()
        .join("test_resources")
        .join("Roboto-Medium.ttf");
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut font_data = vec![];

    reader.read_to_end(&mut font_data).unwrap();

    let face = Face::from_slice(font_data.as_slice(), 0).unwrap();

    let glyph_index = face.glyph_index(glyph).unwrap();

    let bb = face.glyph_bounding_box(glyph_index).unwrap();

    let scale = Vector2 {
        x: 1.0 / 64.0,
        y: 1.0 / 64.0,
    };
    let translation = Vector2 {
        // try to come up with a decent fit
        x: (size as f64 * 64.0 - (bb.width() as f64)) / 2.0 - (bb.x_min as f64),
        y: (size as f64 * 64.0 - (bb.height() as f64)) / 2.0 - (bb.y_min as f64),
    };

    let projection = Projection { scale, translation };

    let shape = face.load_shape(glyph_index).unwrap();

    callback(shape, projection);
}

#[test]
fn can_generate_sdf() {
    with_glyph('A', 32, |shape, projection| {
        let shape = shape.color_edges_simple(3.0);

        let sdf = shape.generate_sdf(32, 32, 10.0 * 64.0, &projection, &Default::default());
        let sdf: DynamicImage = DynamicImage::from(sdf.to_image());
        let sdf = sdf.into_rgba8();

        compare_images(&sdf, "sdf");
    });
}

#[test]
fn can_generate_psdf() {
    with_glyph('B', 32, |shape, projection| {
        let shape = shape.color_edges_simple(3.0);

        let sdf = shape.generate_psuedo_sdf(32, 32, 10.0 * 64.0, &projection, &Default::default());
        let sdf: DynamicImage = DynamicImage::from(sdf.to_image());
        let sdf = sdf.into_rgba8();

        compare_images(&sdf, "psdf");
    });
}

#[test]
fn can_generate_msdf() {
    with_glyph('C', 32, |shape, projection| {
        let shape = shape.color_edges_simple(3.0);

        let sdf = shape.generate_msdf(32, 32, 10.0 * 64.0, &projection, &Default::default());
        let sdf: DynamicImage = DynamicImage::from(sdf.to_image());
        let sdf = sdf.into_rgba8();

        compare_images(&sdf, "msdf");
    });
}

#[test]
fn can_generate_mtsdf() {
    with_glyph('D', 32, |shape, projection| {
        let shape = shape.color_edges_simple(3.0);

        let sdf = shape.generate_mtsdf(32, 32, 10.0 * 64.0, &projection, &Default::default());
        let sdf: DynamicImage = DynamicImage::from(sdf.to_image());
        let sdf = sdf.into_rgba8();

        compare_images(&sdf, "mtsdf");
    });
}

#[test]
fn can_project_msdf() {
    with_glyph('E', 32, |shape, _| {
        let shape = shape.color_edges_simple(3.0);

        let projection = Projection {
            scale: Vector2 {
                x: 1.5 / 64.0,
                y: 0.5 / 64.0,
            },
            translation: Vector2 {
                x: 4.0 * 64.0,
                y: 4.0 * 64.0,
            },
        };

        let sdf = shape.generate_msdf(32, 32, 10.0 * 64.0, &projection, &Default::default());
        let sdf: DynamicImage = DynamicImage::from(sdf.to_image());
        let sdf = sdf.into_rgba8();

        compare_images(&sdf, "projected_msdf");
    });
}

#[test]
fn can_generate_unicode() {
    with_glyph('ç', 32, |shape, projection| {
        let shape = shape.color_edges_simple(3.0);

        let sdf = shape.generate_msdf(32, 32, 10.0 * 64.0, &projection, &Default::default());
        let sdf: DynamicImage = DynamicImage::from(sdf.to_image());
        let sdf = sdf.into_rgba8();

        compare_images(&sdf, "unicode_msdf");
    });
}

#[test]
fn can_render_sdf() {
    let path = env::current_dir()
        .unwrap()
        .join("test_resources")
        .join("sdf.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let sdf = img.to_luma32f();
    let sdf = SDF::from_image(sdf, 10.0, 0.5);

    let render = sdf.render(1024, 1024);
    let render = DynamicImage::from(render);
    let render = render.into_rgba8();

    compare_images(&render, "sdf_render");
}

#[test]
fn can_render_msdf() {
    let path = env::current_dir()
        .unwrap()
        .join("test_resources")
        .join("msdf.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let sdf = img.into_rgb32f();
    let sdf = MSDF::from_image(sdf, 10.0, 0.5);

    let render = sdf.render(1024, 1024);
    let render = DynamicImage::from(render);
    let render = render.into_rgba8();

    compare_images(&render, "msdf_render");
}

#[test]
fn can_render_msdf_colored() {
    let path = env::current_dir()
        .unwrap()
        .join("test_resources")
        .join("msdf.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let sdf = img.into_rgb32f();
    let sdf = MSDF::from_image(sdf, 10.0, 0.5);

    let render = sdf.render_colored(1024, 1024);
    let render = DynamicImage::from(render);
    let render = render.into_rgba8();

    compare_images(&render, "msdf_render_colored");
}

#[test]
fn can_render_mtsdf() {
    let path = env::current_dir()
        .unwrap()
        .join("test_resources")
        .join("mtsdf.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let sdf = img.into_rgba32f();
    let sdf = MTSDF::from_image(sdf, 10.0, 0.5);

    let render = sdf.render(1024, 1024);
    let render = DynamicImage::from(render);
    let render = render.into_rgba8();

    compare_images(&render, "mtsdf_render");
}

#[test]
fn can_render_mtsdf_colored() {
    let path = env::current_dir()
        .unwrap()
        .join("test_resources")
        .join("mtsdf.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let sdf = img.into_rgba32f();
    let sdf = MTSDF::from_image(sdf, 10.0, 0.5);

    let render = sdf.render_colored(1024, 1024);
    let render = DynamicImage::from(render);
    let render = render.into_rgba8();

    compare_images(&render, "mtsdf_render_colored");
}

#[test]
fn can_render_characters_with_overlap() {
    let path = env::current_dir()
        .unwrap()
        .join("test_resources")
        .join("unicode_msdf.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let sdf = img.into_rgb32f();
    let sdf = MSDF::from_image(sdf, 10.0, 0.5);

    let render = sdf.render(1024, 1024);
    let render = DynamicImage::from(render);
    let render = render.into_rgba8();

    compare_images(&render, "unicode_msdf_render");
}
