use std::ops::{Add, Sub, Neg, Mul};


#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64
}

impl Point  {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn coordinate(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn distance(&self, other: Point) -> f64 {
        let vector: Vector3 = *self - other;
        vector.length()
    }

}

impl Sub<Point> for Point {
    type Output = Vector3;

    fn sub(self, other: Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Into<Vector3> for Point {
    fn into(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}


impl Add<Vector3> for Point {
    type Output = Point;

    fn add(self, vector: Vector3) -> Self::Output {
        let (vec_x, vec_y, vec_z) = vector.coordinate();
        Point::new(self.x + vec_x, self.y + vec_y, self.z + vec_z )
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;

    fn sub(self, vector: Vector3) -> Self::Output {
        let (vec_x, vec_y, vec_z) = vector.coordinate();
        Point::new(self.x - vec_x, self.y - vec_y, self.z - vec_z )
    }
}



impl Add<Point> for Vector3 {
    type Output = Point;

    fn add(self, point: Point) -> Self::Output {
        let (p_x, p_y, p_z) = point.coordinate();
        Point::new(self.x + p_x, self.y + p_y, self.z + p_z )
    }
}


impl Sub<Point> for Vector3 {
    type Output = Point;

    fn sub(self, point: Point) -> Self::Output {
        let (p_x, p_y, p_z) = point.coordinate();
        Point::new(self.x - p_x, self.y - p_y, self.z - p_z )
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}


impl Vector3  {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn coordinate(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        let (x1, y1, z1) = other.coordinate();
        self.x * x1 + self.y * y1 + self.z * z1  
    }

    pub fn cross(&self, other: &Vector3) -> Self {
        let (x1, y1, z1) = other.coordinate();
        Self {
            x: self.y * z1 - self.z * y1, 
            y: self.z * x1 - self.x * z1,
            z: self.x * y1 - self.y * x1 

        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let inverse_norm = self.length().recip();
        Self {
            x: self.x * inverse_norm,
            y: self.y * inverse_norm,
            z: self.z * inverse_norm,
        }
    }


}


impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        let (x1, y1, z1) = other.coordinate();
        Vector3::new(self.x + x1, self.y + y1, self.z + z1)
    } 
    
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self::Output {
        let (x1, y1, z1) = other.coordinate();
        Vector3::new(self.x - x1, self.y - y1, self.z - z1)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, factor: f64) -> Self::Output {
        Vector3::new(self.x * factor, self.y * factor, self.z * factor)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Self::Output {
        let (x, y, z) = other.coordinate();
        Vector3::new(x * self, y * self, z * self)
    }
}



impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}