use std::fs;

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let list: std::str::Split<'_, &str> = contents.split("\n");
    let mut count = 0;
    for line in list {
        println!("{}", line);
        let mut last_direction: Option<i32> = None;
        let mut safe;

        let split_line: Vec<&str> = line.split_whitespace().collect(); // Collect into a Vec
        let num1 = split_line[1]
            .parse::<i32>()
            .expect("Invalid number in input");
        let num2 = split_line[0]
            .parse::<i32>()
            .expect("Invalid number in input");

        let result = num1 - num2;
        if result <= 3 && result >= 1 {
            // checks directionbs
            last_direction = Some(1);
        } else if result >= -3 && result <= -1 {
            last_direction = Some(-1);
        }

        safe = is_safe_report(&split_line, last_direction);
        if !safe {
            for i in 0..split_line.len() {
                let mut modified_line = split_line.clone();
                modified_line.remove(i);
                if is_safe_report(&modified_line, last_direction) {
                    safe = true;
                    break;
                }
            }
        }

        if safe {
            count = count + 1;
        }
        println!("{} ", safe)
    }

    println!("Total safe reports:{}", count)
}

fn is_safe_report(split_line: &Vec<&str>, mut last_direction: Option<i32>) -> bool {
    let mut safe = true;
    let num1 = split_line[1]
        .parse::<i32>()
        .expect("Invalid number in input");
    let num2 = split_line[0]
        .parse::<i32>()
        .expect("Invalid number in input");

    let result: i32 = num1 - num2;
    if result <= 3 && result >= 1 {
        // checks directionbs
        last_direction = Some(1);
    } else if result >= -3 && result <= -1 {
        last_direction = Some(-1);
    } else {
        safe = false;
    }

    for i in 2..split_line.len() {
        let num1 = split_line[i]
            .parse::<i32>()
            .expect("Invalid number in input");
        let num2 = split_line[i - 1]
            .parse::<i32>()
            .expect("Invalid number in input");

        let result = num1 - num2;

        safe = check_safe(result, last_direction, safe);
    }
    return safe;
}

fn check_safe(result: i32, last_direction: Option<i32>, mut safe: bool) -> bool {
    if result <= 3 && result >= 1 {
        if last_direction != Some(1) {
            safe = false;
        }
    } else if result >= -3 && result <= -1 {
        if last_direction != Some(-1) {
            safe = false;
        }
    } else {
        safe = false;
    }

    return safe;
}
