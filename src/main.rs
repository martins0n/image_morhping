#![allow(dead_code)]
extern crate image;

mod line_pair;
use line_pair::{distatance, square, triang, Line, LinePair, Vector};

fn warpy<'a>(
    line_pairs: &'a Vec<LinePair>,
    source: &'a image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    target: &'a image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    degree: f32,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let (imgx, imgy) = target.dimensions();
    let mut warpy_target: image::ImageBuffer<image::Rgb<u8>, _> =
        image::ImageBuffer::new(imgx, imgy);
    let line_pairs_mod: Vec<LinePair> = line_pairs
        .iter()
        .map(|x| x.turn_to_degree(degree))
        .collect();
    for (x, y, pixel) in warpy_target.enumerate_pixels_mut() {
        let mut dsum = Vector { x: 0.0, y: 0.0 };
        let mut weightsum = 0.0;
        let X = Vector {
            x: x as f32,
            y: y as f32,
        };
        for lp in line_pairs_mod.iter() {
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

        if (X_new.x <= (imgx - 1) as f32)
            & (X_new.x >= 0.0)
            & (X_new.y <= (imgy - 1) as f32)
            & (X_new.y >= 0.0)
        {
            let t_pixel = source.get_pixel(X_new.x.floor() as u32, X_new.y.floor() as u32);
            *pixel = t_pixel.clone();
        } else {
            *pixel = image::Rgb([255, 255, 255]);
        }
    }

    warpy_target
}

fn cross_dissolve<'a>(
    source_image: &'a image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    image_after_warm: &'a image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    delta: f32,
) -> Vec<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>> {
    let iterations = (1.0 / delta) as i32;
    let mut cd_images = Vec::<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>>::new();
    let (imgx, imgy) = source_image.dimensions();
    for i in 0..iterations {
        let mut temp_image: image::ImageBuffer<image::Rgb<u8>, _> =
            image::ImageBuffer::new(imgx, imgy);
        for (x, y, pixel) in temp_image.enumerate_pixels_mut() {
            let si_pixel = source_image.get_pixel(x, y);
            let iaw_pixel = image_after_warm.get_pixel(x, y);
            let r = (si_pixel[0] as f32
                + i as f32 * delta * (iaw_pixel[0] as f32 - si_pixel[0] as f32))
                as u8;
            let g = (si_pixel[1] as f32
                + i as f32 * delta * (iaw_pixel[1] as f32 - si_pixel[1] as f32))
                as u8;
            let b = (si_pixel[2] as f32
                + i as f32 * delta * (iaw_pixel[2] as f32 - si_pixel[2] as f32))
                as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        cd_images.push(temp_image);
    }
    cd_images
}

fn test_with_simple_polygons() {
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

    let lp = vec![
        LinePair {
            t_line: Line {
                p: Vector { x: 100.0, y: 100.0 },
                q: Vector { x: 100.0, y: 200.0 },
            },
            s_line: Line {
                p: Vector { x: 100.0, y: 100.0 },
                q: Vector { x: 100.0, y: 200.0 },
            },
        },
        LinePair {
            t_line: Line {
                p: Vector { x: 100.0, y: 200.0 },
                q: Vector { x: 200.0, y: 200.0 },
            },
            s_line: Line {
                p: Vector { x: 100.0, y: 200.0 },
                q: Vector { x: 200.0, y: 250.0 },
            },
        },
    ];
    let inv_lp = lp.iter().map(|x| x.swap_lines()).collect();
    let a = warpy(&lp, &imgbufsource, &imgbufdestination, 0.5);
    let b = warpy(&inv_lp, &imgbufdestination, &imgbufsource, 0.5);
    a.save("./data/target_source.png").unwrap();
    b.save("./data/source_target.png").unwrap();
    let vec_images = cross_dissolve(&imgbufsource, &a, 0.1);
    let mut i = 0;
    for _image in vec_images.iter() {
        _image
            .save(format!("./data/target_source_{}.png", i))
            .unwrap();
        i += 1;
    }
}

fn main() {
    test_with_simple_polygons()
}
