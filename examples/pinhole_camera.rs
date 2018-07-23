extern crate cgmath;
extern crate graphics;
extern crate sdl2;
extern crate num_traits;
extern crate timing;

use cgmath::*;
use graphics::rasterizer::*;
use graphics::geometry::*;
use graphics::sdl_utils::*;
use sdl2::{
    event::Event,
    pixels::Color,
};
use timing::Timers;
use graphics::frame::*;
use graphics::textures::*;
use graphics::render::*;
use graphics::camera::*;

fn main() {
    let ctx = sdl2::init().unwrap();
    let mut events = ctx.event_pump().unwrap();
    let mut timers = Timers::new();
    let canvas = create_sdl_canvas(&ctx, 1000, 800);
    let rasterizer = Rasterizer::create(1000, 800);
    let mut renderer = Renderer::new(rasterizer, canvas);

    let mut texture_frame = Frame::new(
        128,
        128,
        Color::RGB(255, 255, 255)
    );
    for x in 0..128 {
        for y in 0..128 {
            if ((x / 8) + (y / 8)) % 2 == 0 {
                texture_frame.set(x, y, Color::RGB(0, 0, 255));
            }
        }
    }
    let texture = Texture::create(texture_frame);
    renderer.set_texture(0, texture);
    let mut camera = Camera::create(
        70.0,
        10.0 / 8.0,
        0.1,
        1000.0,
        Matrix4::from_translation(Vector3{x: 0.0, y: 0.6, z: 2.0}),
    );
    renderer.set_from_camera(&camera);

    let mesh = Mesh::xy_face(2.5)
        .transformed(Matrix4::from_angle_x(Deg(-90.0)));
    let mut mesh2 = Mesh::sphere(0.5, 4)
        .transformed(Matrix4::from_translation(Vector3{x: 0.0, y: 0.5, z: 0.0}));

    'main: loop {

        timers.start("Rendering plane");
        renderer.mesh(&mesh);
        timers.stop("Rendering plane");
        timers.start("Rendering cube");
        renderer.mesh(&mesh2);
        timers.stop("Rendering cube");

        timers.start("Presentation");
        renderer.present();
        timers.stop("Presentation");

        mesh2.transform(Matrix4::from_angle_y(Deg(0.3)));

        {
            events.pump_events();
            let keyboard_state = events.keyboard_state();
            camera.control_with_keyboard(0.04, 0.02, &keyboard_state);
        }

        renderer.set_from_camera(&camera);

        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main,
                _               => continue
            }
        }

    }

    println!("{}", timers);
}


