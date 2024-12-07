use std::collections::HashMap;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    // Read the file contents
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    // Split the contents into two parts by two or more consecutive newlines
    let parts: Vec<&str> = contents.split("\r\n\r\n").collect();
    //println!("{:?}", contents);
    // Ensure that there are exactly two parts
    let pipe: Vec<&str> = parts[0].split("\r\n").collect();
    let list: Vec<&str> = parts[1].split("\r\n").collect();

    //println!("Part 1:");
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    for rule in pipe {
        let values: Vec<String> = rule.split("|").map(|s| s.to_string()).collect();
        rules
            .entry(values[0].clone())
            .or_insert(vec![])
            .push(values[1].clone())
    }

    // for (key, value) in &rules {
    //     println!("{}: {:?}", key, value);
    // }
    let mut sum: i32 = 0;

    //println!("Part 2:");
    for number in list {
        let num: Vec<String> = number.split(",").map(|s| s.to_string()).collect();
        let mut valid = true;
        for i in 0..num.len() {
            // println!("{:?}", num[i]);
            let val: Vec<String> = rules.get(&num[i]).cloned().unwrap_or_else(Vec::new);
            for j in i + 1..num.len() {
                if !val.contains(&num[j]) {
                    valid = false;
                }
            }
        }

        let mid = num.len() / 2;

        if valid {
            sum += num[mid].parse::<i32>().unwrap();
        }
        if !valid {}
        //println!();
    }
    println!("Part 1: {}", sum)
}

fn part2() {
    // Read the file contents
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    // Split the contents into two parts by two or more consecutive newlines
    let parts: Vec<&str> = contents.split("\r\n\r\n").collect();
    //println!("{:?}", contents);
    // Ensure that there are exactly two parts
    let pipe: Vec<&str> = parts[0].split("\r\n").collect();
    let list: Vec<&str> = parts[1].split("\r\n").collect();

    //println!("Part 1:");
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    for rule in pipe {
        let values: Vec<String> = rule.split("|").map(|s| s.to_string()).collect();
        rules
            .entry(values[0].clone())
            .or_insert(vec![])
            .push(values[1].clone())
    }

    for (key, value) in &rules {
        println!("{:?}: {:?}", key, value);
    }
    let mut sum: i32 = 0;

    //println!("Part 2:");

    for number in list {
        let num: Vec<String> = number.split(",").map(|s| s.to_string()).collect();
        let mut valid = true;
        for mut i in 0..num.len() {
            let val: Vec<String> = rules.get(&num[i]).cloned().unwrap_or_else(Vec::new);
            for j in i + 1..num.len() {
                if !val.contains(&num[j]) {
                    valid = false;
                }
            }
        }

        let mid = num.len() / 2;

        if !valid {
            let sortednum: Vec<String> = order(num, rules.clone());
            println!("{:?}", sortednum);
            sum += sortednum[mid].parse::<i32>().unwrap();
        }
    }
    println!("Part 2: {}", sum)
}
fn order(num: Vec<String>, rules: HashMap<String, Vec<String>>) -> Vec<String> {
    let mut sorted_input = num.clone();

    sorted_input.sort_by(|a, b| {
        if rules.get(a).map_or(false, |values| values.contains(b)) {
            std::cmp::Ordering::Greater
        } else if rules.get(b).map_or(false, |values| values.contains(a)) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    sorted_input
}
