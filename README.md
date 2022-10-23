# msdf-rs

[![Licence](https://img.shields.io/github/license/Penple/msdf-rs?color=%23537aed)](https://github.com/Penple/msdf-rs/blob/main/LICENSE)
[![crates.io](https://img.shields.io/crates/v/msdf)](https://crates.io/crates/msdf)
[![Documentation](https://img.shields.io/docsrs/msdf)](https://docs.rs/msdf/latest/msdf/)

Rust bindings for [msdfgen](https://github.com/Chlumsky/msdfgen).
This crate attempts to bind msdfgen in a safe and idiomatic way.
Unsafe bindings provided by [msdf-sys](https://crates.io/crates/msdf-sys).

## Building

In order to build [msdf-sys](https://crates.io/crates/msdf-sys) on Linux, Clang must be installed.

## Examples

### Generate SDFs

```rust
// Load a font from ttf data.
let face: Face;
let glyph_index = face.glyph_index('W').unwrap();

// Load a glyph into a shape using a ttf glyph index.
let shape = face.load_shape(glyph_index).unwrap();

// Not a required step for SDF and Psuedo-SDF generation. Other coloring options exist.
let colored_shape = shape.color_edges_simple(3.0);

// Project glyph down by a factor of 64x.
let projection = Projection {
    scale: Vector2 { x: 1.0 / 64.0, y: 1.0 / 64.0 },
    translation: Vector2 { x: 0.0, y: 0.0 },
};

// Using default configuration.
let sdf_config = Default::default();
let msdf_config = Default::default();

// Generate all types of SDF. Plain SDFs and Psuedo-SDFs do not require edge coloring.
let sdf   = colored_shape.generate_sdf(32, 32, 10.0 * 64.0, &projection, &sdf_config);
let psdf  = colored_shape.generate_psuedo_sdf(32, 32, 10.0 * 64.0, &projection, &sdf_config);
let msdf  = colored_shape.generate_msdf(32, 32, 10.0 * 64.0, &projection, &msdf_config);
let mtsdf = colored_shape.generate_mtsdf(32, 32, 10.0 * 64.0, &projection, &msdf_config);

// Do something with these SDFs.
// let image: DynamicImage = DynamicImage::from(msdf.to_image());
// image.into_rgba8().save("mysdf.png").unwrap();
```

### Render SDFs to images

```rust
// Load MSDF from an image::Rgb32FImage.
let msdf = MSDF::from_image(image, 10.0, 0.5);

// Render to a 1024x1024 image.
let rendered = msdf.render(1024, 1024);

// Render to a 1024x1024 image with edge colors.
let rendered_colored = msdf.render_colored(1024, 1024);

// Do something with these images.
// let image: DynamicImage = DynamicImage::from(rendered);
// image.into_rgba8().save("myrenderedsdf.png").unwrap();
```
