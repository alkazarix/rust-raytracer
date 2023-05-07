

use super::material::{Material, TextureCoords};
use super::geometry::{Point, Vector3};
use super::ray::{Intersectable, Ray};

pub enum Element {
    Sphere(Sphere),
    Plane(Plane)
}

impl Element {
    pub fn material(&self) -> &Material {
        match self {
            Element::Sphere(s) => &s.material,
            Element::Plane(p) => &p.material
        }
    }
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Element::Sphere(s) => s.intersect(ray),
            Element::Plane(p) => p.intersect(ray)
            
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match self {
            Element::Sphere(s) => s.surface_normal(hit_point),
            Element::Plane(p) => p.surface_normal(hit_point)
            
        }
    }

    fn texture_coords(&self, hit_point: &Point) -> super::material::TextureCoords {
        match self {
            Element::Sphere(s) => s.texture_coords(hit_point),
            Element::Plane(p) => p.texture_coords(hit_point)
        }
    }
}

pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub material: Material
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &super::ray::Ray) -> Option<f64> {
        let normal = &self.normal.normalize();
        let denom = normal.dot(&ray.direction.normalize());
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance > 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point) -> Vector3 {
        -self.normal.normalize()
    }


    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {

        let mut x_axis = self.normal.cross(&Vector3::new(0.0, 0.0, 1.0));
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&Vector3::new(0.0, 1.0, 0.0));
        }
        let y_axis = self.normal.cross(&x_axis);
        let hit_vec = *hit_point - self.origin;

        TextureCoords {
            x: hit_vec.dot(&x_axis) as f32,
            y: hit_vec.dot(&y_axis) as f32,
        }

    }
}

pub struct Sphere {
    pub center: Point, 
    pub radius: f64,
    pub material: Material
}

impl Intersectable for Sphere {

    fn intersect(&self, ray: &super::ray::Ray) -> Option<f64> {
         //Create a line segment between the ray origin and the center of the sphere
        let c = self.center - ray.origin;
         //Use c as a hypotenuse and find the length of the adjacent side
        let b_length = c.dot(&ray.direction);
         //Find the length-squared of the opposite side
        let a_square = c.length() * c.length() - (b_length * b_length);
        let radius_square = self.radius * self.radius;

        if a_square > radius_square {
            return  None;
        }
        let tickness = (radius_square - a_square).sqrt();

        let t0 = b_length - tickness;
        let t1 = b_length + tickness;
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }
        else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let hit_vec = *hit_point - self.center;
        let (x,y,z) = hit_vec.coordinate();
        TextureCoords {
            x: (1.0 + (z.atan2(x) as f32) / std::f32::consts::PI) * 0.5,
            y: (y / self.radius).acos() as f32 / std::f32::consts::PI
        }
    }
}