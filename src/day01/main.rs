use once_cell::sync::Lazy;
use regex::Regex;
use std::fs;

fn get_numbers(line: &str) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();

    for c in line.chars() {
        if c.is_ascii_digit() {
            v.push(c.to_digit(10).unwrap());
        }
    }

    return v;
}

fn part_one(lines: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;

    for line in lines {
        let numbers = get_numbers(line);

        sum += numbers.first().unwrap() * 10 + numbers.last().unwrap();
    }

    sum
}

fn str_to_int(s: &str) -> u32 {
    return match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    };
}

fn get_numbers_part_two(line: &str) -> Vec<u32> {
    let chars = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut v: Vec<u32> = Vec::new();

    for (i, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            v.push(c.to_digit(10).unwrap());
            continue;
        }

        for num in chars {
            if i + num.len() > line.len() {
                continue;
            }

            let s = &line[i..i + num.len()];

            if s == num {
                let x = str_to_int(s);
                v.push(x);
                break;
            }
        }
    }

    return v;
}

fn part_two(lines: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;

    for line in lines {
        let parts = get_numbers_part_two(line);

        sum += parts.first().unwrap() * 10 + parts.last().unwrap();
    }

    sum
}

fn get_sum_regex_solution02(input: String) -> u32 {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap());

    static RE2: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap());

    input
        .lines()
        .map(|x| {
            // print!("{}:           ", x);
            let mut iter = RE.captures_iter(&x).map(|x| match &x[0] {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                a => a.parse().unwrap(),
            });
            let x2 = iter.next().unwrap();
            // if no further value there, use the first

            let x = x.chars().rev().collect::<String>();

            let mut iter2 = RE2.captures_iter(x.as_str()).map(|x| match &x[0] {
                "eno" => 1,
                "owt" => 2,
                "eerht" => 3,
                "ruof" => 4,
                "evif" => 5,
                "xis" => 6,
                "neves" => 7,
                "thgie" => 8,
                "enin" => 9,
                a => a.parse().unwrap(),
            });

            let y = iter2.next().unwrap();

            let sol = x2 * 10 + y;

            println!("{}:\tSol {}", x, sol);
            sol
        })
        .sum()
}

fn main() {
    let file_path = "input/day01.txt";

    println!("Reading {}", file_path);

    let contents = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(e) => panic!("Read file: {}", e),
    };

    let lines = contents.lines().collect();

    let sum_one = part_one(&lines);
    let sum_two = part_two(&lines);
    let ruben = get_sum_regex_solution02(contents);

    println!("PartOne:\t{sum_one}");
    println!("PartTwo:\t{sum_two}");
    println!("Ruben:\t\t{ruben}");
}
