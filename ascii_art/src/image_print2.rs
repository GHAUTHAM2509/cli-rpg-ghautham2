mod image_processing;
// extern crate term_size;
use std::{thread, time::Duration};
use image_processing::get_image;
use term_size;

pub fn main() {
    // let image_paths = vec!["image copy 3.png", "image copy 2.png"];
    let image_paths2 = vec!["1.png","2.png","3.png","4.png","5.png","6.png","7.png","8.png","9.png"];
    // let image_paths2 = vec!["part_1.png","part_01.png"];
    let delay = Duration::from_millis(0);  // 500 milliseconds
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
    // let (term_width, term_height) = (200, 50);
    let (mut term_width, mut term_height) = (0, 0);

    // Attempt to get the terminal size
    match termsize::get() {
        Some(size) => {
            term_width = size.cols;
            term_height = size.cols*3;
            term_height = term_height/10;
        }
        None => {
            println!("Unable to determine terminal size.");
        }
    }
    // let (term_width, term_height) = (200, 45);
    
    for image_path2 in &image_paths2 {
        get_image(image_path2, term_width, term_height, line_delay);
        thread::sleep(delay);
    }
}
