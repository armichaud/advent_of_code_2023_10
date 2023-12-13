use nalgebra::{DMatrix};
use std::{fs::File, io::{BufReader, BufRead}};

type Tuple = (char, Direction);

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const START: char = 'S';
const HORIZONTAL_PIPE: char = '-';
const VERTICAL_PIPE: char = '|';
const L_PIPE: char = 'L';
const J_PIPE: char = 'J';
const SEVEN_PIPE: char = '7';
const F_PIPE: char = 'F';

const TOP_CONNECTED_PIPES: [char; 3] = [VERTICAL_PIPE, SEVEN_PIPE, F_PIPE];
const BOTTOM_CONNECTED_PIPES: [char; 3] = [VERTICAL_PIPE, L_PIPE, J_PIPE];
const RIGHT_CONNECTED_PIPES: [char; 3] = [HORIZONTAL_PIPE, J_PIPE, SEVEN_PIPE];
const LEFT_CONNECTED_PIPES: [char; 3] = [HORIZONTAL_PIPE, L_PIPE, F_PIPE];

impl Direction {
    fn from_tuple(tuple: Tuple) -> Option<Direction> {
        let (c, previous) = tuple;
        match c {
            HORIZONTAL_PIPE => Some(previous),
            VERTICAL_PIPE => Some(previous),
            L_PIPE => Some(if previous == Direction::Down {Direction::Right} else {Direction::Up}),
            J_PIPE => Some(if previous == Direction::Down {Direction::Left} else {Direction::Up}),
            SEVEN_PIPE => Some(if previous == Direction::Up {Direction::Left} else {Direction::Down}),
            F_PIPE => Some(if previous == Direction::Up {Direction::Right} else {Direction::Down}),
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

fn find_start(matrix: &DMatrix<char>) -> Option<(usize, usize)> {
    for i in 0..matrix.nrows() {
        for j in 0..matrix.ncols() {
            if matrix[(i, j)] == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn get_paths(matrix: &DMatrix<char>, start: (usize, usize)) -> Vec<Tuple> {
    let mut paths: Vec<Tuple> = Vec::new();
    let (start_row, start_col) = start;

    if start_row > 0 {
        let up = matrix[(start_row - 1, start_col)];
        if TOP_CONNECTED_PIPES.contains(&up) {
            paths.push((up, Direction::Up));
        }
    }
    if start_col > 0 {
        let left = matrix[(start_col - 1, start_row)];
        if LEFT_CONNECTED_PIPES.contains(&left) {
            paths.push((left, Direction::Left));
        }
    }
    if start_row < matrix.nrows() - 1 {
        let down = matrix[(start_row + 1, start_col)];
        if BOTTOM_CONNECTED_PIPES.contains(&down) {
            paths.push((down, Direction::Down));
        }
    }
    if start_col < matrix.ncols() - 1 {
        let right = matrix[(start_row, start_col + 1)];
        if RIGHT_CONNECTED_PIPES.contains(&right) {
            paths.push((right, Direction::Right));
        }
    }
    paths
}

fn solution(filename: &str) -> i32 {
    let matrix = get_matrix(filename);
    let visited = DMatrix::from_element(matrix.nrows(), matrix.ncols(), 0);
    let start = find_start(&matrix).unwrap();
    let paths: Vec<Tuple> = get_paths(&matrix, start);
    println!("{:?}", paths);
    0
}

fn main() {
    assert_eq!(solution("example.txt"), 8);
    assert_eq!(solution("input.txt"), 0);
}
