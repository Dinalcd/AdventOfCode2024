use core::num;
use std::{collections::btree_map::Values, fs};

fn main() {
    //part1();
    part2();
}
fn part2() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut nums: Vec<String> = Vec::new();

    let mut count = 0;
    let mut space = true;

    for number in contents.chars() {
        let counter = number.to_digit(10).unwrap();
        for _ in 0..counter {
            if space {
                nums.push(count.to_string()); // Push the character into the vector
            } else {
                nums.push(".".to_string());
            }
        }
        if space {
            space = false;
            count += 1;
        } else {
            space = true;
        }
        // print!("{} ,", counter);
    }
    println!();

    for val in &nums {
        print!("{}", val);
    }
    println!("{:?}", nums);

    println!();
    for val in &nums {
        print!("{}", val);
    }
    println!();
    let mut sum: i64 = 0;

    for i in 0..nums.len() {
        sum += nums[i].parse::<i64>().unwrap_or(0) * i as i64;
    }

    let mut length = 0;
    let mut startempty = 0;
    let mut endempty;

    let mut locations: Vec<(String, usize, usize, usize)> = Vec::new();

    for i in 0..nums.len() {
        // Check if we are at the last element or the current group ends
        if i + 1 < nums.len() && nums[i] == nums[i + 1] {
            // Increment length for consecutive matches
            length += 1;
        } else {
            // Calculate start and end of the current group
            startempty = i - length;
            endempty = i;
            length += 1; // Include the current element in the count

            // Print and store the group information
            // println!(
            //     "start: {}, end: {}, character: {}, count: {}",
            //     startempty, endempty, nums[i], length
            // );
            locations.push((nums[i].clone(), startempty, endempty, length));

            // Reset length for the next group
            length = 0;
        }
    }

    for i in (0..locations.len()).rev() {
        let (char2, start2, end2, length2) = &locations[i]; // Immutable reference for outer loop

        if char2 == "." {
            continue;
        }

        for j in 0..locations.len() {
            let (char1, start1, end1, length1) = &locations[j]; // Immutable reference for inner loop

            if char1 != "." {
                continue;
            }

            if *length2 <= *length1 && *start2 >= *start1 + *length1 {
                // println!(
                //     "char: {}, start: {}, end1: {}, length: {}",
                //     char1, start1, end1, length1
                // );
                // println!(
                //     "char: {}, start: {}, end1: {}, length: {}",
                //     char2, start2, end2, length2
                // );

                for k in (*start1 as usize)..((*start1 + *length2) as usize) {
                    nums[k] = char2.to_string();
                }
                for l in (*start2 as usize)..((*start2 + *length2) as usize) {
                    nums[l] = ".".to_string();
                }
                let new_location_i = (".".to_string(), *start2, *end2, *length2);

                // Mutable borrow here is safe since we're not iterating directly
                locations[j] = (
                    ".".to_string(),
                    *start1 + *length2,
                    *end1,
                    *length1 - *length2,
                );
                locations[i] = new_location_i;

                // for j in &locations {
                //     println!("{:?}", j)
                // }
                // for num in &nums {
                //     print!("{}", num)
                // }
                // println!();

                break; // Stop after processing this location
            }
        }
    }
    let mut sum: i64 = 0;

    for i in 0..nums.len() {
        // println!("{}", nums[i]);
        sum += nums[i].parse::<i64>().unwrap_or(0) * i as i64;
    }
    println!("{}", sum);
}
fn part1() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let mut nums: Vec<String> = Vec::new();

    let mut count = 0;
    let mut space = true;

    for number in contents.chars() {
        let counter = number.to_digit(10).unwrap();
        for _ in 0..counter {
            if space {
                nums.push(count.to_string()); // Push the character into the vector
            } else {
                nums.push(".".to_string());
            }
        }
        if space {
            space = false;
            count += 1;
        } else {
            space = true;
        }
        // print!("{} ,", counter);
    }
    println!();

    for val in &nums {
        print!("{}", val);
    }
    println!("{:?}", nums);

    let mut rightpointer = nums.len() - 1;

    for i in 0..nums.len() {
        if nums[i] == "." {
            if rightpointer >= i {
                while nums[rightpointer] == "." {
                    rightpointer -= 1
                }
                // Swap the element at `i` with the element at `rightpointer`
                let temp = nums[rightpointer].clone(); // Clone the value to avoid moving it
                nums[rightpointer] = ".".to_string(); // Put a dot at rightpointer
                nums[i] = temp; // Put the swapped value at i

                // Move the right pointer
                rightpointer -= 1;
            }
        }
    }
    println!();
    for val in &nums {
        print!("{}", val);
    }
    println!();
    let mut sum: i64 = 0;

    for i in 0..nums.len() {
        println!("{}", nums[i]);
        sum += nums[i].parse::<i64>().unwrap_or(0) * i as i64;
    }
    print!("{}", sum);
}
