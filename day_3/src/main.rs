use regex::Regex;
use std::fs; // 1.1.8

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let re = Regex::new(r"mul\([1-9][0-9]*,[1-9][0-9]*\)").unwrap();
    let re2 = Regex::new(r"([1-9][0-9]*)").unwrap();

    let mut sum: i32 = 0;

    for match_ in re.find_iter(&contents) {
        println!("Found match: {}", match_.as_str());
        let matches: Vec<&str> = re2
            .find_iter(match_.as_str())
            .map(|mat| mat.as_str())
            .collect();
        println!("{:?}", matches);
        sum = sum + (matches[0].parse::<i32>().unwrap() * matches[1].parse::<i32>().unwrap());
    }

    println!("Sum: {}", sum);
}
