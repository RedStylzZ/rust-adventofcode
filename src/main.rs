use std::fs;

fn get_numbers(line: &str) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();

    for c in line.chars() {
        if c >= '0' && c <= '9' {
            v.push(c.to_digit(10).unwrap());
        }
    }

    return v;
}

fn part_one(lines: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    for line in lines {
        let numbers = get_numbers(line);
        let num: u32 = format!("{}{}", numbers[0], numbers[numbers.len() - 1])
            .parse()
            .unwrap();
        sum += num;
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
        if c >= '0' && c <= '9' {
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

        let num: u32 = format!("{}{}", parts[0], parts[parts.len() - 1])
            .parse()
            .unwrap();

        sum += num;
    }

    sum
}

fn main() {
    let file_path = "input/day01.txt";

    println!("Reading {}", file_path);

    let contents = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(e) => panic!("Read file: {}", e),
    };

    let lines = contents.split("\n").collect();

    let sum_one = part_one(&lines);
    let sum_two = part_two(&lines);

    println!("PartOne: {sum_one}");
    println!("PartTwo: {sum_two}");
}
