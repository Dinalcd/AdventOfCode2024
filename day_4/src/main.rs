use std::fs;

fn main() {
    part1();
    part2();
    // longpart1();
    // longpart2();
}
fn part1() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let char_2d_array: Vec<Vec<char>> = contents
        .lines() // Split the string into lines
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect();

    // Print the resulting 2D array

    let mut count = 0;

    let directions = vec![
        // east: (0, 1), (0, 2), (0, 3)
        (vec![0, 0, 0], vec![1, 2, 3]),
        // west: (0, -1), (0, -2), (0, -3)
        (vec![0, 0, 0], vec![-1, -2, -3]),
        // // north: (-1, 0), (-2, 0), (-3, 0)
        (vec![-1, -2, -3], vec![0, 0, 0]),
        // south: (1, 0), (2, 0), (3, 0)
        (vec![1, 2, 3], vec![0, 0, 0]),
        // // southeast: (1, 1), (2, 2), (3, 3)
        (vec![1, 2, 3], vec![1, 2, 3]),
        // // southwest: (1, -1), (2, -2), (3, -3)
        (vec![1, 2, 3], vec![-1, -2, -3]),
        // // northwest: (-1, -1), (-2, -2), (-3, -3)
        (vec![-1, -2, -3], vec![-1, -2, -3]),
        // // northeast: (-1, 1), (-2, 2), (-3, 3)
        (vec![-1, -2, -3], vec![1, 2, 3]),
    ];
    let letters = vec!['M', 'A', 'S'];
    for i in 0..char_2d_array.len() {
        for j in 0..char_2d_array.len() {
            if char_2d_array[i][j] == 'X' {
                for (p_i, p_j) in &directions {
                    let mut flag: bool = true;
                    for idx in 0..=2 {
                        let new_i = (i as i32 + p_i[idx]) as usize;
                        let new_j = (j as i32 + p_j[idx]) as usize;

                        if new_j < char_2d_array.len() && new_i < char_2d_array.len() {
                            if char_2d_array[new_i][new_j] != letters[idx] {
                                flag = false;
                                break;
                            }
                        } else {
                            flag = false;
                        }
                    }

                    if flag == true {
                        // debugging
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count)
}

fn longpart1() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let char_2d_array: Vec<Vec<char>> = contents
        .lines() // Split the string into lines
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect();

    // Print the resulting 2D array

    let mut count = 0;

    for i in 0..char_2d_array.len() {
        for j in 0..char_2d_array.len() {
            //east
            print!("{:?}", char_2d_array[i][j]);

            if j + 3 < char_2d_array.len() {
                if char_2d_array[i][j] == 'X' {
                    if char_2d_array[i][j + 1] == 'M'
                        && char_2d_array[i][j + 2] == 'A'
                        && char_2d_array[i][j + 3] == 'S'
                    {
                        count += 1;
                    }
                }
            }
            // west
            if j >= 3 {
                if char_2d_array[i][j] == 'X' {
                    if char_2d_array[i][j - 1] == 'M'
                        && char_2d_array[i][j - 2] == 'A'
                        && char_2d_array[i][j - 3] == 'S'
                    {
                        count += 1;
                    }
                }
            }
            // north
            if i >= 3 {
                if char_2d_array[i][j] == 'X' {
                    if char_2d_array[i - 1][j] == 'M'
                        && char_2d_array[i - 2][j] == 'A'
                        && char_2d_array[i - 3][j] == 'S'
                    {
                        count += 1;
                    }
                }
            }
            //south
            if i + 3 < char_2d_array.len() {
                if char_2d_array[i][j] == 'X' {
                    if char_2d_array[i + 1][j] == 'M'
                        && char_2d_array[i + 2][j] == 'A'
                        && char_2d_array[i + 3][j] == 'S'
                    {
                        count += 1;
                    }
                }
            }
            //SE
            if i + 3 < char_2d_array.len() && j + 3 < char_2d_array.len() {
                if char_2d_array[i][j] == 'X' {
                    if char_2d_array[i + 1][j + 1] == 'M'
                        && char_2d_array[i + 2][j + 2] == 'A'
                        && char_2d_array[i + 3][j + 3] == 'S'
                    {
                        count += 1;
                    }
                }
            }
            //SW
            if i + 3 < char_2d_array.len() && j >= 3 {
                if char_2d_array[i][j] == 'X' {
                    if char_2d_array[i + 1][j - 1] == 'M'
                        && char_2d_array[i + 2][j - 2] == 'A'
                        && char_2d_array[i + 3][j - 3] == 'S'
                    {
                        count += 1;
                    }
                }
            }
            //NW
            if i >= 3 && j >= 3 {
                if char_2d_array[i][j] == 'X' {
                    if char_2d_array[i - 1][j - 1] == 'M'
                        && char_2d_array[i - 2][j - 2] == 'A'
                        && char_2d_array[i - 3][j - 3] == 'S'
                    {
                        count += 1;
                    }
                }
            }
            //NE
            if i >= 3 && j + 3 < char_2d_array.len() {
                if char_2d_array[i][j] == 'X' {
                    if char_2d_array[i - 1][j + 1] == 'M'
                        && char_2d_array[i - 2][j + 2] == 'A'
                        && char_2d_array[i - 3][j + 3] == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }
        println!("")
    }
    print!("Xmas found = {}", count);
}

fn part2() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let char_2d_array: Vec<Vec<char>> = contents
        .lines() // Split the string into lines
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect();

    // Print the resulting 2D array

    let mut count = 0;
    let directions: [(Vec<i32>, Vec<i32>, Vec<char>); 4] = [
        // m on the left and s on the right
        (
            vec![-1, 1, -1, 1],
            vec![-1, -1, 1, 1],
            vec!['M', 'M', 'S', 'S'],
        ),
        // m on the right and s on the left
        (
            vec![-1, 1, -1, 1],
            vec![-1, -1, 1, 1],
            vec!['S', 'S', 'M', 'M'],
        ),
        // M on top and S on the bottom
        (
            vec![-1, 1, -1, 1],
            vec![-1, -1, 1, 1],
            vec!['M', 'S', 'M', 'S'],
        ),
        // M on bottom and S on top
        (
            vec![-1, 1, -1, 1],
            vec![-1, -1, 1, 1],
            vec!['S', 'M', 'S', 'M'],
        ),
    ];

    for i in 0..char_2d_array.len() {
        for j in 0..char_2d_array.len() {
            //print!("{:?}", char_2d_array[i][j]);
            // m on the left and s on the right
            if i > 0 && j > 0 && i + 1 < char_2d_array.len() && j + 1 < char_2d_array.len() {
                if char_2d_array[i][j] == 'A' {
                    for (p_i, p_j, chars) in &directions {
                        let mut flag: bool = true;
                        for idx in 0..p_i.len() {
                            let new_i = (i as i32 + p_i[idx]) as usize;
                            let new_j = (j as i32 + p_j[idx]) as usize;
                            if char_2d_array[new_i][new_j] != chars[idx] {
                                // debugging
                                // println!(
                                //     "i: {:?}, j: {:?}, character: {:?}",
                                //     p_i[idx], p_j[idx], chars[idx]
                                // );
                                flag = false;
                            }
                        }
                        if flag == true {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", count)
}

fn longpart2() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let char_2d_array: Vec<Vec<char>> = contents
        .lines() // Split the string into lines
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect();

    // Print the resulting 2D array

    let mut count = 0;
    for i in 0..char_2d_array.len() {
        for j in 0..char_2d_array.len() {
            if i > 0 && j > 0 && i + 1 < char_2d_array.len() && j + 1 < char_2d_array.len() {
                if char_2d_array[i][j] == 'A' {
                    if char_2d_array[i - 1][j - 1] == 'M'
                        && char_2d_array[i + 1][j - 1] == 'M'
                        && char_2d_array[i - 1][j + 1] == 'S'
                        && char_2d_array[i + 1][j + 1] == 'S'
                    {
                        count += 1;
                    }
                }
                // m on the right and s on the left
                if char_2d_array[i][j] == 'A' {
                    if char_2d_array[i - 1][j - 1] == 'S'
                        && char_2d_array[i + 1][j - 1] == 'S'
                        && char_2d_array[i - 1][j + 1] == 'M'
                        && char_2d_array[i + 1][j + 1] == 'M'
                    {
                        count += 1;
                    }
                }
                // M on top and S on the bottom
                if char_2d_array[i][j] == 'A' {
                    if char_2d_array[i - 1][j - 1] == 'M'
                        && char_2d_array[i + 1][j - 1] == 'S'
                        && char_2d_array[i - 1][j + 1] == 'M'
                        && char_2d_array[i + 1][j + 1] == 'S'
                    {
                        count += 1;
                    }
                }
                // M on botton and S on the top
                if char_2d_array[i][j] == 'A' {
                    if char_2d_array[i - 1][j - 1] == 'S'
                        && char_2d_array[i + 1][j - 1] == 'M'
                        && char_2d_array[i - 1][j + 1] == 'S'
                        && char_2d_array[i + 1][j + 1] == 'M'
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count)
}
