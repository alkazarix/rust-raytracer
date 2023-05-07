

use image::{DynamicImage, GenericImage};
use super::{element::Element, camera::Camera, material::{Color, SurfaceType}, ray::{Intersectable, Intersection, Ray}, light::Light, geometry::{Vector3, Point}};

const MAX_RECURSION_DEPTH : i32 = 10;

pub struct Scene {
    pub height: u32, 
    pub width: u32, 
    pub elements:  Vec<Element>,
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub shadow_bias: f64,
}


impl Scene {
    pub fn new(height: u32, width: u32, elements: Vec<Element>, lights: Vec<Light>) -> Self {
        let aspect_ratio = (width as f64) / (height as f64);
        let camera = Camera::default_with_aspect_ratio(aspect_ratio);
        Self {height, width, elements , camera, lights,  shadow_bias: 1e-13}
    }

    pub fn dimension(&self)  -> (u32, u32){
        (self.width, self.height)
    }

    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|elt| elt.intersect(ray).map(|d| Intersection::new(d, elt)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())

    }
}

fn shade_diffuse(scene: &Scene, element: &Element, hit_point: Point, surface_normal: Vector3)  -> Color {
    let mut color  = Color::black();
    let texture_coord = element.texture_coords(&hit_point);

    for light in &scene.lights {
        let direction_to_light = light.direction_from(&hit_point);
        let shadow_ray = Ray::create_shadow(&hit_point, surface_normal, direction_to_light, scene.shadow_bias); 

        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none() ||
                       shadow_intersection.unwrap().distance > light.distance(&hit_point);

        let light_intensity = if in_light { light.intensity(&hit_point)} else { 0.0 };
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = element.material().albedo / std::f32::consts::PI;

        let light_color = light.color() * light_power * light_reflected;

        color = color +  element.material().coloration.color(&texture_coord).clone() * light_color;

    }

    color.clamp()
   

}


fn fresnel(incident: Vector3, normal: Vector3, index: f32) -> f64 {
    let i_dot_n = incident.dot(&normal);
    let mut eta_i = 1.0;
    let mut eta_t = index as f64;
    if i_dot_n > 0.0 {
        eta_i = eta_t;
        eta_t = 1.0;
    }

    let sin_t = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
    if sin_t > 1.0 {
        //Total internal reflection
        return 1.0;
    } else {
        let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
        let cos_i = cos_t.abs();
        let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
        let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
        return (r_s * r_s + r_p * r_p) / 2.0;
    }
}


fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: i32)  -> Color {
    let hit_point = ray.origin + ray.direction * intersection.distance;
    let surface_normal = intersection.element.surface_normal(&hit_point);
    let material = intersection.element.material();

    match  material.surface {
         SurfaceType::Diffuse =>  shade_diffuse(scene, intersection.element, hit_point, surface_normal),
         SurfaceType::Reflective{reflectivity} => {
            let mut color = shade_diffuse(scene, intersection.element, hit_point, surface_normal);
            let reflective_ray = Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);
            color = color * (1.0 - reflectivity);
            color = color + trace_ray(scene, &reflective_ray, depth + 1);
            color
         },
         SurfaceType::Refractive { index, transparency } => {
            let mut refraction_color = Color::black();
            let kr = fresnel(ray.direction, surface_normal, index) as f32;
            let surface_color = material.coloration
                .color(&intersection.element.texture_coords(&hit_point));

            if kr < 1.0 {
                let transmission_ray =
                    Ray::create_transmission(surface_normal, ray.direction, hit_point, scene.shadow_bias, index);
                if transmission_ray.is_some() {
                    refraction_color = trace_ray(scene, &transmission_ray.unwrap(), depth + 1);
                } 
            }

            let reflective_ray = Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);
            let reflection_color = trace_ray(scene, &reflective_ray, depth + 1);
            let mut color = reflection_color * kr + refraction_color * (1.0 - kr);
            color = color * transparency * surface_color;

            color
         }
    }
}

fn trace_ray(scene: &Scene, ray: &Ray, depth: i32) -> Color {

    if depth >= MAX_RECURSION_DEPTH {
        return Color::black();
    }

    let intersection = scene.trace(&ray);
    intersection.map(|i| get_color(&scene, &ray, &i, depth))
            .unwrap_or(Color::black())
} 

pub fn render(scene: &Scene) -> DynamicImage{
    let (width, height) = scene.dimension();
    let mut image = DynamicImage::new_rgb8(width, height);



    for x in 0..scene.width {
        for y in 0..scene.height {
            let xx = ((x as f64) + 0.5)/ (width as f64);
            let yy = ((y as f64) + 0.5)/ (height as f64);
            let ray = scene.camera.get_ray(xx, yy);

            let color = trace_ray(scene, &ray, 0);

            image.put_pixel(x, y, color.to_rgba());

            // let intersection = scene.trace(&ray);
            // let color = intersection.map(|i| get_color(&scene, &ray, &i).to_rgba())
            //     .unwrap_or(Color::new(0.2, 0.2, 0.6).to_rgba());
            
        }
    }

    image
}