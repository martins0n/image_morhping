#![allow(dead_code)]
extern crate image;
extern crate num_complex;

use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

fn triang(x: u32, y: u32) -> bool {
    if (x > 200) & (x < 600) & (y > 300) & (y < 600) {
        true
    } else {
        false
    }
}

fn square(x: u32, y: u32) -> bool {
    if (x > 200) & (x < 300) & (y > 100) & (y < 400) {
        true
    } else {
        false
    }
}

#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f32,
    y: f32,
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
    fn dist2(self, other: Vector) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    fn perpendicular(self) -> Vector {
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
    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

struct Line {
    p: Vector,
    q: Vector,
}

struct LinePair {
    t_line: Line,
    s_line: Line,
}

fn distatance(p1: &Vector, p2: &Vector, X: &Vector) -> f32 {
    let dist = ((p2.y - p1.y) * X.x - (p2.x - p1.x) * X.y + p2.x * p1.y - p2.y * p1.x).abs()
        / ((p2.y - p1.y).powi(2) + (p2.x - p1.x).powi(2)).sqrt();
    dist
}

fn warpy<'a>(
    line_pairs: &'a mut Vec<LinePair>,
    source: &'a mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    target: &'a mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> (
    &'a mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    &'a mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) {
    for (x, y, pixel) in target.enumerate_pixels_mut() {
        let mut dsum = Vector { x: 0.0, y: 0.0 };
        let mut weightsum = 0.0;
        let X = Vector {
            x: x as f32,
            y: y as f32,
        };
        for lp in line_pairs.iter() {
            let u =
                (X - lp.t_line.p) * (lp.t_line.q - lp.t_line.p) / lp.t_line.q.dist2(lp.t_line.p);
            let v = ((X - lp.t_line.p) * (lp.t_line.q - lp.t_line.p).perpendicular())
                / lp.t_line.q.dist2(lp.t_line.p).sqrt();
            let X_new = lp.s_line.p
                + (lp.s_line.q - lp.s_line.p).mul(u)
                + (lp.s_line.q - lp.s_line.p)
                    .perpendicular()
                    .mul(v / lp.s_line.q.dist2(lp.s_line.p).sqrt());
            let D = X_new - X;
            let a = 0.001;
            let p = 0;
            let b = 2;
            let lenght = ((lp.t_line.q - lp.t_line.p) * (lp.t_line.q - lp.t_line.p)).sqrt();
            let dist = distatance(&lp.t_line.p, &lp.t_line.q, &X);
            let weight = (lenght.powi(p) / (a + dist)).powi(b);
            dsum = dsum + D.mul(weight);
            weightsum += weight;
        }
        let X_new = X + dsum.mul(1.0 / weightsum);
        println!("{:?}", &X_new);
        // TODO: fix sizes
        if (X_new.x <= 799.0) & (X_new.x >= 0.0) & (X_new.y <= 799.0) & (X_new.y >= 0.0) {
            let t_pixel = source.get_pixel(X_new.x.floor() as u32, X_new.y.floor() as u32);
            *pixel = t_pixel.clone();
        } else {
            // *pixel = image::Rgb([0, 0, 0]);
        }
    }

    (source, target)
}

fn main() {
    let imgx = 800;
    let imgy = 800;

    let mut imgbufdestination: image::ImageBuffer<image::Rgb<u8>, _> =
        image::ImageBuffer::new(imgx, imgy);

    let mut imgbufsource: image::ImageBuffer<image::Rgb<u8>, _> =
        image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbufdestination.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        if triang(x, y) {
            *pixel = image::Rgb([r, 0, b]);
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    }

    for (x, y, pixel) in imgbufsource.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        if square(x, y) {
            *pixel = image::Rgb([r, 0, b]);
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    }

    imgbufsource.save("./data/source.png").unwrap();
    imgbufdestination.save("./data/destination.png").unwrap();

    let mut lp = vec![
        LinePair {
            t_line: Line {
                p: Vector { x: 400.0, y: 500.0 },
                q: Vector { x: 400.0, y: 520.0 },
            },
            s_line: Line {
                p: Vector { x: 200.0, y: 500.0 },
                q: Vector { x: 200.0, y: 550.0 },
            },
        },
        LinePair {
            t_line: Line {
                p: Vector { x: 400.0, y: 520.0 },
                q: Vector { x: 500.0, y: 530.0 },
            },
            s_line: Line {
                p: Vector { x: 200.0, y: 500.0 },
                q: Vector { x: 200.0, y: 550.0 },
            },
        },
    ];

    let (a, b) = warpy(&mut lp, &mut imgbufsource, &mut imgbufdestination);
    a.save("./data/source_source.png").unwrap();
    b.save("./data/target_source.png").unwrap();
}
