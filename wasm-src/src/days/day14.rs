







use ndarray::prelude::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
enum I {
    S,
    W,
    E
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum D {
    N,
    E,
    W,
    S
}

trait FromChar {
    fn from_char(c: char) -> Result<Self, String> where Self: Sized;
}

impl FromChar for I {
    fn from_char(c: char) -> Result<I, String> {
        let s = match c {
            '.' => I::E,
            '#' => I::W,
            'O' => I::S,
            c => return Err(format!("Encountered unexpected character: '{c}'").to_string())
        };
        return Ok(s);
    }
}

trait ToArray {
    fn str_to_array(input: &str) -> Result<Array2<Self>, String>  where Self: Sized;
}


fn parse<I: FromChar>(input: &str) -> Result<Array2<I>, String> {
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
            let cell = I::from_char(c)?;
            grid.push(cell);
        }
    }
    return match Array::from_shape_vec((num_rows, num_columns), grid) {
        Ok(array) => Ok(array),
        Err(_) => Err("Could not turn vec into array.".to_string())
    };
}


fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}


fn tilt_north(map: & mut Array2<I>) {
    let nrow = map.shape()[0];
    let ncol = map.shape()[1];
    for col in 0..ncol {
        let mut floor = 0;
        for cursor in 0..nrow {
            let mut item = map[[cursor, col]]; 
            match item {
                I::E => (),
                I::W => floor = cursor+1,
                I::S => {
                    map[[cursor, col]] = I::E;
                    map[[floor, col]] = I::S;
                    floor += 1
                    
                }
            }
        }
    }
}

fn tilt_west(map: & mut Array2<I>) {
    let nrow = map.shape()[0];
    let ncol = map.shape()[1];
    for row in 0..nrow {
        let mut floor = 0;
        for cursor in 0..ncol {
            let mut item = map[[row, cursor]]; 
            match item {
                I::E => (),
                I::W => floor = cursor+1,
                I::S => {
                    map[[row, cursor]] = I::E;
                    map[[row, floor]] = I::S;
                    floor += 1
                    
                }
            }
        }
    }
}


fn tilt_south(map: & mut Array2<I>) {
    let nrow = map.shape()[0];
    let ncol = map.shape()[1];
    for col in 0..ncol {
        let mut floor = nrow-1;
        for cursor in (0..nrow).rev() {
            let mut item = map[[cursor, col]]; 
            match item {
                I::E => (),
                I::W => {
                    if cursor > 0 {
                        floor = cursor-1;
                    }
                },
                I::S => {
                    map[[cursor, col]] = I::E;
                    map[[floor, col]] = I::S;
                    if floor > 0{
                        floor -= 1
                    } 
                    
                }
            }
        }
    }
}


fn tilt_east(map: & mut Array2<I>) {
    let nrow = map.shape()[0];
    let ncol = map.shape()[1];
    for row in 0..nrow {
        let mut floor = ncol-1;
        for cursor in (0..ncol).rev() {
            let mut item = map[[row, cursor]]; 
            match item {
                I::E => (),
                I::W => {
                    if cursor > 0 {
                        floor = cursor-1;
                    }
                },
                I::S => {
                    map[[row, cursor]] = I::E;
                    map[[row, floor]] = I::S;
                    if floor > 0{
                        floor -= 1
                    }                    
                }
            }
        }
    }
}

fn count(map: &Array2<I>) -> usize {
    let nrow = map.shape()[0];
    let ncol = map.shape()[1];
    let mut points = 0;
    for col in 0..ncol {
        for row in 0..nrow { 
            points += match map[[row, col]] {
                I::E => 0,
                I::W => 0,
                I::S => nrow-row
            }
        }
    }
    return points;
}


fn part1(input: &str) -> Result<String, String> {
    let mut map: Array2<I> = parse(input)?;
    tilt_north(&mut map);
    let value = count(&map);
    return Ok(value.to_string());
}

fn part2(input: &str) -> Result<String, String> {
    let mut map: Array2<I> = parse(input)?;
    let mut counts = vec![];
    let mut cache = vec![];
    let mut value = 0;
    //println!("{:?}", map);
    //tilt_north(& mut map);
    //println!("{:?}", map);
    //println!("{:?}", count(&map));
    let n = 1000000000;
    //let n = 23;
    let mut switch = true;
    for i in 0..n {  
        //println!("{}", printa(&map));
        let mut h = calculate_hash(&map);
        let mut point = count(&map);
        //println!("Index: {:?} Hash {:?} point {:?}, ", i, h, point);
        if switch {
            //println!("In switch");
            match cache.iter().enumerate().find(|(i, v)| **v == h) {
                None => (),
                Some((cycle_index, _)) => {
                    switch = false;
                    let cycle_len = i - cycle_index;
                    let offset = cycle_index;
                    let idx = ((n-offset) % cycle_len) + offset;
                    //println!("Index {:?} cycle_index {:?} cycle len {:?} final idx {:?}", i, cycle_index, cycle_len, idx);
                    value = counts[idx];
                    //println!("{:?}", counts);
                    break;
                }
            }
        }        
        cache.push(h);
        counts.push(point);
        tilt_north(& mut map);
        tilt_west(& mut map);
        tilt_south(& mut map);
        tilt_east(& mut map);
    }
    //println!("{}", printa(&map));
    //println!("{:?}", counts);
    //87700
    Ok(value.to_string())
}


fn printa(a: &Array2<I>) -> String {
    let mut s = String::from("");
    for row in 0..a.shape()[0]{
        for col in 0..a.shape()[1] {
            s.push(match a[[row, col]] {
                I::E => '.',
                I::S => 'O',
                I::W => '#'
            });
        }
        s.push('\n');
    }
    s
}

// n = c*k + o + i

// n % c = o + i
// 10000 = 1*k + 3

// y = k*x+m
// y //k = x
// y % k = m


// pub fn main() {
//     part1();
//     // let n = 25;
//     // let ci = 3;
//     // let cl = 7;
//     // 3+7+7+7+1
//     // 25 % 7 = 4
//     // 4 - 4
// }






