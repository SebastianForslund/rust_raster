use std::ops::{Deref, DerefMut};

use cgmath::{Vector2, Vector4};
use sdl2::{rect::Point, render::Canvas, video::Window};

pub struct ObjectSpacePoint(pub Vector4<f32>);

impl ObjectSpacePoint {
  // Create a point in screen space from object space
  pub fn new(x: f32, y: f32, z: f32) -> ObjectSpacePoint {
    return ObjectSpacePoint(Vector4::new(x, y, z, 1.0));
  }
}

// Let my wrapper type be used as if it was a real Vector3
impl Deref for ObjectSpacePoint {
  type Target = Vector4<f32>;

  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

impl DerefMut for ObjectSpacePoint {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
  }
}

impl Into<ScreenSpacePoint> for &ObjectSpacePoint {
    fn into(self) -> ScreenSpacePoint {
      ScreenSpacePoint::new(&self)
    }
}

// ----- ScreenSpacePoint -------
pub struct ScreenSpacePoint(pub Vector2<f32>);

impl ScreenSpacePoint {
  // Create a point in screen space from object space
  pub fn new(obj_point: &ObjectSpacePoint) -> ScreenSpacePoint {
    let x_proj: f32 = obj_point.x as f32 / -obj_point.z as f32;
    let y_proj: f32 = obj_point.y as f32 / -obj_point.z as f32;

    return ScreenSpacePoint(Vector2::new(x_proj, y_proj));
  }
}

// Let my wrapper type be used as if it was a real Vector2
impl Deref for ScreenSpacePoint {
  type Target = Vector2<f32>;

  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

impl DerefMut for ScreenSpacePoint {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
  }
}

// ----- PixelPoint -----
pub struct PixelPoint<'a> {
  pub vec: Vector2<u32>,
  canvas: &'a Canvas<Window>,
}

impl<'a> PixelPoint<'a> {
  pub fn new(input: &ScreenSpacePoint, canvas: &'a Canvas<Window>) -> PixelPoint<'a> {
    // Convert the screen space point to a raster space point
    let x = ((input.x + 1.0) / 2.0 * canvas.window().size().0 as f32) as u32;
    let y = ((input.y + 1.0) / 2.0 * canvas.window().size().1 as f32) as u32;
    let v: Vector2<u32> = Vector2::new(x, y);
    PixelPoint {vec: v, canvas: canvas}
  }

}

impl Into<Point> for PixelPoint<'_> {
    fn into(self) -> Point {
      Point::new(self.vec.x as i32, self.vec.y as i32)
    }
}
