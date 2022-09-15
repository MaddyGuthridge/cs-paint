use bmp::{self, Image, Pixel, open};
use std::{io::{stdout, Write, self}, process::exit};

type Coordinate = (u32, u32);

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

    let mut responses = Vec::new();

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
fn draw_line(mut img: Image, mut start: Coordinate, mut end: Coordinate, color: Pixel) -> Image {
    fn get_y(start: Coordinate, end: Coordinate, x: u32) -> u32 {
        f64::round(
            (end.1 as i32 - start.1 as i32) as f64
            / (end.0 - start.0) as f64
            * (x - start.0) as f64
            + start.1 as f64
        ) as u32
    }
    if start.0 < end.0 {
        (end, start) = (start, end);
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
 * Draw ellipse
 */
fn draw_ellipse(img: Image, centre: Coordinate, r_ver: i32, r_hor: i32, filled: bool) -> Image {
    img
}


fn prompt(prompt_str: String) -> String {
    if prompt_str != "" {
        println!("{prompt_str}");
    }
    print!(">>> ");
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().to_string()
}


fn prom_num() -> u32 {
    prompt(String::new()).parse().unwrap()
}


fn prompt_coord() -> Coordinate {
    println!("X");
    let x = prom_num();
    println!("Y");
    let y = prom_num();
    (x, y)
}


fn prompt_main() -> Image {
    println!("Welcome to CS Paint");
    println!("Main menu");
    println!("===================");
    println!("o => open file");
    println!("n => new file");
    println!("===================");
    print!(">>> ");

    let choice = prompt(String::new());
    match choice.as_str() {
        "o" => {
            let filename = prompt("Enter filename".to_string());
            return load(&filename);
        }
        "n" => {
            let dims = prompt_coord();
            return create(Dimensions { width: dims.0, height: dims.1 });
        }
        &_ => {
            println!("Invalid option :(");
            panic!();
        }
    }
}


fn prompt_edit(mut img: Image) -> Image {
    println!("p => set pixel");
    println!("l => draw line");
    println!("s => save");

    let choice = prompt(String::new());
    match choice.as_str() {
        "p" => {
            let coord = prompt_coord();
            return draw_pixel(img, coord, Pixel::new(255, 255, 255));
        }
        "l" => {
            println!("Start point");
            let start = prompt_coord();
            println!("End point");
            let end = prompt_coord();
            return draw_line(img, start, end, Pixel::new(255, 255, 255));
        }
        "s" => {
            let filename = prompt("Filename".to_string());
            save(img, &filename.to_string());
            exit(0);
        }
        &_ => {
            println!("Invalid option :(");
            panic!();
        }
    }
}


fn main() {
    let mut img = prompt_main();
    loop {
        img = prompt_edit(img);
    }
}
