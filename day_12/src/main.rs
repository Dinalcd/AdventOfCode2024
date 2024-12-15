use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
fn main() {
    part1();
}
fn part1() {
    let mut seen: VecDeque<(usize, usize)> = VecDeque::new();
    let contents = parse_input("src/input.txt");
    let rows = contents.len();
    let col = contents[0].len();
    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    for line in contents.clone() {
        for char in line {
            print!("{}", char);
        }
        println!("");
    }

    for r in 0..rows {
        for c in 0..col {
            if seen.contains(&(r, c)) {
                continue;
            }
            seen.push_back((r, c));
            let mut region = HashSet::new();
            region.insert((r, c));
            let mut q: VecDeque<(usize, usize)> = VecDeque::new();
            q.push_back((r, c));
            let char = contents[r][c];
            while !q.is_empty() {
                if let Some((cr, cc)) = q.pop_front() {
                    let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // directions to go left, down, right, up
                    for (dr, dc) in directions.iter() {
                        let nr = cr as isize + *dr; // New row (after direction)
                        let nc = cc as isize + *dc; // New column (after direction)

                        // Check if new row and column are within bounds
                        if nr < 0 || nc < 0 || nr >= rows as isize || nc >= col as isize {
                            continue;
                        }

                        // Convert back to usize for indexing
                        let nr = nr as usize;
                        let nc = nc as usize;

                        // Check if the position is valid
                        if contents[nr][nc] != char {
                            continue;
                        }
                        if seen.contains(&(nr, nc)) {
                            continue;
                        }
                        region.insert((nr, nc));
                        seen.push_back((nr, nc));
                        q.push_back((nr, nc));
                    }
                }
            }
            regions.push(region);
        }
    }

    println!("{:?}", regions);
    let mut price = 0;
    for region in regions {
        price += (perimiter(region.clone()) * &region.len())
    }
    println!("Price: {}", price);
}
fn perimiter(region: HashSet<(usize, usize)>) -> usize {
    let mut sum = 0;
    for &(r, c) in &region {
        sum += 4;
        let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // directions to go left, down, right, up
        for (dr, dc) in directions.iter() {
            let nr = (r as isize + *dr) as usize; // New row (after direction)
            let nc = (c as isize + *dc) as usize; // New column (after direction)
            if region.contains(&(nr, nc)) {
                sum -= 1; // Subtract 1 if the neighbor exists in the region
            }
        }
    }
    sum
}

fn parse_input(file_path: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let grid: Vec<Vec<char>> = contents
        .lines() // Split by newlines
        .map(|line| {
            line.split_whitespace() // Split each line by whitespace
                .flat_map(|word| word.chars()) // Convert each word into its characters
                .collect::<Vec<char>>() // Collect characters into a vector for each row
        })
        .collect(); // Collect all rows into a 2D vector
    grid
}
