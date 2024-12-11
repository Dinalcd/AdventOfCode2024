use std::collections::HashMap;
use std::fs;

fn main() {
    //part1();
    part2();
}

fn part2() {
    // Read the input from the file and initialize the `stones` map
    let mut line = parse_input("src/input.txt");
    let mut stones: HashMap<i64, i64> = HashMap::new();

    // Print the initial input to check its contents (for debugging)
    println!("{:?}", line);

    // Step 1: Populate the `stones` map with counts of each number
    for stone in line {
        // `try_into().unwrap()` is used to convert `stone` to an `i64` (this assumes `stone` can be converted)
        // `or_insert(0)` ensures that if the number is not yet in the map, it gets initialized to 0
        // `+= 1` increments the count of `stone` in the map
        *stones.entry(stone.try_into().unwrap()).or_insert(0) += 1;
    }

    // Step 2: Iterate for 75 iterations to transform the stones
    for _ in 0..75 {
        let mut new_stones: HashMap<i64, i64> = HashMap::new(); // New map to store the updated stones after each iteration

        // Step 3: Iterate over each `stone` and its count in the `stones` map
        for (&stone, &count) in &stones {
            // If the stone is 0, replace it with 1
            if stone == 0 {
                // `or_insert(0)` ensures that if `1` is not in `new_stones`, it's initialized to 0
                *new_stones.entry(1).or_insert(0) += count;
            } else {
                // Step 4: Calculate the length of the stone's string representation
                let length = stone.to_string().len();

                // If the length of the number is even, split the number into two parts
                if length % 2 == 0 {
                    let num_str = stone.to_string();
                    let half = length / 2;
                    let (left_str, right_str) = num_str.split_at(half); // Split the string into two parts

                    // Try to parse the two parts back into integers
                    if let (Ok(left), Ok(right)) =
                        (left_str.parse::<i64>(), right_str.parse::<i64>())
                    {
                        // If parsing is successful, increment the count for both parts in the new map
                        *new_stones.entry(left).or_insert(0) += count;
                        *new_stones.entry(right).or_insert(0) += count;
                    }
                } else {
                    // If the stone's length is odd, multiply it by 2024
                    *new_stones.entry(stone * 2024).or_insert(0) += count;
                }
            }
        }

        // Step 5: Update `stones` with the new values for the next iteration
        stones = new_stones;
    }

    // Step 6: Calculate the final output by summing all the counts in the `stones` map
    let output: i64 = stones.values().sum();

    // Print the final result: total number of stones after 75 iterations
    println!("Number of stones:{}", output);
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
