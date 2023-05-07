
use crate::raytracer::geometry::Vector3;
use super::{geometry::Point, ray::Ray};

pub struct Camera {
    look_from: Point,
    look_at: Point,
    vup: Vector3,
    vfov: f64, // vertical field-of-view in degrees
    aspect_ratio: f64 
} 

impl Camera {

    pub fn default_with_aspect_ratio(aspect_ratio: f64) -> Self {
        let vfov: f64 = 90.0; //90° 

        let look_from = Point::zero();
        let look_at = Point::new(0.0, 0.0, -1.0);

        let vup =  Vector3::new(0.0, -1.0, 0.0);

        Self { look_from, look_at, vup, vfov, aspect_ratio}
    }

    fn focal_dimension(&self) -> (f64, f64) {
        let theta = (self.vfov / 2.0).to_radians();
        let height = theta.tan();
        let width =  self.aspect_ratio * height;
        (height, width)
    }

    fn coordinate_system(&self) -> (Vector3, Vector3, Vector3) {
        let w = (self.look_at - self.look_from).normalize();
        let u = self.vup.cross(&w).normalize();
        let v = w.cross(&u);
        (u,v,w)
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {

        let origin = self.look_from;
        let (u,v, _) = self.coordinate_system();
        let (height, width) = self.focal_dimension();
        let lower_left_corner = self.look_at - (u * (width / 2.0)) - (v * (height / 2.0));

        let direction = lower_left_corner + (x * width * u) + ( y * height * v) - origin;

        Ray::new(
            origin,
            direction.normalize(),
        )
    }

    pub fn get_aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }
}