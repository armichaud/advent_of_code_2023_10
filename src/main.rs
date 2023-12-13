use nalgebra::DMatrix;
use std::{fs::File, io::{BufReader, BufRead}};

struct Pipe { 
    symbol: char, 
    previous_direction: Direction, 
    coords: (usize, usize)
}

#[derive(PartialEq, Debug, Clone)]
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
    fn from_char_and_direction(c: char, previous: &Direction) -> Option<Direction> {
        match c {
            HORIZONTAL_PIPE => Some(previous.clone()),
            VERTICAL_PIPE => Some(previous.clone()),
            L_PIPE => Some(if previous == &Direction::Down {Direction::Right} else {Direction::Up}),
            J_PIPE => Some(if previous == &Direction::Down {Direction::Left} else {Direction::Up}),
            SEVEN_PIPE => Some(if previous == &Direction::Up {Direction::Left} else {Direction::Down}),
            F_PIPE => Some(if previous == &Direction::Up {Direction::Right} else {Direction::Down}),
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

fn get_paths(matrix: &DMatrix<char>, start: (usize, usize)) -> Vec<Pipe> {
    let mut paths: Vec<Pipe> = Vec::new();
    let (start_row, start_col) = start;

    if start_row > 0 {
        let coords = (start_row - 1, start_col);
        let up = matrix[coords];
        if TOP_CONNECTED_PIPES.contains(&up) {
            paths.push(Pipe { symbol: up, previous_direction: Direction::Up, coords });
        }
    }
    if start_col > 0 {
        let coords = (start_row, start_col - 1);
        let left = matrix[coords];
        if LEFT_CONNECTED_PIPES.contains(&left) {
            paths.push(Pipe { symbol: left, previous_direction: Direction::Left, coords });
        }
    }
    if start_row < matrix.nrows() - 1 {
        let coords = (start_row + 1, start_col);
        let down = matrix[coords];
        if BOTTOM_CONNECTED_PIPES.contains(&down) {
            paths.push(Pipe { symbol: down, previous_direction: Direction::Down, coords });
        }
    }
    if start_col < matrix.ncols() - 1 {
        let coords = (start_row, start_col + 1);
        let right = matrix[coords];
        if RIGHT_CONNECTED_PIPES.contains(&right) {
            paths.push(Pipe { symbol: right, previous_direction: Direction::Right, coords });
        }
    }
    paths
}

fn traverse(matrix: &DMatrix<char>, visited: &mut DMatrix<i32>, start: Pipe) -> DMatrix<i32> {
    let mut current = start;
    let mut count = 1;
    while matrix[(current.coords.0, current.coords.1)] != START {
        if visited[current.coords] == 0 || visited[current.coords] > count {
            visited[current.coords] = count;
        }
        count += 1;
        let new_direction = Direction::from_char_and_direction(current.symbol, &current.previous_direction).unwrap();
        match new_direction {
            Direction::Up => {
                current.coords.0 -= 1;
            },
            Direction::Down => {
                current.coords.0 += 1;
            },
            Direction::Left => {
                current.coords.1 -= 1;
            },
            Direction::Right => {
                current.coords.1 += 1;
            },
        }
        current.previous_direction = new_direction;
        current.symbol = matrix[current.coords];
    }
    visited.to_owned()
}

fn solution(filename: &str) -> i32 {
    let matrix = get_matrix(filename);
    let mut visited = DMatrix::from_element(matrix.nrows(), matrix.ncols(), 0);
    let start = find_start(&matrix).unwrap();
    let starting_points: Vec<Pipe> = get_paths(&matrix, start);
    for start in starting_points {
        visited = traverse(&matrix, &mut visited, start);
    }
    visited.iter().max().unwrap().to_owned()
}

fn main() {
    assert_eq!(solution("example.txt"), 8);
    assert_eq!(solution("input.txt"), 7107);
}
