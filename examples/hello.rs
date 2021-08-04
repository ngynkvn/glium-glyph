extern crate glium;
extern crate glium_glyph;
extern crate glyph_brush;

use glium::glutin::{Api, GlProfile, GlRequest};
use glium::{glutin, Surface};

use glium_glyph::GlyphBrush;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glyph_brush::ab_glyph::{FontArc};
use glyph_brush::{Section, Text};

pub fn main() {
    let event_loop = EventLoop::new();
    let window = glutin::window::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)))
        .with_srgb(true);
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let dejavu = FontArc::try_from_slice(include_bytes!("../fonts/DejaVuSans-2.37.ttf")).unwrap();
    let fonts = vec![dejavu];

    let mut glyph_brush = GlyphBrush::new(&display, fonts);

    event_loop.run(move |event, _tgt, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            _ => (),
        }
        let screen_dims = display.get_framebuffer_dimensions();

        glyph_brush.queue(
            Section::default()
                .add_text(Text::new("Hello, world!"))
                .with_bounds((screen_dims.0 as f32, screen_dims.1 as f32 / 2.0)),
        );
        glyph_brush.queue(
            Section::default()
                .add_text(Text::new("Hello, world!"))
                .with_bounds((screen_dims.0 as f32, screen_dims.1 as f32 / 2.0))
                .with_screen_position((0.0, screen_dims.1 as f32 / 2.0)),
        );

        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 0.0), 1.0);
        glyph_brush.draw_queued(&display, &mut target);
        target.finish().unwrap();
    });
}
