use std::ops::{Mul, Add};

use image::{Rgba, DynamicImage, GenericImageView, Pixel};

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
     encoded.powf(GAMMA)
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn black() -> Self {
        Self { red: 0.0, blue: 0.0, green: 0.0 }

    }  

    pub fn blue() -> Self {
        Self { red: 0.2, blue: 0.8, green: 0.2}

    } 

    pub fn new(red: f32,  green: f32, blue: f32) -> Self{
        Self { red, green, blue }
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba([
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255,
        ])
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        let channels = rgba.channels();
        Color {
            red: gamma_decode((channels[0] as f32) / 255.0),
            green: gamma_decode((channels[1] as f32) / 255.0),
            blue : gamma_decode((channels[2] as f32) / 255.0),
        }
    }
}


impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color { 
            red: self.red * other.red, 
            green: self.green * other.green, 
            blue: self.blue * other.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color { 
            red: self.red * other, 
            green: self.green * other, 
            blue: self.blue * other,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color { 
            red: self.red + other.red, 
            green: self.green + other.green, 
            blue: self.blue + other.blue,
        }
    }
}

pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { index: f32, transparency: f32 },
}


pub struct Material {
    pub coloration: Coloration,
    pub albedo: f32,
    pub surface: SurfaceType
}

pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
}

pub struct Texture {
    pub image: DynamicImage
}

impl Texture {

    pub fn load_texture(path: &str) -> Result<Texture, String> {
        let image = image::open(path).map_err(|_| String::from("Texture not found!"))?;
        Ok(Texture {image})
    }
}

pub enum Coloration {
    Color(Color),
    Texture(Texture)
}

impl Coloration  {

    pub fn color(&self, texture_coord: &TextureCoords) -> Color {
        match self {
            Coloration::Color(c) => *c, 
            Coloration::Texture(texture) => {
                let texture_image = &texture.image;
                let width  = texture_image.width();
                let heigth = texture_image.height();
                let (x,y) = (wrap(texture_coord.x, width), wrap(texture_coord.y,heigth));

                Color::from_rgba(texture_image.get_pixel(x , y))
            }
            
        }
    }
    
}

fn wrap(val: f32, bound: u32) -> u32 {
    let coord = (val * (bound as f32)) as i32;
    let warp_coord = coord.rem_euclid(bound as i32);
    warp_coord as u32
}

