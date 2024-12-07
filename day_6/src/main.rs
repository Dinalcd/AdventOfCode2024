use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    part2();
}

fn part2() {
    let mut char_2d_array = parse_input("src/input.txt");
    let mut x = 0;
    let mut y = 0;
    let mut initial_x = 0;
    let mut initial_y = 0;

    let dirs: HashMap<char, (i32, i32)> =
        HashMap::from([('^', (0, -1)), ('>', (1, 0)), ('v', (0, 1)), ('<', (-1, 0))]);

    // Find the initial position of the guard ('^')
    for (row_index, row) in char_2d_array.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|&c| c == '^') {
            initial_x = row_index;
            initial_y = col_index;
        }
    }
    // Find the initial position of the guard

    let mut count = 0;
    for i in 0..char_2d_array.len() {
        for mut j in 0..char_2d_array.len() {
            let mut visited: HashSet<(usize, usize, char)> = HashSet::new();
            // Reparse input every loop iteration
            char_2d_array = parse_input("src/input.txt");
            //println!("i:{}, j:{}", i, j);

            x = initial_x;
            y = initial_y;

            // Change '.' to 'O'
            if char_2d_array[i][j] == '.' {
                char_2d_array[i][j] = 'O';
            } else if char_2d_array[i][j] == '#' {
                if j < char_2d_array.len() - 2 {
                    j += 2;
                }
            }
            let direction_changes: HashMap<char, char> =
                HashMap::from([('^', '>'), ('>', 'v'), ('v', '<'), ('<', '^')]);

            // print_grid(&char_2d_array);
            // println!();
            let mut cross_path = false;
            let mut change_direction = false;

            while x >= 0 && y >= 0 && x < char_2d_array.len() && y < char_2d_array[0].len() {
                let direction = char_2d_array[x][y];
                // print_grid(&char_2d_array);
                if !visited.insert((x, y, direction)) {
                    // print_grid(&char_2d_array);
                    // println!();
                    println!(
                        "Guard is stuck in a loop at ({}, {}) with direction {}",
                        x, y, direction
                    );
                    count += 1;
                    break; // Loop detected
                } else {
                    visited.insert((x, y, direction));
                }

                if let Some((movex, movey)) = dirs.get(&direction) {
                    let new_x = x as i32 + movey;
                    let new_y = y as i32 + movex;

                    // Check if the new position is out of bounds
                    if new_x < 0
                        || new_y < 0
                        || new_x >= char_2d_array.len() as i32
                        || new_y >= char_2d_array[0].len() as i32
                    {
                        break;
                    }

                    // Handle moving to an empty space ('.')
                    if char_2d_array[new_x as usize][new_y as usize] == '.' {
                        char_2d_array[new_x as usize][new_y as usize] = direction;
                        if cross_path || change_direction {
                            char_2d_array[x][y] = '+';
                            cross_path = false;
                            change_direction = false;
                        }
                        // Mark current path based on direction
                        else if direction == '^' || direction == 'v' {
                            char_2d_array[x][y] = '|';
                        } else if direction == '<' || direction == '>' {
                            char_2d_array[x][y] = '-';
                        }

                        // Update the guard's position
                        x = new_x as usize;
                        y = new_y as usize;
                    } else if (char_2d_array[new_x as usize][new_y as usize] == '|'
                        && (direction == '^' || direction == 'v'))
                        || (char_2d_array[new_x as usize][new_y as usize] == '+'
                            && (direction == '^' || direction == 'v'))
                    {
                        char_2d_array[new_x as usize][new_y as usize] = direction;
                        char_2d_array[x][y] = '|';
                        x = new_x as usize;
                        y = new_y as usize;
                    } else if (char_2d_array[new_x as usize][new_y as usize] == '-'
                        && (direction == '<' || direction == '>'))
                        || (char_2d_array[new_x as usize][new_y as usize] == '+'
                            && (direction == '<' || direction == '>'))
                    {
                        char_2d_array[new_x as usize][new_y as usize] = direction;
                        char_2d_array[x][y] = '-';
                        x = new_x as usize;
                        y = new_y as usize;
                    }
                    // Handle moving across an intersection ('+' or crossing paths)
                    else if (char_2d_array[new_x as usize][new_y as usize] == '|'
                        && (direction == '<' || direction == '>'))
                    {
                        char_2d_array[new_x as usize][new_y as usize] = direction;
                        char_2d_array[x][y] = '-';

                        cross_path = true;

                        // Update the guard's position
                        x = new_x as usize;
                        y = new_y as usize;
                    } else if (char_2d_array[new_x as usize][new_y as usize] == '-'
                        && (direction == '^' || direction == 'v'))
                    {
                        char_2d_array[new_x as usize][new_y as usize] = direction;
                        char_2d_array[x][y] = '|';

                        cross_path = true;

                        // Update the guard's position
                        x = new_x as usize;
                        y = new_y as usize;
                    }

                    //println!("New position: ({}, {})", new_x, new_y);

                    // Handle changes based on obstacles
                    if char_2d_array[new_x as usize][new_y as usize] == '#'
                        || char_2d_array[new_x as usize][new_y as usize] == 'O'
                    {
                        change_direction = true;
                        // Change direction based on obstacles
                        if change_direction {
                            char_2d_array[x][y] =
                                *direction_changes.get(&char_2d_array[x][y]).unwrap();
                        }
                    }

                    // Print the updated grid after each move
                    // print_grid(&char_2d_array);
                    // println!();
                } else {
                    println!("Direction not found");
                }
            }
            // print_grid(&char_2d_array);
            // println!();

            // Reset the grid position after finishing the loop
            char_2d_array[i][j] = '.';
        }
    }

    // Print the number of times the guard got stuck in a loop
    println!("{}", count);
}

fn part1() {
    let mut char_2d_array = parse_input("src/input.txt");

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
        let direction = char_2d_array[x][y];

        if let Some((movex, movey)) = dirs.get(&direction) {
            // Now x is the first value and y is the second
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

    println!("{}", count);
    // print grid
    print_grid(&char_2d_array);
}

fn parse_input(file_path: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn print_grid(char_2d_array: &Vec<Vec<char>>) {
    for line in char_2d_array {
        for character in line {
            print!("{} ", character);
        }
        println!();
    }
}
