use super::element::Element;
use super::geometry::Point;
use super::geometry::Vector3;
use super::material::TextureCoords;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn create_reflection(normal: Vector3, incident: Vector3, intersection: Point, bias: f64) -> Ray {
        Ray {
            origin: intersection + (normal * bias),
            direction: incident - (2.0 * incident.dot(&normal) * normal),
        }
    }

    pub fn create_shadow(hit_point: &Point, surface_normal: Vector3, light_direction: Vector3, bias: f64) -> Ray {
        Ray {
            origin: *hit_point + surface_normal * bias,
            direction: light_direction,
        }
    } 

    pub fn create_transmission(surface_normal: Vector3, incident: Vector3, intersection: Point, bias: f64, index: f32) -> Option<Ray> {
        let  mut normal = surface_normal;
        let mut eta_t = index as f64;
        let mut eta_i = 1.0f64;

        let mut i_dot_n = incident.dot(&normal);
        if i_dot_n < 0.0 {
            //Outside the surface
            i_dot_n = -i_dot_n;
        } else {
            //Inside the surface; invert the normal and swap the indices of refraction
            normal = -normal;
            eta_t = 1.0;
            eta_i = index as f64;
        }

        let eta = eta_i / eta_t;
        let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);
        if k < 0.0 {
            None
        } else {
            Some(Ray {
                origin: intersection + (normal * -bias),
                direction: (incident + i_dot_n * normal) * eta - normal * k.sqrt(),
            })
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords;
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,
}


impl<'a> Intersection<'a> {
    pub fn new(distance: f64, element: &'a Element) -> Intersection<'a> {
        Self{distance, element}
    }
}