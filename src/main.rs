use std::env;
use image::{DynamicImage, GenericImageView, Rgba};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut fixed_path = String::new();

    if args.len() >= 2 {
        fixed_path = args[1].clone();
    }

    let picture: DynamicImage = read_data(fixed_path);

    let mut out = String::new();

    let mut x = 0;
    let mut y = 0;

    let mut pix: Rgba<u8> = picture.get_pixel(x, y);

    while !(pix[1] == 0 && pix[2] == 0) {
        pix = picture.get_pixel(x, y);

        let (ch, x_d, y_d) = analyze_pixel(pix);
        out.push(ch);

        x += x_d;
        x %= picture.width();

        y += y_d;
        y %= picture.height();
    }

    println!("{}", out);
}

fn read_data(fixed_path: String) -> DynamicImage {
    let mut input = String::new();

    if fixed_path.is_empty() {
        println!("Enter path to file:");
        std::io::stdin().read_line(&mut input).unwrap();
    } else {
        input = fixed_path;
    }

    let file_path = input.trim();
    println!("{}", file_path);

    let img = image::open(file_path).unwrap();

    img
}

fn analyze_pixel(pixel: Rgba<u8>) -> (char, u32, u32) {
    let ch: char = pixel[0] as char;
    let x: u32 = pixel[1] as u32;
    let y: u32 = pixel[2] as u32;

    (ch, x, y)
}
