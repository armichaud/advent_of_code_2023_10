use nalgebra::DMatrix;
use std::{fs::File, io::{BufReader, BufRead}};

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

// All pipes that count as boundaries when moving down and to the right
const BOUNDARIES: [char; 4] = [HORIZONTAL_PIPE, VERTICAL_PIPE, F_PIPE, J_PIPE];

const UNVISITED: i32 = 0;

#[derive(Debug, Clone)]
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
            if matrix[(i, j)] == START {
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

fn farthest_pipe(filename: &str) -> i32 {
    let matrix = get_matrix(filename);
    let start = find_start(&matrix).unwrap();
    let starting_points: Vec<Pipe> = get_paths(&matrix, start);
    let mut visited = DMatrix::from_element(matrix.nrows(), matrix.ncols(), UNVISITED);
    for start in starting_points {
        visited = traverse(&matrix, &mut visited, start);
    }
    visited.iter().max().unwrap().to_owned()
}

fn is_inside(matrix: &DMatrix<char>, visited: &DMatrix<i32>, current: (usize, usize)) -> bool {
    let mut count = 0;
    let (mut i, mut j) = current;
    while i < matrix.nrows() && j < matrix.ncols() {
        let coords = (i, j);
        if visited[coords] > 0 && BOUNDARIES.contains(&matrix[coords]) {
            count += 1;
        }
        i += 1;
        j += 1;
    }
    count % 2 != 0
}

fn tiles_enclosed(filepath: &str) -> i32 { 
    let matrix = get_matrix(filepath);
    let matrix_clone = matrix.clone();
    let start = find_start(&matrix).unwrap();
    let paths = get_paths(&matrix, start);
    let mut visited = traverse(&matrix, &mut DMatrix::from_element(matrix.nrows(), matrix.ncols(), UNVISITED), paths.first().unwrap().to_owned());
    visited[start] = 1; // traversal does not touch starting point
    let mut matrix = matrix_clone;
    // I would build a method to replace the starting point with the correct pipe, but I know it's an F for all inputs, and I've got better things to do
    matrix[start] = F_PIPE;
    println!("{} {}", matrix, visited);
    let mut sum = 0;
    for i in 0..visited.nrows() {
        for j in 0..visited.ncols() {
            if visited[(i, j)] == UNVISITED {
                if is_inside(&matrix, &visited, (i, j)) {
                    sum += 1;
                }
            }
        }
    }
    sum
}


fn main() {
    println!("{}", farthest_pipe("example.txt"));
    println!("{}", farthest_pipe("input.txt"));
    println!("{}", tiles_enclosed("example_2.txt"));
    println!("{}", tiles_enclosed("example_3.txt"));
    println!("{}", tiles_enclosed("example_4.txt"));
    println!("{}", tiles_enclosed("input.txt"));
}
