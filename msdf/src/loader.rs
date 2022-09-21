use crate::Shape;
use mint::Vector2;
use msdf_sys::{
    msdfgen_Contour, msdfgen_EdgeColor_WHITE, msdfgen_EdgeHolder, msdfgen_Point2, msdfgen_Shape,
};
use std::alloc::{alloc, Layout};
use ttf_parser::{Face, GlyphId, OutlineBuilder};

fn point_from_font_coords(x: f32, y: f32) -> msdfgen_Point2 {
    msdfgen_Point2 {
        x: x as f64,
        y: y as f64,
    }
}

pub trait GlyphLoader {
    type Glyph;

    fn load_shape(&self, glyph: Self::Glyph) -> Option<Shape>;
}

struct ShapeOutlineBuilder {
    shape: msdfgen_Shape,
    cur_pos: Vector2<f32>,
    current_contour: Option<*mut msdfgen_Contour>,
}

impl From<ShapeOutlineBuilder> for Shape {
    fn from(mut outline: ShapeOutlineBuilder) -> Self {
        outline.shape.inverseYAxis = true;
        Shape {
            shape: outline.shape,
        }
    }
}

impl OutlineBuilder for ShapeOutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        unsafe {
            self.current_contour = Some(self.shape.addContour1());
        }
        self.cur_pos = Vector2 { x, y };
    }

    fn line_to(&mut self, x: f32, y: f32) {
        unsafe {
            let layout = Layout::new::<msdfgen_EdgeHolder>();
            let ptr = alloc(layout) as *mut msdfgen_EdgeHolder;
            *ptr = msdfgen_EdgeHolder::new2(
                point_from_font_coords(self.cur_pos.x, self.cur_pos.y),
                point_from_font_coords(x, y),
                msdfgen_EdgeColor_WHITE,
            );
            self.current_contour.unwrap().as_mut().unwrap().addEdge(ptr);
        }
        self.cur_pos = Vector2 { x, y };
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        unsafe {
            let layout = Layout::new::<msdfgen_EdgeHolder>();
            let ptr = alloc(layout) as *mut msdfgen_EdgeHolder;
            *ptr = msdfgen_EdgeHolder::new3(
                point_from_font_coords(self.cur_pos.x, self.cur_pos.y),
                point_from_font_coords(x1, y1),
                point_from_font_coords(x, y),
                msdfgen_EdgeColor_WHITE,
            );
            self.current_contour.unwrap().as_mut().unwrap().addEdge(ptr);
        }
        self.cur_pos = Vector2 { x, y };
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        unsafe {
            let layout = Layout::new::<msdfgen_EdgeHolder>();
            let ptr = alloc(layout) as *mut msdfgen_EdgeHolder;
            *ptr = msdfgen_EdgeHolder::new4(
                point_from_font_coords(self.cur_pos.x, self.cur_pos.y),
                point_from_font_coords(x1, y1),
                point_from_font_coords(x2, y2),
                point_from_font_coords(x, y),
                msdfgen_EdgeColor_WHITE,
            );
            self.current_contour.unwrap().as_mut().unwrap().addEdge(ptr);
        }
        self.cur_pos = Vector2 { x, y };
    }

    fn close(&mut self) {
        self.current_contour = None;
    }
}

impl GlyphLoader for Face<'_> {
    type Glyph = GlyphId;

    fn load_shape(&self, glyph: Self::Glyph) -> Option<Shape> {
        let mut builder = unsafe {
            ShapeOutlineBuilder {
                shape: msdfgen_Shape::new(),
                cur_pos: Vector2 { x: 0.0, y: 0.0 },
                current_contour: None,
            }
        };
        if self.outline_glyph(glyph, &mut builder).is_some() {
            Some(builder.into())
        } else {
            None
        }
    }
}
