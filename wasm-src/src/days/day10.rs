use std::cmp;
use std::collections::HashMap;
use std::cmp::Ordering;
use nom::{
    IResult,
    multi::{
        separated_list1
    },
    character::complete::{
        i32 as str_i32
    },
    bytes::complete::{
        tag
    }
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction{
    N,
    E,
    W,
    S
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    EMPTY,
    Pipe([Direction; 2]),
    Start
}


fn parse(input: &str) -> (Vec<Vec<Cell>>, Option<[usize; 2]>) {
    let mut grid = vec![];
    let mut start = None;
    for (row_i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (col_i, c) in line.chars().enumerate() {
            let cell = match c {
                '.' => Cell::EMPTY,
                '-' => Cell::Pipe([Direction::E, Direction::W]),
                '|' => Cell::Pipe([Direction::S, Direction::N]),
                'F' => Cell::Pipe([Direction::S, Direction::E]),
                'L' => Cell::Pipe([Direction::N, Direction::E]),
                'J' => Cell::Pipe([Direction::W, Direction::N]),
                '7' => Cell::Pipe([Direction::W, Direction::S]),
                'S' => Cell::Start,
                _ => panic!("Weird character")
            };
            if cell == Cell::Start {
                start = Some([row_i, col_i])
            }
            row.push(cell);
        }
        grid.push(row);
    }
    (grid, start)
}

fn find_initial(grid: &Vec<Vec<Cell>>, start: [usize; 2]) -> [Direction; 2] {
    let r = start[0];
    let c = start[1];
    let mut res = [Direction::S, Direction::S];
    let mut current_i = 0;
    if r > 0 {
        match grid[r-1][c] {
            Cell::Pipe(directions) => {
                if directions.contains(&Direction::S) {
                    res[current_i] = Direction::N;
                    current_i += 1;
                }
            }
            _ => ()
        }
    }
    if r < grid.len()-1 {
        match grid[r+1][c] {
            Cell::Pipe(directions) => {
                if directions.contains(&Direction::N) {
                    res[current_i] = Direction::S;
                    current_i += 1;
                }
            }
            _ => ()
        }
    }
    if c > 0 {
        match grid[r][c-1] {
            Cell::Pipe(directions) => {
                if directions.contains(&Direction::E) {
                    res[current_i] = Direction::W;
                    current_i += 1;
                }
            }
            _ => ()
        }
    }
    if c < grid[start[0]].len()-1 {
        match grid[r][c+1] {
            Cell::Pipe(directions) => {
                if directions.contains(&Direction::W) {
                    res[current_i] = Direction::E;
                    current_i += 1;
                }
            }
            _ => ()
        }
    }
    return res;
}

fn get_next_pos(pos: &[usize; 2], d: Direction) -> [usize; 2] {
    match d {
        Direction::S => [pos[0]+1, pos[1]],
        Direction::E => [pos[0], pos[1]+1],
        Direction::N => [pos[0]-1, pos[1]],
        Direction::W => [pos[0], pos[1]-1]
    }
}

fn get_next_direction(new_pos: &[usize; 2], old_d: Direction, grid: &Vec<Vec<Cell>>) -> Direction{
    match grid[new_pos[0]][new_pos[1]] {
        Cell::EMPTY => panic!("Should not be empty"),
        Cell::Start => panic!("Should not be start"),
        Cell::Pipe(ds) => {
            if old_d == ds[0] {
                ds[1]
            } else if old_d == ds[1] {
                ds[0]
            } else {
                println!("{:?}", ds);
                println!("{:?}", new_pos);
                panic!("Old d should be in new pos cell")
            }
        }
    }
}


fn main() {
    let input = "\
.....
.S-7.
.|.|.
.L-J.
....."; 

    let (grid, start) = parse(input);
    let op = find_initial(&grid, start.unwrap());
    println!("{:?}", op);
    let mut i = 1;
    let mut d1 = op[0];
    let mut d2 = op[1];
    let mut p1 = get_next_pos(&start.unwrap(), d1);
    let mut p2 = get_next_pos(&start.unwrap(), d2);
    loop {
        println!("{:?}", p1);
        println!("{:?}", p2);
        p1 = get_next_pos(&p1, d1);
        if p1 == p2 {
            break
        }
        d1 = get_next_direction(&p1, d1, &grid);
        p2 = get_next_pos(&p1, d1);
        if p1 == p2 {
            break
        }
        d2 = get_next_direction(&p2, d2, &grid);
    }
    println!("{:?}", p1)
    

    
}




