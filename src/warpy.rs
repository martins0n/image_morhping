use super::line_pair::{distatance, LinePair, Vector};

pub fn warpy<'a>(
    line_pairs: &'a Vec<LinePair>,
    source: &'a image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    target: &'a image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    degree: f32,
) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let (imgx, imgy) = source.dimensions();
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
            let a = 50.0;
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

pub fn cross_dissolve<'a>(
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