use std::fs;
extern crate queues;
use queues::*;
use std::collections::HashMap;
use std::collections::VecDeque;

use std::collections::HashSet;

fn main() {
    part1();
    part2();
}
fn part2() {
    let char_2d_array = parse_input("src/input.txt"); // Parse the input and create a 2D array from the file
    let mut sum = 0; // Initialize sum variable to accumulate the scores from trailheads
    let mut trailhead: Vec<(usize, usize)> = vec![]; // This will hold the coordinates of all trailheads (starting points with height 0)

    // Loop through each cell in the 2D array to find trailheads
    for i in 0..char_2d_array.len() {
        for j in 0..char_2d_array[0].len() {
            if char_2d_array[i][j] == 0 {
                trailhead.push((i, j)) // when found 0, push to trail head
            }
        }
    }
    // for each trail head calculate the reachable 9s and get the sum
    let mut sum: usize = 0;
    for (r, c) in trailhead {
        sum += score2(char_2d_array.clone(), r, c)
    }

    println!("Sum: {:?}", sum);
}

fn score2(array: Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((i, j)); // initiallise q with the trailhead
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new(); // hashmap which stores locations and how many times we can reach the cell
    seen.insert((i, j), 1);
    let mut trails: usize = 0; // count how many 9s we have reached
    while !q.is_empty() {
        // while the queue is not empty
        if let Some((cr, cc)) = q.pop_front() {
            let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // directions to go left, down, right, up
            for (dr, dc) in directions.iter() {
                let nr = (cr as isize + *dr) as usize; // New row
                let nc = (cc as isize + *dc) as usize; // New column
                                                       // skip if it is out of bounds
                if nr < 0 || nc < 0 || nr >= array.len() || nc >= array[0].len() {
                    continue;
                }
                // if the cell next to it is not incrementing by 1, then skip
                if array[nr][nc] != array[cr][cc] + 1 {
                    continue;
                } // if it is 9 add to trails

                if array[nr][nc] == 9 {
                    trails += 1
                } else {
                    // if it is not 9, push this cell to the queue to explore its surrounding
                    q.push_back((nr, nc));
                }
            }
        }
    }
    return trails; // Return the total number of reachable '9' cells
}

fn part1() {
    let char_2d_array = parse_input("src/input.txt");
    let mut sum = 0;
    let mut visited = vec![vec![false; char_2d_array[0].len()]; char_2d_array.len()];
    let mut trailhead: Vec<(usize, usize)> = vec![];

    for i in 0..char_2d_array.len() {
        for j in 0..char_2d_array[0].len() {
            if char_2d_array[i][j] == 0 {
                trailhead.push((i, j))
            }
        }
    }
    let mut sum: usize = 0;
    for (r, c) in trailhead {
        sum += score(char_2d_array.clone(), r, c)
    }

    println!("Sum: {:?}", sum);
}

fn score(array: Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((i, j));
    let mut seen = HashSet::from([(i, j)]);
    let mut summit: usize = 0;
    while !q.is_empty() {
        if let Some((cr, cc)) = q.pop_front() {
            let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
            for (dr, dc) in directions.iter() {
                let nr = (cr as isize + *dr) as usize; // New row
                let nc = (cc as isize + *dc) as usize; // New column
                if nr < 0 || nc < 0 || nr >= array.len() || nc >= array[0].len() {
                    continue;
                }
                if array[nr][nc] != array[cr][cc] + 1 {
                    continue;
                }
                if seen.contains(&(nr, nc)) {
                    continue;
                }
                seen.insert((nr, nc));

                if array[nr][nc] == 9 {
                    summit += 1
                } else {
                    q.push_back((nr, nc));
                }
            }
        }
    }
    return summit;
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
