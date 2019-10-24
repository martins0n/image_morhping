extern crate image;
extern crate num_complex;

fn triang(x: u32, y: u32) -> bool {
    if (x > 200) & (x < 600) & (y > 300) & (y < 600) {
        true
    } else{
        false
    }
}


fn square(x: u32, y: u32) -> bool {
    if (x > 300) & (x < 600) & (y > 300) & (y < 600) {
        true
    } else{
        false
    }
}

fn main() {
    let imgx = 800;
    let imgy = 800;

    let mut imgbufdestination: image::ImageBuffer<image::Rgb<u8>, _> = image::ImageBuffer::new(imgx, imgy);

    let mut imgbufsource: image::ImageBuffer<image::Rgb<u8>, _> = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbufdestination.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        if triang(x, y){
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


    
  
    imgbufsource.save("source.png").unwrap();
    imgbufdestination.save("destination.png").unwrap();
}

