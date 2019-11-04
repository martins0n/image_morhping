extern crate num_complex;

use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

pub fn triang(x: u32, y: u32) -> bool {
    if (x > 200) & (x < 400) & (y > 300) & (y < 600) {
        true
    } else {
        false
    }
}

pub fn square(x: u32, y: u32) -> bool {
    if (x > 100) & (x < 150) & (y > 100) & (y < 130) {
        true
    } else {
        false
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vector {
    pub fn dist2(self, other: Vector) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    pub fn perpendicular(self) -> Vector {
        Vector {
            x: -self.y,
            y: self.x,
        }
    }
}

impl Mul for Vector {
    type Output = f32;
    fn mul(self, other: Vector) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl Vector {
    pub fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
   pub p: Vector,
   pub q: Vector,
}

pub struct LinePair {
    pub t_line: Line,
    pub s_line: Line,
}

impl LinePair {
    pub fn swap_lines(&self) -> Self {
        LinePair {
            t_line: self.s_line,
            s_line: self.t_line,
        }
    }

    pub fn turn_to_degree(&self, degree: f32) -> Self {
        if degree == 1.0 {
            LinePair {
                s_line: self.s_line,
                t_line: self.t_line,
            }
        }else{
            let mut new_t_line = self.s_line;
            new_t_line.p = self.s_line.p + (self.s_line.p - self.t_line.p).mul(degree);
            new_t_line.q = self.s_line.q + (self.s_line.q - self.t_line.q).mul(degree);
            LinePair {
                s_line: self.s_line,
                t_line: new_t_line,
            }

        }
    }
}

pub fn distatance(p1: &Vector, p2: &Vector, X: &Vector) -> f32 {
    let dist = ((p2.y - p1.y) * X.x - (p2.x - p1.x) * X.y + p2.x * p1.y - p2.y * p1.x).abs()
        / ((p2.y - p1.y).powi(2) + (p2.x - p1.x).powi(2)).sqrt();
    dist
}