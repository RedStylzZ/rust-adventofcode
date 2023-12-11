use std::fs;

fn numbers(line: &str) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    let mut buf: String = String::new();
    for c in line.chars() {
        if c >= '0' && c <= '9' {
            buf.push(c);
            continue;
        }

        if buf.len() > 0 {
            let i = buf.parse().unwrap();
            v.push(i);
            buf.clear()
        }
    }

    if buf.len() > 0 {
        let i = buf.parse().unwrap();
        v.push(i);
    }

    return v
}

fn main() {

    let file_path = "input/day06_test.txt";

    println!("Reading {}", file_path);

    let contents = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(e) => panic!("Read file: {}", e),
    };

    for line in contents.split("\n") {
        let parts = numbers(line);
        println!("{}", line);
        println!("{:?}", parts);
    }
}
