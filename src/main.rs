use nalgebra::{DMatrix};
use std::{fs::File, io::{BufReader, BufRead}};

fn get_matrix(filename: &str) -> DMatrix<char> {
    let file = File::open(filename);
    let reader = BufReader::new(file.unwrap());
    let rows: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let num_rows = rows.len();
    let num_cols = rows[0].len();
    DMatrix::from_row_slice(num_rows, num_cols, &rows.iter().flatten().copied().collect::<Vec<char>>())
}


fn solution(filename: &str) -> i32 {
    let matrix = get_matrix(filename);
    0
}

fn main() {
    assert_eq!(solution("example.txt"), 8);
    assert_eq!(solution("input.txt"), 0);
}
