use bmp::{self, Image, open};
use std::io::{stdout, Write};

type Coordinate = (i32, i32);

struct Dimensions {
    width: u32,
    height: u32,
}

// TODO: Color constants

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
