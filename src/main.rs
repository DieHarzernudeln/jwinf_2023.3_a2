use image::{DynamicImage, GenericImageView, Rgba};

fn main() {
    let picture: DynamicImage = read_data();

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

fn read_data() -> DynamicImage {
    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename).unwrap();
    println!("{}", filename);

    let img = image::open(filename.trim()).unwrap();

    img
}

fn analyze_pixel(pixel: Rgba<u8>) -> (char, u32, u32) {
    let ch: char = pixel[0] as char;
    let x: u32 = pixel[1] as u32;
    let y: u32 = pixel[2] as u32;

    (ch, x, y)
}
