mod image_processing;
extern crate term_size;
use std::{thread, time::Duration};
use image_processing::get_image;

pub fn main() {
    let image_paths = vec!["image copy 3.png", "image copy 2.png"];
    let image_paths2 = vec!["image copy 5.png"];
    let delay = Duration::from_millis(500);  // 500 milliseconds
    let line_delay = Duration::from_millis(50);  // 50 milliseconds delay between lines
    let mut loop_count = 0;

    loop {
        // Get terminal size
        
        let (term_width, term_height) = (190, 60);
        
        for image_path in &image_paths {
            print!("\x1B[2J\x1B[1;1H"); // Clear the screen
            get_image(image_path, term_width, term_height, line_delay);
            thread::sleep(delay);
        }

        loop_count += 1;
        if loop_count >= 2 {
            break;
        }
    }
    let (term_width, term_height) = (189, 190);
    // let (term_width, term_height) = (170, 50);
    for image_path2 in &image_paths2 {
        // print!("\x1B[2J\x1B[1;1H"); // Clear the screen
        get_image(image_path2, term_width, term_height, line_delay);
        thread::sleep(delay);
    }
}
