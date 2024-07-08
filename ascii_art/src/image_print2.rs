mod image_processing;
extern crate term_size;
use std::{thread, time::Duration};
use image_processing::get_image;

pub fn main() {
    // let image_paths = vec!["image copy 3.png", "image copy 2.png"];
    let image_paths2 = vec!["01.png","02.png","03.png","04.png","05.png","06.png","07.png","08.png","09.png"];
    // let image_paths2 = vec!["part_1.png","part_01.png"];
    let delay = Duration::from_millis(20);  // 500 milliseconds
    let line_delay = Duration::from_millis(20);  // 50 milliseconds delay between lines
    // let mut loop_count = 0;

    // loop {
    //     // Get terminal size
    //     let (term_width, term_height) = (190, 60);
        
    //     for image_path in &image_paths {
    //         print!("\x1B[2J\x1B[1;1H"); // Clear the screen
    //         get_image(image_path, term_width, term_height, line_delay);
    //         thread::sleep(delay);
    //     }

    //     loop_count += 1;
    //     if loop_count >= 2 {
    //         break;
    //     }
    // }
    let (term_width, term_height) = (200, 50);
    // let (term_width, term_height) = (200, 45);
    
    for image_path2 in &image_paths2 {
        get_image(image_path2, term_width, term_height, line_delay);
        thread::sleep(delay);
    }
}
