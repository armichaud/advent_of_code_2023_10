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

const UNVISITED: i32 = 0;
const ENCLOSED: i32 = -1;

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

    fn new_direction_from_coords(current: (usize, usize), next: (usize, usize)) -> Direction {
        let row_diff = next.0 as i32 - current.0 as i32;
        let col_diff = next.1 as i32 - current.1 as i32;
        match (row_diff, col_diff) {
            (-1,  0) => Direction::Up,
            ( 1,  0) => Direction::Down,
            ( 0, -1) => Direction::Left,
            ( 0,  1) => Direction::Right,
            _  => panic!("Invalid direction"),
        }
    }

    fn get_inward_direction(new_direction: Direction, previous_direction: Direction, inward_direction: Direction) -> Direction {
        match (previous_direction, new_direction, &inward_direction) {
            (Direction::Up, Direction::Left, Direction::Left) => Direction::Down,
            (Direction::Up, Direction::Right, Direction::Right) => Direction::Down,
            (Direction::Up, Direction::Left, Direction::Right) => Direction::Up,
            (Direction::Up, Direction::Right, Direction::Left) => Direction::Up,
            (Direction::Down, Direction::Left, Direction::Left) => Direction::Up,
            (Direction::Down, Direction::Right, Direction::Right) => Direction::Up,
            (Direction::Down, Direction::Left, Direction::Right) => Direction::Down,
            (Direction::Down, Direction::Right, Direction::Left) => Direction::Down,
            (Direction::Left, Direction::Up, Direction::Up) => Direction::Right,
            (Direction::Left, Direction::Down, Direction::Down) => Direction::Right,
            (Direction::Left, Direction::Up, Direction::Down) => Direction::Left,
            (Direction::Left, Direction::Down, Direction::Up) => Direction::Left,
            (Direction::Right, Direction::Up, Direction::Up) => Direction::Left,
            (Direction::Right, Direction::Down, Direction::Down) => Direction::Left,
            (Direction::Right, Direction::Up, Direction::Down) => Direction::Right,
            (Direction::Right, Direction::Down, Direction::Up) => Direction::Right,
            _ => inward_direction,
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

fn visit(visited: &mut DMatrix<i32>, current: (usize, usize)) {
    let mut stack = Vec::<(usize, usize)>::from([current]);
    while stack.len() > 0 {
        let target = stack.pop().unwrap();
        visited[target] = ENCLOSED;
        // This will never happen on the edges, so all neighbors are valid
        let neighbors = Vec::<(usize, usize)>::from([
            (target.0 - 1, target.1),
            (target.0 + 1, target.1),
            (target.0, target.1 - 1),
            (target.0, target.1 + 1),
        ]);
        for neighbor in neighbors {
            // println!("{:?}", neighbor);
            // println!("{}", visited);
            if visited[neighbor] == UNVISITED {
                stack.push(neighbor);
            }
        }
    }
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

fn tiles_enclosed(filepath: &str) -> i32 { 
    let matrix = get_matrix(filepath);
    let start = find_start(&matrix).unwrap();
    let starting_points: Vec<Pipe> = get_paths(&matrix, start);
    let first_step = starting_points.first().unwrap().clone();
    let reverse_first_step = starting_points.last().unwrap().clone();
    let mut visited = traverse(&matrix, &mut DMatrix::from_element(matrix.nrows(), matrix.ncols(), UNVISITED), first_step.clone());
    let end = visited.iter().max().unwrap().to_owned();
    visited[start] = end; // traversal does not touch starting point
    ride_rails(&mut visited, first_step.coords, end, first_step.previous_direction, reverse_first_step.previous_direction);
    visited.iter().filter(|&x| *x == ENCLOSED).count() as i32
 }

fn ride_rails(visited: &mut DMatrix<i32>, mut current: (usize, usize), end: i32, mut moving_direction: Direction, mut inward_direction: Direction) {
    while visited[current] != end {
        // println!("current {}, direction {:?}, inward {:?} visited {}", visited[current], moving_direction, inward_direction, visited);
        let inner = match inward_direction {
            Direction::Up => (current.0 - 1, current.1),
            Direction::Down => (current.0 + 1, current.1),
            Direction::Left => (current.0, current.1 - 1),
            Direction::Right => (current.0, current.1 + 1),
        };
        if visited[inner] == UNVISITED {
            visit(visited, inner);
        }
        let next_cell = visited[current] + 1;
        let mut neighbors = Vec::<(usize, usize)>::new();
        if current.0 > 0 {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.0 < visited.nrows() - 1 {
            neighbors.push((current.0 + 1, current.1));
        }
        if current.1 > 0 {
            neighbors.push((current.0, current.1 - 1));
        }
        if current.1 < visited.ncols() - 1 {
            neighbors.push((current.0, current.1 + 1));
        }
        for neighbor in neighbors {
            if visited[neighbor] == next_cell {
                let new_direction = Direction::new_direction_from_coords(current, neighbor);
                inward_direction = Direction::get_inward_direction(new_direction.clone(), moving_direction, inward_direction);
                current = neighbor;
                moving_direction = new_direction;
                break;
            }
        }
        
    }   
}

fn main() {
    assert_eq!(farthest_pipe("example.txt"), 8);
    assert_eq!(farthest_pipe("input.txt"), 7107);
    assert_eq!(tiles_enclosed("example_2.txt"), 4);
    assert_eq!(tiles_enclosed("example_3.txt"), 8);
    assert_eq!(tiles_enclosed("example_4.txt"), 10);
    assert_eq!(tiles_enclosed("input.txt"), 0);
}
