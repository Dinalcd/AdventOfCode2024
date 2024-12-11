use std::collections::HashMap;
use std::fs;

fn main() {
    //part1();
    part2();
}

fn part2() {
    let mut line = parse_input("src/input.txt");
    let mut stones: HashMap<i64, i64> = HashMap::new();

    println!("{:?}", line);

    for stone in line {
        *stones.entry(stone.try_into().unwrap()).or_insert(0) += 1;
    }
    for _ in 0..75 {
        let mut new_stones: HashMap<i64, i64> = HashMap::new();

        for (&stone, &count) in &stones {
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else {
                let length = stone.to_string().len();
                if length % 2 == 0 {
                    // Split the number into two parts (if even length)
                    let num_str = stone.to_string();
                    let half = length / 2;
                    let (left_str, right_str) = num_str.split_at(half);

                    if let (Ok(left), Ok(right)) =
                        (left_str.parse::<i64>(), right_str.parse::<i64>())
                    {
                        *new_stones.entry(left).or_insert(0) += count;
                        *new_stones.entry(right).or_insert(0) += count;
                    }
                } else {
                    // Multiply the number by 2024
                    *new_stones.entry(stone * 2024).or_insert(0) += count;
                }
            }
        }

        // Update stones map for the next iteration
        stones = new_stones;
    }
    let output: i64 = stones.values().sum();

    println!("Number of stones:{}", output)
}

fn part1() {
    let mut line = parse_input("src/input.txt");

    println!("{:?}", line);
    for _ in 0..25 {
        let mut i = 0;

        while true {
            //println!("index: {}, number: {}", i, line[i]);
            let length = line[i].to_string().len();
            if line[i] == 0 {
                line[i] = 1;
                i += 1;
            } else if (length % 2) == 0 {
                let num_str = line[i].to_string();
                let mid = length / 2;
                let (left, right) = num_str.split_at(mid);

                // Parse the left and right parts into usize
                let left_num = left.parse::<usize>().unwrap_or(0);
                let right_num = right.parse::<usize>().unwrap_or(0);
                // println!("left: {}", left_num);
                // println!("right: {}", right_num);
                // Replace the current element and insert the new one
                line[i] = left_num;
                line.insert(i + 1, right_num);
                i += 2;
            } else {
                let new = &(line[i] * 2024);
                line[i] = *new;
                i += 1;
            }
            //println!("{:?}", line);
            if i == line.len() {
                break;
            }
        }
    }
    println!("Number of stones:{}", line.len())
}

fn parse_input(file_path: &str) -> Vec<usize> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
        .split_whitespace()
        .map(|num| num.parse::<usize>().expect("Failed to parse number"))
        .collect()
}
