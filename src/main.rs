use bmp::{self, Image, Pixel, open};
use std::io::{stdout, Write};

type Coordinate = (u32, u32);

struct Dimensions {
    width: u32,
    height: u32,
}

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
fn create(dimensions: Dimensions) -> Image {

    let image = Image::new(dimensions.width, dimensions.height);
    image
}

/**
 * Load from file
 */
fn load(path: &str) -> Image {

    let image = open(path);
    let image = match image {
        Ok(image) => image,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        },
    };

    image

}

/**
 * Obtain user input
 */
fn query_user(questions: Vec<&String>) -> Vec<String> {

    let responses = Vec::new();

    for question in questions.iter() {
        print!("{}", &question[..]);
        stdout().flush().unwrap();

        let mut response = String::new();
        std::io::stdin().read_line(&mut response).unwrap();

        responses.push(response);
    }

    responses
}

fn save(img: Image, path: &str) {

    let status = img.save(path);
    match status {
        Ok(_) => println!("File save successfully"),
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    }

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
fn draw_pixel(mut img: Image, coord: Coordinate, color: Pixel) -> Image {
    img.set_pixel(coord.0, coord.1, color);
    img
}

/**
 * Draw line (vertical, horizontal, diagonal)
 */
fn draw_line(mut img: Image, start: Coordinate, end: Coordinate, color: Pixel) -> Image {
    fn get_y(start: Coordinate, end: Coordinate, x: u32) -> u32 {
        f64::round(
            (end.1 - start.1) as f64
            / (end.0 - start.0) as f64
            * (x - start.0) as f64
            + start.1 as f64
        ) as u32
    }
    for x in start.0..=end.0 {
        img = draw_pixel(img, (x, get_y(start, end, x)), color);
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

    // TODO: make configurable
    let image_dimensions = Dimensions {
        width: 256,
        height: 256,
    };

    let image = create(image_dimensions);

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
