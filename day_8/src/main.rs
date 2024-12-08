use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    part1();
    part2();
}
fn parse_input(file_path: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part2() {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut unique_antinodes: HashSet<(i32, i32)> = HashSet::new(); // Store unique antinodes

    // Parse input grid
    let mut char_2d_array = parse_input("src/input.txt");

    // Store positions of antennas in the map
    for (row_idx, row) in char_2d_array.iter().enumerate() {
        for (col_idx, &character) in row.iter().enumerate() {
            if character != '.' {
                antennas
                    .entry(character)
                    .or_insert(vec![])
                    .push((col_idx as i32, row_idx as i32)); // Push tuple (col, row)
            }
        }
    }

    let mut count = 0;

    // Process each set of antennas by their frequency (key)
    for (_, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                // Calculate the differences between antennas
                let diff_x = x2 - x1;
                let diff_y = y2 - y1;

                // Process the antinode from the direction of (x1, y1) towards (x2, y2)
                let mut antinode_x1 = x1;
                let mut antinode_y1 = y1;
                // First, move backwards along the line and check antinodes
                while antinode_x1 >= 0 && antinode_y1 >= 0 {
                    antinode_x1 -= diff_x;
                    antinode_y1 -= diff_y;

                    if is_valid_antinode(&char_2d_array, (antinode_x1, antinode_y1)) {
                        if unique_antinodes.insert((antinode_x1, antinode_y1)) {
                            char_2d_array[antinode_y1 as usize][antinode_x1 as usize] = '#';
                            count += 1;
                        }
                    }
                }

                // Now, move forwards along the line and check antinodes
                let mut antinode_x2 = x2;
                let mut antinode_y2 = y2;
                while (antinode_x2 as usize) < char_2d_array[0].len()
                    && (antinode_y2 as usize) < char_2d_array.len()
                {
                    antinode_x2 += diff_x;
                    antinode_y2 += diff_y;

                    if is_valid_antinode(&char_2d_array, (antinode_x2, antinode_y2)) {
                        if unique_antinodes.insert((antinode_x2, antinode_y2)) {
                            char_2d_array[antinode_y2 as usize][antinode_x2 as usize] = '#';
                            count += 1;
                        }
                    }
                }

                // Also ensure the positions of the antennas themselves are counted as antinodes
                if unique_antinodes.insert((x1, y1)) {
                    char_2d_array[y1 as usize][x1 as usize] = '#';
                    count += 1;
                }
                if unique_antinodes.insert((x2, y2)) {
                    char_2d_array[y2 as usize][x2 as usize] = '#';
                    count += 1;
                }
            }
        }
    }

    // Print the total number of unique antinodes
    println!("Total number of unique antinodes: {}", count);

    //  print the grid with antinodes marked
    // for row in char_2d_array {
    //     for character in row {
    //         print!("{}", character);
    //     }
    //     println!();
    // }
}

fn part1() {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut unique_antinodes: HashSet<(i32, i32)> = HashSet::new();

    let mut char_2d_array = parse_input("src/input.txt");
    // for (row_idx, row) in char_2d_array.iter().enumerate() {
    //     for (col_idx, &character) in row.iter().enumerate() {
    //         print!("{}", char_2d_array[row_idx][col_idx]);
    //     }
    //     println!();
    // }
    for (row_idx, row) in char_2d_array.iter().enumerate() {
        for (col_idx, &character) in row.iter().enumerate() {
            if character != '.' {
                antennas
                    .entry(character)
                    .or_insert(vec![])
                    .push((col_idx as i32, row_idx as i32)); // Push tuple (row, col)
            }
        }
    }
    let mut count = 0;

    for (key, value) in antennas.iter() {
        for i in 0..value.len() {
            for j in i + 1..value.len() {
                let (x1, y1) = value[i];
                let (x2, y2) = value[j];
                let diff_x = x2 - x1;
                let diff_y = y2 - y1;

                let antinode_x1 = x1 - diff_x;
                let antinode_y1 = y1 - diff_y;
                let antinode_x2 = x2 + diff_x;
                let antinode_y2 = y2 + diff_y;

                if is_valid_antinode(&char_2d_array, (antinode_x1, antinode_y1)) {
                    if unique_antinodes.insert((antinode_x1, antinode_y1)) {
                        char_2d_array[antinode_y1 as usize][antinode_x1 as usize] = '#';
                        count += 1;
                    }
                }

                // Check and insert the second antinode
                if is_valid_antinode(&char_2d_array, (antinode_x2, antinode_y2)) {
                    if unique_antinodes.insert((antinode_x2, antinode_y2)) {
                        char_2d_array[antinode_y2 as usize][antinode_x2 as usize] = '#';
                        count += 1;
                    }
                }
            }
        }
    }
    println!("Anti nodes: {:?}", count);
    // for (row_idx, row) in char_2d_array.iter().enumerate() {
    //     for (col_idx, &character) in row.iter().enumerate() {
    //         print!("{}", char_2d_array[row_idx][col_idx]);
    //     }
    //     println!();
    // }

    //println!("{:?}", antennas);
}
fn is_valid_antinode(grid: &Vec<Vec<char>>, coord: (i32, i32)) -> bool {
    let (x, y) = coord;
    if x >= 0 && y >= 0 {
        if let Some(row) = grid.get(y as usize) {
            if let Some(&cell) = row.get(x as usize) {
                return true;
            }
        }
    }
    false
}
