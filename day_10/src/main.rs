use std::fs;
fn main() {
    println!("Hello, world!");
    let char_2d_array = parse_input("src/input.txt");
    let mut sum = 0;
    for i in 0..char_2d_array.len() {
        for j in 0..char_2d_array[i].len() {
            print!("{}", char_2d_array[i][j]);

            // Check if the character is '0' (as a character, not integer 0)
            if char_2d_array[i][j] == 0 {
                sum += nextnumber(char_2d_array, char_2d_array[i][j], i, j);
            }
        }
        println!(); // New line after each row
    }

    println!("Sum: {}", sum);
}

fn nextnumber(array: Vec<Vec<usize>>, value: usize, i: usize, j: usize) -> usize {
    if !(i >= 0 && i < array.len()) && (j > 0 && j < array[0].len()) {
        return 0;
    } else if array[i][j] == 9 {
        return 1;
    }

    nextnumber(array.clone(), array[i + 1][j], i, j);
    nextnumber(array.clone(), array[i - 1][j], i, j);
    nextnumber(array.clone(), array[i][j + 1], i, j);
    nextnumber(array.clone(), array[i][j - 1], i, j);
    return 0;
}

fn parse_input(file_path: &str) -> Vec<Vec<usize>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
        .lines() // Split the file content into lines
        .map(|line| {
            line.chars() // Iterate over each character in the line
                .filter_map(|c| c.to_digit(10)) // Convert each char to a digit (u32)
                .map(|d| d as usize) // Convert from u32 to usize
                .collect::<Vec<usize>>() // Collect the digits into a vector
        })
        .collect() // Collect the vectors of usize into a Vec<Vec<usize>>
}
