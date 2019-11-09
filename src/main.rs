#![allow(dead_code)]
extern crate image;
extern crate serde_json;

mod line_pair;
mod warpy;

use line_pair::{square, triang, Line, LinePair, Vector};
use std::fs::File;
use std::path::Path;
use warpy::{cross_dissolve, warpy};

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

fn warp_joker() {
    let json_file_path = Path::new("./data/pairs.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let lp: Vec<LinePair> = serde_json::from_reader(json_file).expect("error while reading json");

    let mut imgbufdestination: image::ImageBuffer<image::Rgb<u8>, _> =
        image::open(Path::new("./data/gg/ua881bcd690deuh59y75x.jpg"))
            .unwrap()
            .to_rgb();
    let mut imgbufsource: image::ImageBuffer<image::Rgb<u8>, _> =
        image::open(Path::new("./data/gg/iphone360_9144.jpg"))
            .unwrap()
            .to_rgb();
    let inv_lp = lp.iter().map(|x| x.swap_lines()).collect();
    let a = warpy(&lp, &imgbufsource, &imgbufdestination, 1.0);
    let b = warpy(&inv_lp, &imgbufsource, &imgbufdestination, 1.0);
    a.save("./data/target_source.png").unwrap();
    b.save("./data/source_target.png").unwrap();
    let vec_images = cross_dissolve(&a, &imgbufdestination, 0.1);
    let mut i = 0;
    for _image in vec_images.iter() {
        _image
            .save(format!("./data/target_source_{}.png", i))
            .unwrap();
        i += 1;
    }

    //  b.save("./data/source_target.png").unwrap();
}

fn main() {
    warp_joker()
}
