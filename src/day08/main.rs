use std::{collections::HashMap, fs};

fn part_one(contents: &String) -> i64 {
    let (instr, maps) = contents.split_once("\n\n").unwrap();

    let map = maps
        .split("\n")
        .map(|s| {
            let (key, fields) = s.split_once(" = ").unwrap();
            let (left, right) = fields.split_once(", ").unwrap();
            let left = left.replace("(", "");
            let right = right.replace(")", "");

            (key, (left, right))
        })
        .collect::<HashMap<_, _>>();
    let (mut key, mut value) = map.get_key_value("AAA").unwrap();

    let mut sum = 0;
    for i in instr.chars().into_iter().cycle() {
        if *key == "ZZZ" {
            break;
        }

        match i {
            'L' => (key, value) = map.get_key_value(&value.0.as_str()).unwrap(),
            'R' => (key, value) = map.get_key_value(&value.1.as_str()).unwrap(),
            val => panic!("Invalid instruction {val}"),
        }

        sum += 1;
    }

    return sum;
}

fn part_two(lines: &String) -> i64 {
    todo!()
}

fn main() {
    let file_path = "input/day08.txt";

    println!("---------- Day08 ----------");
    println!("Reading {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Read file");

    let sum_one = part_one(&contents);

    println!("PartOne:\t{sum_one}");
}

#[cfg(test)]
mod tests {
    use crate::part_one;

    #[test]
    fn part_one_test() {
        let result = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
            .to_string();

        assert_eq!(part_one(&result), 6);
    }
}
