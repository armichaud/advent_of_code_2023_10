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
