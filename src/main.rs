use nalgebra::{DMatrix};
use std::{fs::File, io::{BufReader, BufRead}};

type Tuple = (char, Direction);

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_tuple(tuple: Tuple) -> Option<Direction> {
        let (c, previous) = tuple;
        match c {
            '-' => Some(previous),
            '|' => Some(previous),
            'L' => Some(if previous == Direction::Down {Direction::Right} else {Direction::Up}),
            'J' => Some(if previous == Direction::Down {Direction::Left} else {Direction::Up}),
            '7' => Some(if previous == Direction::Up {Direction::Left} else {Direction::Down}),
            'F' => Some(if previous == Direction::Up {Direction::Right} else {Direction::Down}),
            _ => None,
        }
    }
}

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

fn find_start(matrix: DMatrix<char>) -> Option<(usize, usize)> {
    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            if matrix[(i, j)] == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn solution(filename: &str) -> i32 {
    let matrix = get_matrix(filename);
    let visited = DMatrix::from_element(matrix.nrows(), matrix.ncols(), 0);
    let stack = Vec::<Tuple>::new();
    let start = find_start(matrix).unwrap();
    0
}

fn main() {
    assert_eq!(solution("example.txt"), 8);
    assert_eq!(solution("input.txt"), 0);
}
