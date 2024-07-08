use image::{GenericImageView, imageops::FilterType};
use colored::*;
use std::{thread, time::Duration};

pub fn get_str_ascii(intent: u8) -> &'static str {
    let ascii = ["0", "!", "*", "@", "~", "[", "/", "&", "o", ":", "*", "#", "-", " "];
    let index = (intent as usize * (ascii.len() - 1)) / 255;
    ascii[index]
}

pub fn get_colored_ascii(intent: u8, r: u8, g: u8, b: u8) -> ColoredString {
    let ascii_char = get_str_ascii(intent);
    ascii_char.truecolor(r, g, b)
}

pub fn get_image(dir: &str, term_width: u16, term_height: u16, line_delay: Duration) {
    match image::open(dir) {
        Ok(img) => {
            // Resize the image to fit the terminal size
            let resized_img = img.resize_exact(term_width.into(), term_height.into(), FilterType::Nearest);

            let (width, height) = resized_img.dimensions();
            let scale_width = (width as f32 / term_width as f32).ceil() as u32;
            let scale_height = (height as f32 / term_height as f32).ceil() as u32;
            let mut g = 0;
            let mut b = 255;
            let mut r = 0;
            let mut count = 0;

            for y in (0..height).step_by(scale_height as usize) {
                for x in (0..width).step_by(scale_width as usize) {
                    let pix = resized_img.get_pixel(x, y);
                    let mut intensity = (pix[0] as u32 / 3 + pix[1] as u32 / 3 + pix[2] as u32 / 3) as u8;
                    if pix[3] == 0 {
                        intensity = (255 + 0 + 0) as u8;
                        print!("{}", get_colored_ascii(intensity, r, g, b));
                    } else {
                        print!("{}", get_colored_ascii(intensity, pix[0], pix[1], pix[2]));
                    }
                }
                if g < 250 && b > 0 && count == 0{
                    g += 5;
                    b -= 5;  
                }
                if g > 230{
                    count = 1;
                }
                if count == 1 {
                    if g > 3{
                        g -= 3;
                        r += 3;
                    }
                    if 0 < g && g < 10 && r > 5
                    {
                        r -=4;
                        b +=4;
                    }
                }

                println!("");
                thread::sleep(line_delay);
            }
        },
        Err(e) => {
            println!("Failed to open image: {}", e);
        }
    }
}
