use super::{material::Color, geometry::{Point, Vector3}};


pub enum Light {
    DirectionalLight(DirectionalLight),
    SphericalLight(SphericalLight)
}

impl  Light {
    pub fn color(&self) -> Color {
        match self {
            Light::SphericalLight(s) => s.color,
            Light::DirectionalLight(s) => s.color,

        }
    }

    pub fn intensity(&self, hit_point: &Point) -> f32 {
        match self {
            Light::SphericalLight(s) => s.intensity(hit_point),
            Light::DirectionalLight(s) => s.intensity,
        }
    }

    pub fn distance(&self, hit_point: &Point) -> f64 {
        match self {
            Light::SphericalLight(s) => s.distance(hit_point),
            Light::DirectionalLight(s) => s.distance(hit_point),
        }
    }

    pub fn direction_from(&self, hit_point: &Point) -> Vector3 {
        match self {
            Light::SphericalLight(s) => s.direction_from(hit_point),
            Light::DirectionalLight(s) => s.direction_from(hit_point),
        }
    }

}


pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

impl DirectionalLight {
    pub fn new(direction: Vector3, color: Color, intensity: f32) -> Self {
        Self { direction, color, intensity }
    }

    fn distance(&self, _: &Point) -> f64 {
        std::f64::MAX
    }

    fn direction_from(&self, _: &Point) -> Vector3 {
        -self.direction.normalize()
    }
}


pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

impl SphericalLight {
    pub fn new(position: Point, color: Color, intensity: f32) -> Self {
        Self { position, color, intensity }
    }


    fn distance(&self, hit_point: &Point) -> f64 {
        (self.position - *hit_point).length()
    }

    fn direction_from(&self, hit_point: &Point) -> Vector3 {
        (self.position - *hit_point).normalize()
    }

    fn intensity(&self, hit_point: &Point) -> f32 {
        let r2 = (self.position - *hit_point).length().powi(2) as f32;
        self.intensity / (4.0 * ::std::f32::consts::PI * r2)
    }
}
