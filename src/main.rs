use bmp::{self, Image};
use std::io::{stdout, Write};

type Coordinate = (i32, i32);

// TODO: Color constants
const YELLOW: (u32, u32, u32) = (255,255,0);
const RED: (u32,u32,u32) = (255,0,0);
const BLUE: (u32,u32,u32) = (0,0,255);
const WHITE: (u32,u32,u32) = (255,255,255);
const GREEN: (u32,u32,u32) = (0,255,0);
const ORANGE: (u32,u32,u32) = (255,165,0);
const PURPLE: (u32,u32,u32) = (255,0,255);
const IND_RED: (u32, u32,u32) = (204,0,0);
const FIN_BLUE: (u32,u32,u32) = (0,47,108);
const ICE_BLUE: (u32,u32,u32) = (2,82,156);
const ICE_RED: (u32,u32,u32) = (220,30,53);
/**
 * Create bitmap
 */
fn create(width: i32, height: i32) -> Image {

}

/**
 * Load from file
 */
fn load(path: &str) -> Image {

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
 * Draw circle
 */
fn draw_circle(mut img: Image, centre: Coordinate, r: i32) -> Image {

    centre_x = centre.0;
    centre_y = centre.1;
    diameter = r * 2;
    for x in 0..=diameter {
        for y in 0..=diameter {
            if (centre_x - x).pow(2) + (centre_y - y).pow(2) <= r.pow(2) {
                img = draw_pixel(img, (x, y), (255, 255, 0));
            }
        }
    }

    return img;
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
