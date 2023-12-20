use std::fs;

fn get_numbers(line: &str, is_part_two: bool) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();

    let mut buf: String = String::new();
    for c in line.chars() {
        if c.is_ascii_digit() {
            buf.push(c);
            continue;
        }

        if buf.len() > 0 && !is_part_two {
            let i: i64 = buf.parse().unwrap();
            numbers.push(i);
            buf.clear();
        }
    }

    if buf.len() > 0 {
        let i: i64 = buf.parse().unwrap();
        numbers.push(i);
    }

    return numbers;
}

fn get_fastest(time: i64, distance: i64) -> Vec<i64> {
    let mut better: Vec<i64> = Vec::new();

    for i in 0..time + 1 {
        let d = i * (time - i);
        if d > distance {
            better.push(i);
        }
    }

    return better;
}

struct Stat {
    time: i64,
    distance: i64,
}

fn part_one(lines: &Vec<&str>) -> i64 {
    let mut numbers: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        numbers.push(get_numbers(line, false));
    }

    let mut stats: Vec<Stat> = Vec::new();
    for i in 0..numbers[0].len() {
        let stat: Stat = Stat {
            time: numbers[0][i],
            distance: numbers[1][i],
        };
        stats.push(stat);
    }

    let mut sum: i64 = 1;

    for stat in stats {
        let better = get_fastest(stat.time, stat.distance);

        sum *= better.len() as i64;
    }


    return sum;
}

fn part_two(lines: &Vec<&str>) -> i64 {
    let mut numbers: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        numbers.push(get_numbers(line, true));
    }

    let stat: Stat = Stat {
        time: numbers[0][0],
        distance: numbers[1][0],
    };

    let better = get_fastest(stat.time, stat.distance);

    return better.len() as i64;
}

fn main() {
    let file_path = "input/day06.txt";

    println!("Reading {}", file_path);

    let contents = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(e) => panic!("Read file: {}", e),
    };

    let lines = contents.lines().collect();

    let sum_one = part_one(&lines);
    let sum_two = part_two(&lines);

    println!("PartOne:\t{sum_one}");
    println!("PartTwo:\t{sum_two}");
}
