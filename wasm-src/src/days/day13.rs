use ndarray::prelude::*;
use std::cmp::min;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Ash,
    Rock
}

fn parse(input: &str) -> Result<Array2<Cell>, String> {
    let num_rows = input.lines().count();
    let num_columns = match input.lines().find_map(|line| Some(line.chars().count())) {
        Some(n) => n,
        None => return Err("Could not find any rows in input.".to_string())
    };
    if input.lines().map(|line| line.chars().count()).any(|count| count != num_columns) {
       return Err("Found columns of different lengts.".to_string()) 
    }
    let mut grid = vec![];
    for (row_i, line) in input.lines().enumerate() {
        for (col_i, c) in line.chars().enumerate() {
            let cell = match c {
                '.' => Cell::Ash,
                '#' => Cell::Rock,
                c => return Err(format!("Encountered unexpected character: '{c}'").to_string())
            };
            grid.push(cell);
        }
    }
    return match Array::from_shape_vec((num_rows, num_columns), grid) {
        Ok(array) => Ok(array),
        Err(_) => Err("Could not turn vec into array.".to_string())
    };
}


fn is_row_mirror(row: usize, map: &Array2<Cell>) -> bool {
    let num_repeat = min(map.shape()[0] - row, row);
    for i in 0..num_repeat {
        if map.row(row-i-1) != map.row(row+i) {
            return false;
        }
    }
    return true;
}

fn find_row_mirror(map: &Array2<Cell>) -> Option<usize> {
    for row in 1..map.shape()[0] {
        if is_row_mirror(row, map) {
            return Some(row);
        }
    }
    return None;
}

fn is_col_mirror(col: usize, map: &Array2<Cell>) -> bool {
    let num_repeat = min(map.shape()[1] - col, col);
    for i in 0..num_repeat {
        if map.column(col-i-1) != map.column(col+i) {
            return false;
        }
    }
    return true;
}

fn find_col_mirror(map: &Array2<Cell>) -> Option<usize> {
    for col in 1..map.shape()[1] {
        if is_col_mirror(col, map) {
            return Some(col);
        }
    }
    return None;
}

fn find_mirror(map: &Array2<Cell>) -> (Option<usize>, Option<usize>) {
    return (
        find_row_mirror(map),
        find_col_mirror(map)
    )
}

fn main() {
    let a = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    let b = a.split("\n\n")
        .map(|s| parse(s).unwrap())
        .map(|map| find_mirror(&map))
        .map(|(r, c)| {
            let a = match r {
                Some(rr) => 100*rr,
                None => 0
            }; 
            let b = match c {
                Some(cc) => cc,
                None => 0
            };
            return a + b;
        } ).sum::<usize>();
    println!("{:?}", b);
    
}