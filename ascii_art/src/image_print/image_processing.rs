use image::GenericImageView;
use colored::*;
use std::{thread, time::Duration};

pub fn get_str_ascii(intent: u8) -> &'static str {
    let ascii = ["!","*", "@","~","[","/","&", "o", ":", "*", "#", " ", "!"];
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
            let (width, height) = img.dimensions();

            // Calculate scale factors
            let scale_width = (width / term_width as u32) * 2;
            let scale_height = (height / term_height as u32) * 2; // Adjust height to compensate for terminal aspect ratio

            for y in (0..height).step_by(scale_height as usize) {
                for x in (0..width).step_by(scale_width as usize) {
                    let pix = img.get_pixel(x, y);
                    let mut intensity = (pix[0] as u32 / 3 + pix[1] as u32 / 3 + pix[2] as u32 / 3) as u8;
                    if pix[3] == 0 {
                        intensity = (255 + 0 + 0) as u8;
                        print!("{}", get_colored_ascii(intensity, 0, 200, 0));
                    }
                    if pix[3] != 0 {
                        print!("{}", get_colored_ascii(intensity, pix[0], pix[1], pix[2]));
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
