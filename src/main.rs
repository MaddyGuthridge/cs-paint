use bmp::{self, Image, open};
use std::io::{stdout, Write};

type Coordinate = (i32, i32);

// TODO: Color constants

/**
 * Create bitmap
 */
fn create(width: i32, height: i32) -> Image {

}

/**
 * Load from file
 */
fn load(path: &str) -> Image {

    let image = open(path);
    match image {
        Ok(image) => image,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    }

    image

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

}

/**
 * Draw line (vertical, horizontal, diagonal)
 */
fn draw_line(img: Image, start: Coordinate, end: Coordinate) -> Image {

}

/**
 * Draw rectangle
 */
fn draw_rect(img: Image, top_left: Coordinate, top_right: Coordinate) -> Image {

}

/**
 * Draw ellipse
 */
fn draw_ellipse(img: Image, centre: Coordinate, r_ver: i32, r_hor: i32, filled: bool) -> Image {

}


fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");

    print!("Which operation? ");
    // We use "flush" so that we see the question before the answer.
    // We can only use `flush` when we use `Write` too -- don't worry why yet!
    stdout().flush().unwrap();
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).unwrap();

    match op.as_str() {
        "pixel\n" => draw_pixel(path.as_str()),
        _ =>  {
            eprintln!("The operation {op} was not recognised!");
        },
    }

}
