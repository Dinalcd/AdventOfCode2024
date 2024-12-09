use core::num;
use std::{collections::btree_map::Values, fs};

fn main() {
    part1();
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
                let digits: Vec<_> = count
                    .to_string()
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect();
                println!("{:?}", digits);
                for d in digits {
                    nums.push(d.to_string()); // Push the character into the vector
                }
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

    for mut i in 0..nums.len() {
        if nums[i] == "." {
            if rightpointer >= i {
                if nums[rightpointer] == "." {
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
        sum += nums[i].parse::<i64>().unwrap_or(0) * i as i64;
    }
    print!("{}", sum);
}
