use bmp::{self, Image};
use std::io::{stdout, Write};

type Coordinate = (i32, i32);

// TODO: Color constants

/**
 * Create bitmap
 */
fn create(width: u32, height: u32) -> Image {
    Image::new(width, height)
}

/**
 * Load from file
 */
fn load(path: &str) -> Image {
    Image::new(69, 420)
}

fn save(img: Image, path: &str) {

}

// fn draw_pixel(path: &str) {
//     let mut image = match bmp::open(path) {
//         Ok(i) => i,
//         Err(_) => bmp::Image::new(100, 100)
//     };
//     image.set_pixel(50, 50, bmp::Pixel::new(255, 255, 255));
//
//     image.save(path).expect("This should save correctly.");
// }

/**
 * Set colour of pixel
 */
fn draw_pixel(img: Image, coord: Coordinate) -> Image {
    img
}

/**
 * Draw line (vertical, horizontal, diagonal)
 */
fn draw_line(mut img: Image, start: Coordinate, end: Coordinate) -> Image {
    fn get_y(start: Coordinate, end: Coordinate, x: i32) -> i32 {
        f64::round(
            (end.1 - start.1) as f64
            / (end.0 - start.0) as f64
            * (x - start.0) as f64
            + start.1 as f64
        ) as i32
    }
    for x in start.0..=end.0 {
        img = draw_pixel(img, (x, get_y(start, end, x)));
    }
    img
}

/**
 * Draw rectangle
 */
fn draw_rect(img: Image, top_left: Coordinate, top_right: Coordinate) -> Image {
    img
}

/**
 * Draw ellipse
 */
fn draw_ellipse(img: Image, centre: Coordinate, r_ver: i32, r_hor: i32, filled: bool) -> Image {
    img
}


fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");

    print!("Which operation? ");
    // We use "flush" so that we see the question before the answer.
    // We can only use `flush` when we use `Write` too -- don't worry why yet!
    stdout().flush().unwrap();
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).unwrap();

    // match op.as_str() {
    //     "pixel\n" => draw_pixel(path.as_str()),
    //     _ =>  {
    //         eprintln!("The operation {op} was not recognised!");
    //     },
    // }
}
