use std::fs;

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut all_numbers: Vec<Vec<i64>> = parse_input("src/input.txt"); // gets input into vectors
    let mut result = 0; // stores result

    for equation in all_numbers {
        // for each line
        let sliced = &equation[1..]; // remove first number

        if check_result2(equation[0], sliced, 0) {
            // if it returns true, add to the result
            result += equation[0];
        }
    }
    println!("{}", result); // print result
}
fn part1() {
    let mut all_numbers: Vec<Vec<i64>> = parse_input("src/input.txt"); // gets input into vectors
    let mut result = 0; // stores result

    for equation in all_numbers {
        // for each line
        let sliced = &equation[1..]; // remove first number

        if check_result1(equation[0], sliced, 0) {
            // if it returns true, add to the result
            result += equation[0];
        }
    }
    println!("{}", result); // print result
}
fn parse_input(file_path: &str) -> Vec<Vec<i64>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file"); // read file
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect(); // split each line

    let mut all_numbers: Vec<Vec<i64>> = Vec::new();

    for equation in lines {
        // for each line split by the semi colon
        let parts: Vec<&str> = equation.split(": ").collect();

        if parts.len() > 1 {
            let first_number: i64 = parts[0].parse().unwrap_or(0); // gets first number

            let numbers: Vec<i64> = parts[1] // splits the 1st index in the vector into individual integers
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();

            let mut full_line = vec![first_number];
            full_line.extend(numbers); // turns into one vector

            all_numbers.push(full_line); // pushes current vector into another vec which stores all the lines
        }
    }
    all_numbers // returns the vector
}

fn check_result1(result: i64, numbers: &[i64], current_value: i64) -> bool {
    if numbers.is_empty() {
        // checks if it empty
        return current_value == result; // returns a bool value if the current value is the result
    }
    if current_value > result {
        return false;
    }
    let current_number = numbers[0]; // gets the first number

    if check_result1(result, &numbers[1..], current_value + current_number) {
        // tries to add the numbers
        return true;
    }

    if check_result1(result, &numbers[1..], current_value * current_number) {
        // tries to multiple the numbers
        return true;
    }

    false
}

fn check_result2(result: i64, numbers: &[i64], current_value: i64) -> bool {
    if numbers.is_empty() {
        // checks if it empty
        return current_value == result; // returns a bool value if the current value is the result
    }

    let current_number = numbers[0]; // gets the first number

    if check_result2(result, &numbers[1..], current_value + current_number) {
        // tries to add the numbers
        return true;
    }

    if check_result2(result, &numbers[1..], current_value * current_number) {
        // tries to multiple the numbers
        return true;
    }

    let concat = format!("{}{}", current_value, current_number); // tries to concatenate the numbers togeter
    if let Ok(concat_value) = concat.parse::<i64>() {
        // checks if the value is a integer
        if check_result2(result, &numbers[1..], concat_value) {
            return true;
        }
    }
    false
}
