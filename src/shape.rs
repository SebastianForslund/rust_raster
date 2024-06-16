use cgmath::{Matrix4, Rad};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::gfx::primitives::{self};

use crate::coordinate::{self, ObjectSpacePoint};
use coordinate::PixelPoint;
use coordinate::ScreenSpacePoint;

use sdl2::render::Canvas;
use sdl2::video::Window;

fn rot_matrix_x(angle: Rad<f32>) -> Matrix4<f32> {
  Matrix4::new(1.0, 0.0,           0.0,            0.0,
               0.0, angle.0.cos(), -angle.0.sin(), 0.0,
               0.0, angle.0.sin(), angle.0.cos(),  0.0,
               0.0, 0.0,           0.0,            1.0)
}

fn rot_matrix_y(angle: Rad<f32>) -> Matrix4<f32> {
  Matrix4::new(angle.0.cos(),  0.0, angle.0.sin(), 0.0,
               0.0,            1.0, 0.0,           0.0,
               -angle.0.sin(), 0.0, angle.0.cos(), 0.0,
              0.0,             0.0, 0.0,           1.0)
}

fn rot_matrix_z(angle: Rad<f32>) -> Matrix4<f32> {
  Matrix4::new(angle.0.cos(), -angle.0.sin(), 0.0, 0.0,
               angle.0.sin(), angle.0.cos(),  0.0, 0.0,
               0.0,           0.0,            1.0, 0.0,
               0.0,           0.0,            0.0, 1.0)
}

fn draw_point(canvas: &mut Canvas<Window>, point: &ScreenSpacePoint) -> Result<(), String> {
  let pp = PixelPoint::new(point, canvas);
  let p: Point = pp.into();
  primitives::DrawRenderer::filled_circle(canvas, p.x as i16, p.y as i16, 3, Color::YELLOW)
}


fn draw_line(canvas: &mut Canvas<Window>, p1: &ScreenSpacePoint, p2: &ScreenSpacePoint) -> Result<(), String> {
  let pp_first = PixelPoint::new(p1, canvas);
  let pp_second = PixelPoint::new(p2, canvas);

  primitives::DrawRenderer::line(canvas, pp_first.vec.x as i16, pp_first.vec.y as i16,
                                         pp_second.vec.x as i16, pp_second.vec.y as i16,
                                         Color::RED)
}

pub struct Shape {
  points: Vec<ObjectSpacePoint>,
}

impl Shape {
  pub fn new() -> Shape {
    Shape { points: Vec::new() }
  }

  pub fn add_point(&mut self, point: ObjectSpacePoint) {
    self.points.push(point);
  }

  pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
    for p in &self.points {
      // Is this how i should be doing this conversion?
      draw_point(canvas, &p.into())?;
    }

    for outer in &self.points {
      for inner in &self.points {
        draw_line(canvas, &outer.into(), &inner.into())?;
      }
    }

    Ok(())
  }

  pub fn rotate_x(&mut self, angle: Rad<f32>) {
    let m = rot_matrix_x(angle);

    self.points.iter_mut().for_each(|p| *p = ObjectSpacePoint(m * p.0));
  }

  pub fn rotate_y(&mut self, angle: Rad<f32>) {
    let m = rot_matrix_y(angle);

    self.points.iter_mut().for_each(|p| *p = ObjectSpacePoint(m * p.0));
  }

  pub fn rotate_z(&mut self, angle: Rad<f32>) {
    let m = rot_matrix_z(angle);

    self.points.iter_mut().for_each(|p| *p = ObjectSpacePoint(m * p.0));
  }
}
