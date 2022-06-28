# msdf-rs

Rust bindings for [msdfgen](https://github.com/Chlumsky/msdfgen).
This crate attempts to bind msdfgen in a safe and idiomatic way.
Unsafe bindings provided by [msdf-sys](https://crates.io/crates/msdf-sys).

# Examples
## Generate SDFs
```rust
// Load a font from ttf data.
let face: Face;
let glyph_index = face.glyph_index('W').unwrap();

// Load a glyph into a shape using a ttf glyph index.
let shape = face.load_shape(glyph_index).unwrap();

// Not a required step for SDF and Psuedo-SDF generation. Other coloring options exist.
let colored_shape = shape.color_edges_simple(3.0);

// Using default projection.
let projection = Default::default();

// Using default configuration.
let sdf_config = Default::default();
let msdf_config = Default::default();

// Generate all types of SDF. Plain SDFs and Psuedo-SDFs do not require edge coloring.
let sdf   = colored_shape.generate_sdf(32, 32, 10.0, &projection, &sdf_config);
let psdf  = colored_shape.generate_psuedo_sdf(32, 32, 10.0, &projection, &sdf_config);
let msdf  = colored_shape.generate_msdf(32, 32, 10.0, &projection, &msdf_config);
let mtsdf = colored_shape.generate_mtsdf(32, 32, 10.0, &projection, &msdf_config);

// Do something with these SDFs.
// let image: DynamicImage = DynamicImage::from(msdf.to_image());
// image.into_rgba8().save("mysdf.png").unwrap();
```
## Render SDFs to images
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