use ndarray::prelude::*;
use std::cmp::{min, max};


#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    EMPTY,
    GALAXY
}


fn distance(a: &usize, b: &usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}


fn parse(input: &str) -> Result<Array2<Cell>, String> {
    let num_rows = input.lines().count();
    let num_columns = match input.lines().find_map(|line| Some(line.chars().count())) {
        Some(n) => n,
        None => return Err("Could not find any rows in input.".to_string())
    };
    if input
        .lines()
        .map(|line| {
            line.chars().count()
        }).any(|count| count != num_columns) 
    {
       return Err("Found columns of different lengts input.".to_string()) 
    }
    let mut grid = vec![];
    for (row_i, line) in input.lines().enumerate() {
        for (col_i, c) in line.chars().enumerate() {
            let cell = match c {
                '.' => Cell::EMPTY,
                '#' => Cell::GALAXY,
                c => return Err(format!("Encountered unexpected character: '{c}' in row {row_i} and column {col_i}.").to_string())
            };
            grid.push(cell);
        }
    }
    return match Array::from_shape_vec((num_rows, num_columns), grid) {
        Ok(array) => Ok(array),
        Err(_) => Err("Could not convert vec into array.".to_string())
    };
}


pub fn part1(input: &str) -> Result<String, String> {
    let sky: Array2<Cell> = match parse(input) {
        Ok(grid) => grid,
        Err(msg) => return Err(format!("Could not parse input. {msg}").to_string())
    };
    let empty_rows: Vec<usize> = sky
        .rows()
        .into_iter()
        .enumerate()
        .filter(|(i, r)| r.iter().all(|c| c == &Cell::EMPTY))
        .map(|(i, r)| i)
        .collect();
    let empty_columns: Vec<usize> = sky
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(i, r)| r.iter().all(|c| c == &Cell::EMPTY))
        .map(|(i, r)| i)
        .collect();
    let galaxies: Vec<[usize; 2]> = sky
        .indexed_iter()
        .filter(|((r, c), v)| v == &&Cell::GALAXY)
        .map(|((r, c), v)| [r, c])
        .collect();
    let mut total = 0;
    for (g1i, [g1r, g1c]) in galaxies.iter().take(galaxies.len()-1).enumerate() {
        for (g2i, [g2r, g2c]) in galaxies.iter().enumerate().skip(g1i+1) {
            let extra_rows = empty_rows.iter().filter(|row| min(g1r, g2r) < *row && *row < max(g1r, g2r)).count();
            let extra_cols = empty_columns.iter().filter(|col| min(g1c, g2c) < *col && *col < max(g1c, g2c)).count();
            let dist = distance(g2r, g1r) + distance(g2c, g1c) + extra_rows + extra_cols;
            total += dist;
        }
    }
    return Ok(total.to_string());
}


pub fn part2(input: &str) -> Result<String, String> {
    let sky: Array2<Cell> = match parse(input) {
        Ok(grid) => grid,
        Err(msg) => return Err(format!("Could not parse input. {msg}").to_string())
    };
    let empty_rows: Vec<usize> = sky
        .rows()
        .into_iter()
        .enumerate()
        .filter(|(i, r)| r.iter().all(|c| c == &Cell::EMPTY))
        .map(|(i, r)| i)
        .collect();
    let empty_columns: Vec<usize> = sky
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(i, r)| r.iter().all(|c| c == &Cell::EMPTY))
        .map(|(i, r)| i)
        .collect();
    let galaxies: Vec<[usize; 2]> = sky
        .indexed_iter()
        .filter(|((r, c), v)| v == &&Cell::GALAXY)
        .map(|((r, c), v)| [r, c])
        .collect();
    let mut total = 0;
    for (g1i, [g1r, g1c]) in galaxies.iter().take(galaxies.len()-1).enumerate() {
        for (g2i, [g2r, g2c]) in galaxies.iter().enumerate().skip(g1i+1) {
            let extra_rows = empty_rows.iter().filter(|row| min(g1r, g2r) < *row && *row < max(g1r, g2r)).count();
            let extra_cols = empty_columns.iter().filter(|col| min(g1c, g2c) < *col && *col < max(g1c, g2c)).count();
            total += distance(g2r, g1r) as u64 + 
                     distance(g2c, g1c) as u64 + 
                     (1_000_000-1)*extra_rows as u64 + 
                     (1_000_000-1)*extra_cols as u64;
        }
    }
    return Ok(total.to_string());
}

