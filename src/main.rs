use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    read_file("data/example.txt");
}

fn read_file(name: &str) {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{line:?}");
    }
}

#[test]
fn test_read_file() {
    read_file("data/example.txt");
}
