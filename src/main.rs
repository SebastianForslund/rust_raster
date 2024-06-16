extern crate sdl2;

mod shape;
use cgmath::Rad;
use coordinate::ObjectSpacePoint;
use shape::Shape;

mod coordinate;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Duration, Instant};

// TODO: Add some struct to keep track of multiple shapes
fn draw_shit(canvas: &mut Canvas<Window>, shape: &Shape) -> Result<(), String> {

    shape.draw(canvas)?;

    Ok(())
}

pub fn main() -> Result<(), String> {
    // FPS shite
    let fps = 144.0;
    let frame_time = Duration::from_secs_f32(1.0/fps);
    let mut fps_counter_mono_start = Instant::now();
    let mut fps_counter = 0;

    // Initialize SDL
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Rust drawing program", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut shape: Shape = Shape::new();
    // Would be nice to do this through an initializer list instead
    shape.add_point(ObjectSpacePoint::new(1.0, 1.0, -7.0));
    shape.add_point(ObjectSpacePoint::new(1.0, 1.0, -5.0));
    shape.add_point(ObjectSpacePoint::new(1.0, -1.0, -7.0));
    shape.add_point(ObjectSpacePoint::new(1.0, -1.0, -5.0));
    shape.add_point(ObjectSpacePoint::new(-1.0, 1.0, -7.0));
    shape.add_point(ObjectSpacePoint::new(-1.0, 1.0, -5.0));
    shape.add_point(ObjectSpacePoint::new(-1.0, -1.0, -7.0));
    shape.add_point(ObjectSpacePoint::new(-1.0, -1.0, -5.0));

    // Event loop
    'running: loop {
        let start_mono = Instant::now();
        let end_mono;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_shit(&mut canvas, &shape)?;
        canvas.present();

        // Perform a rotation
        //shape.rotate_x(Rad(0.01));
        shape.rotate_y(Rad(0.01));
        shape.rotate_z(Rad(0.01));

        // FPS counter
        fps_counter += 1;
        end_mono = Instant::now();
        if end_mono - fps_counter_mono_start > Duration::from_secs(1) {
            println!("FPS: {}", fps_counter);
            fps_counter_mono_start = end_mono;
            fps_counter = 0;
        }

        // Cap the FPS
        let diff = end_mono - start_mono;
        if diff < frame_time {
            // Sleep the remainder of the desired frame time
            std::thread::sleep(frame_time - diff);
        }
    }

    Ok(())
}
