use image::{Rgb, RgbImage};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    part1()
}

fn part1() {
    print!("\x1B[2J\x1B[H"); // Clear the screen and move the cursor to the top-left

    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut char_2d_array: Vec<Vec<char>> = contents
        .lines() // Split the string into lines
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect();
    let mut x = 0;
    let mut y = 0;

    let dirs: HashMap<char, (i32, i32)> =
        HashMap::from([('^', (0, -1)), ('>', (1, 0)), ('v', (0, 1)), ('<', (-1, 0))]);

    let mut x = 0;
    let mut y = 0;
    for (row_index, row) in char_2d_array.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|&c| c == '^') {
            x = row_index;
            y = col_index;
        }
    }
    // println!("{:?}", dirs);

    // println!("{:?}", x);
    // println!("{:?}", y);
    let mut count = 0;
    while x >= 0 && y >= 0 && x < char_2d_array.len() && y < char_2d_array[0].len() {
        let cell_size = 5; // Set cell size to 20x20 pixels

        let img = render_frame(&char_2d_array, cell_size);
        save_frame(img, count);
        let mut direction = char_2d_array[x][y];

        if let Some((movex, movey)) = dirs.get(&direction) {
            // Now `x` is the first value and `y` is the second
            //println!("The first number is {}", movex); // Access the first number
            //println!("The second number is {}", movey); // Access the first number
            let new_x = x as i32 + movey;
            let new_y = y as i32 + movex;
            // println!("{}", new_x);
            // println!("{}", new_y);
            if new_x < 0
                || new_y < 0
                || new_x >= char_2d_array.len() as i32
                || new_y >= char_2d_array[0].len() as i32
            {
                count += 1;
                break;
            }

            if char_2d_array[new_x as usize][new_y as usize] == '.' {
                char_2d_array[new_x as usize][new_y as usize] = direction;
                char_2d_array[x][y] = 'X';
                x = new_x as usize;
                y = new_y as usize;
                count += 1;
            } else if char_2d_array[new_x as usize][new_y as usize] == 'X' {
                char_2d_array[new_x as usize][new_y as usize] = direction;
                char_2d_array[x][y] = 'X';
                x = new_x as usize;
                y = new_y as usize;
            }
            if char_2d_array[new_x as usize][new_y as usize] == '#' {
                if char_2d_array[x][y] == '^' {
                    char_2d_array[x][y] = '>';
                } else if char_2d_array[x][y] == '>' {
                    char_2d_array[x][y] = 'v';
                } else if char_2d_array[x][y] == 'v' {
                    char_2d_array[x][y] = '<';
                } else if char_2d_array[x][y] == '<' {
                    char_2d_array[x][y] = '^';
                }
            }
        } else {
            println!("Direction not found");
        }
    }
    create_video_from_frames();

    println!("{}", count);
    // print grid
    for line in char_2d_array {
        for character in line {
            print!("{} ", character)
        }
        println!();
    }
}

fn render_frame(grid: &Vec<Vec<char>>, cell_size: u32) -> RgbImage {
    let width = grid[0].len() as u32 * cell_size;
    let height = grid.len() as u32 * cell_size;

    // We use a Mutex to allow safe concurrent access to the image buffer
    let img = Arc::new(Mutex::new(RgbImage::new(width, height)));

    // Parallelize the grid processing
    grid.par_iter().enumerate().for_each(|(y, row)| {
        row.par_iter().enumerate().for_each(|(x, &cell)| {
            let color = match cell {
                'X' => Rgb([255, 0, 0]),     // Red for 'X'
                '.' => Rgb([0, 0, 0]),       // black for empty space
                '#' => Rgb([255, 255, 255]), // White for empty space
                '^' => Rgb([0, 0, 255]),     // White for empty space
                '<' => Rgb([0, 0, 255]),     // White for empty space
                '>' => Rgb([0, 0, 255]),     // White for empty space
                'v' => Rgb([0, 0, 255]),     // White for empty space

                _ => Rgb([0, 0, 0]), // Black for other cells
            };

            // Calculate the pixel coordinates in the image
            let px_x = (x as u32 * cell_size);
            let px_y = (y as u32 * cell_size);

            // Lock the image for writing
            let mut img_lock = img.lock().unwrap();

            // Fill the pixels in the corresponding area (cell_size x cell_size block)
            for dy in 0..cell_size {
                for dx in 0..cell_size {
                    let px_x_final = px_x + dx;
                    let px_y_final = px_y + dy;
                    img_lock.put_pixel(px_x_final, px_y_final, color);
                }
            }
        });
    });

    // Unlock and return the final image
    Arc::try_unwrap(img).unwrap().into_inner().unwrap()
}
fn save_frame(img: RgbImage, frame_number: usize) {
    let filename = format!("frame_{:04}.png", frame_number);
    img.save(filename).unwrap();
}
fn create_video_from_frames() {
    // Run ffmpeg command to create video from frames
    let status = Command::new("ffmpeg")
        .arg("-framerate")
        .arg("240") // Set frame rate (30 frames per second)
        .arg("-i")
        .arg("frame_%04d.png") // Input files (e.g., frame_0001.png, frame_0002.png, etc.)
        .arg("-c:v")
        .arg("libx264") // Set the video codec
        .arg("-pix_fmt")
        .arg("yuv420p") // Set pixel format (required for compatibility)
        .arg("output_video.mp4") // Output file name
        .status()
        .expect("Failed to execute ffmpeg");

    if status.success() {
        println!("Video creation successful!");
    } else {
        println!("Video creation failed.");
    }
}
