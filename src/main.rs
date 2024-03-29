use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

mod intcode;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

fn read_file_to_string(file_path: &Path) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();

    let _len = file.read_to_string(&mut contents).unwrap();

    contents
}

fn parse_lines<T, F>(input: &str, parser: F) -> Vec<T> where F: Fn(&str) -> T {
    input.split("\n").map(|s| parser(s)).collect()
}

fn parse_intcode_program(input_str: &str) -> intcode::Program {
    input_str
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn main() {
    let root = env::current_dir().unwrap().join("src");
    println!("Current dir: {:?}", root.to_str());

    day01::run(&read_file_to_string(root.join("day01").join("input.txt").into_boxed_path().as_ref()));
    day02::run(&read_file_to_string(root.join("day02").join("input.txt").into_boxed_path().as_ref()));
    day03::run(&read_file_to_string(root.join("day03").join("input.txt").into_boxed_path().as_ref()));
    day04::run(&read_file_to_string(root.join("day04").join("input.txt").into_boxed_path().as_ref()));
    day05::run(&read_file_to_string(root.join("day05").join("input.txt").into_boxed_path().as_ref()));
    day06::run(&read_file_to_string(root.join("day06").join("input.txt").into_boxed_path().as_ref()));
    day07::run(&read_file_to_string(root.join("day07").join("input.txt").into_boxed_path().as_ref()));
    day08::run(&read_file_to_string(root.join("day08").join("input.txt").into_boxed_path().as_ref()));
    day09::run(&read_file_to_string(root.join("day09").join("input.txt").into_boxed_path().as_ref()));
    day10::run(&read_file_to_string(root.join("day10").join("input.txt").into_boxed_path().as_ref()));
    day11::run(&read_file_to_string(root.join("day11").join("input.txt").into_boxed_path().as_ref()));
    day12::run(&read_file_to_string(root.join("day12").join("input.txt").into_boxed_path().as_ref()));
    day13::run(&read_file_to_string(root.join("day13").join("input.txt").into_boxed_path().as_ref()));
}
