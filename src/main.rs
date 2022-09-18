use bmp::{self, Image, Pixel, open};
use std::{io::{Write, self}, process::exit};

// type Coordinate = (u32, u32);
#[derive(Copy, Clone)]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone)]
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

fn save(img: Image, path: &str) -> Image {
    let status = img.save(path);
    match status {
        Ok(_) => println!("File save successfully"),
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    }
    img
}

/**
 * Set colour of pixel
 */
fn draw_pixel(mut img: Image, coord: Coordinate, colour: Pixel) -> Image {
    img.set_pixel(coord.x, coord.y, colour);
    img
}

/**
 * Draw line (vertical, horizontal, diagonal)
 */
fn draw_line(mut img: Image, mut start: Coordinate, mut end: Coordinate, color: Pixel) -> Image {
    fn get_y(start: Coordinate, end: Coordinate, x: u32) -> u32 {
        f64::round(
            (end.y as i32 - start.y as i32) as f64
            / (end.x - start.x) as f64
            * (x - start.x) as f64
            + start.y as f64
        ) as u32
    }
    if start.x > end.x {
        (end, start) = (start, end);
    }
    for x in start.x..=end.x {
        img = draw_pixel(img, Coordinate { x, y: get_y(start, end, x) }, color);
    }
    img
}

/**
 * Draw rectangle
 */
fn draw_rect(mut img: Image, mut top_left: Coordinate, mut top_right: Coordinate, color: Pixel) -> Image {
    // Make it always go from top-to-bottom
    // draw_line will handle the left-to-right for us
    if top_left.y > top_right.y {
        (top_right, top_left) = (top_left, top_right);
    }
    for y in top_left.y..=top_right.y {
        img = draw_line(img, Coordinate { x: top_left.x, y }, Coordinate { x: top_right.x, y: y }, color);
    }
    img
}

/**
 * Draw ellipse
 */
fn draw_ellipse(img: Image, centre: Coordinate, r_ver: i32, r_hor: i32, filled: bool) -> Image {
    img
}

fn print_colour(colour: &Pixel) {
    println!("Colour == {}, {}, {}", colour.r, colour.g, colour.b);
}


fn prompt(prompt_str: String) -> String {
    if prompt_str != "" {
        println!("{prompt_str}");
    }
    print!(">>> ");
    io::stdout().flush().unwrap();
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
    Coordinate { x, y }
}

fn prompt_colour() -> Pixel {
    println!("Red");
    let r = prom_num() as u8;
    println!("Green");
    let g = prom_num() as u8;
    println!("Blue");
    let b = prom_num() as u8;
    Pixel { r, g, b }
}


fn prompt_main() -> Image {
    println!("Welcome to CS Paint");
    println!("Main menu");
    println!("===================");
    println!("o => open file");
    println!("n => new file");
    println!("x => exit");
    println!("===================");

    let choice = prompt(String::new());
    match choice.as_str() {
        "o" => {
            let filename = prompt("Enter filename".to_string());
            return load(&filename);
        }
        "n" => {
            let dims = prompt_coord();
            return create(Dimensions { width: dims.x, height: dims.y });
        }
        "x" => {
            exit(0);
        }
        &_ => {
            println!("Invalid option, try again");
            // Recurse until the give a valid one
            prompt_main()
        }
    }
}


fn prompt_edit(mut img: Image, mut colour: Pixel) -> (Image, Pixel) {
    print_colour(&colour);
    println!("========================");
    println!("c => choose colour");
    println!("p => set pixel");
    println!("l => draw line");
    println!("r => draw rectangle");
    println!("s => save");
    println!("x => exit without saving");

    let choice = prompt(String::new());
    match choice.as_str() {
        "c" => {
            colour = prompt_colour();
        }
        "p" => {
            let coord = prompt_coord();
            img = draw_pixel(img, coord, colour);
        }
        "l" => {
            println!("Start point");
            let start = prompt_coord();
            println!("End point");
            let end = prompt_coord();
            img = draw_line(img, start, end, colour);
        }
        "r" => {
            println!("Start point");
            let start = prompt_coord();
            println!("End point");
            let end = prompt_coord();
            img = draw_rect(img, start, end, colour);
        }
        "s" => {
            let filename = prompt("Filename".to_string());
            img = save(img, &filename.to_string());
        }
        "x" => {
            exit(0);
        }
        &_ => {
            println!("Invalid option :(");
        }
    }
    (img, colour)
}


fn main() {
    let mut colour = Pixel::new(255, 255, 255);
    let mut img = prompt_main();
    loop {
        (img, colour) = prompt_edit(img, colour);
    }
}
