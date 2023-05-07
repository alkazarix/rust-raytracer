use std::path::Path;

use image::ImageFormat;
use raytracer::raytracer::{element::{Sphere, Plane, Element}, material::{Material, Color, Coloration, Texture, SurfaceType}, geometry::{Point, Vector3}, scene::{Scene, render}, light::{ Light, SphericalLight}};

fn main() {

    let texture_sphere = Texture::load_texture("textures/checkerboard.png").expect("could not load texture");
    let texture_plane = Texture::load_texture("textures/checkerboard.png").expect("could not load texture");
    let width: u32 = 800;
    let height: u32 = 600;
    let sphere_green = Sphere {
        center: Point::new(0.0, 0.0, -5.0),
        radius: 0.75,
        material: Material { 
            coloration: Coloration::Color(Color::new(0.4, 1.0, 0.4)), 
            albedo: 0.18,
            surface: SurfaceType::Reflective { reflectivity: 0.7 }
        }
    };

    let sphere_blue = Sphere {
        center: Point::new(-2.0, 1.0, -6.0),
        radius: 1.5,
        material: Material { 
            coloration: Coloration::Texture(texture_sphere), 
            albedo: 0.58,
            surface: SurfaceType::Diffuse
        }
    };

    let sphere_red = Sphere {
        center: Point::new(1.0, 1.5, -4.0),
        radius: 1.5,
        material: Material { 
            coloration: Coloration::Color(Color::new(0.8, 0.1, 0.1)), 
            albedo: 0.18,
            surface: SurfaceType::Diffuse
        }

    };

    let plane_horizontal = Plane {
        origin: Point::new(0.0, -2.0, -5.0),
        normal: Vector3::new(0.0, -1.0, 0.0),
        material: Material { 
            coloration: Coloration::Texture(texture_plane), 
            albedo: 0.18,
            surface:  SurfaceType::Reflective { reflectivity: 0.5 }
        }
    };


    let light_1 = SphericalLight {
        position: Point::new(-2.0, 5.0, -3.0),
        color: Color::new(0.9, 0.9, 0.9),
        intensity: 1000.0
    };

    let light_2 = SphericalLight {
        position: Point::new(0.25, 0.0, -2.0),
        color: Color::new(0.2, 0.2, 0.5),
        intensity: 250.0
    };



    let elements= vec![
        Element::Sphere(sphere_green),
        Element::Sphere(sphere_blue),
        Element::Sphere(sphere_red),
        Element::Plane(plane_horizontal),
    ];

    let lights: Vec<Light> = vec![Light::SphericalLight(light_1), Light::SphericalLight(light_2)];

    let scene = Scene::new(height, width, elements, lights);
    let image = render(&scene);

    let image_path = Path::new("sphere.png");
    image.save_with_format(image_path, ImageFormat::Png).unwrap();
}
