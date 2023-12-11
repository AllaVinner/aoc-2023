use std::collections::HashMap;

use nom::{
    IResult,
    bytes::complete::{
        tag,
    },
    multi::{
        separated_list1,
    },
    character::complete::{
        alpha1,
        alphanumeric1
    },
    sequence::{
        separated_pair,
        delimited
    }
};


#[derive(Debug, Copy, Clone)]
enum Direction {
    L,
    R
}


fn parse_map_line(input: &str) -> IResult<&str,(String, (String, String))> {
    let (input, (name, (l, r))) =
        separated_pair(
            alphanumeric1,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(
                    alphanumeric1,
                    tag(", "),
                    alphanumeric1
                ),
                tag(")")
            )        
        )(input)?;
    return Ok((input, (name.to_string(), (l.to_string(), r.to_string()))));
}


pub fn part1(input: &str) -> Result<String, String> {
    let (direction_str, map_str) = match input.trim().split_once("\n\n") {
        Some(s) => s,
        None => return Err("Could not find double newline to split the input int a direction and map part.".to_string())
    };
    let dircetions: Vec<Direction> = direction_str.trim().chars().map(|c| match c {
        'L' => Ok(Direction::L),
        'R' => Ok(Direction::R),
        _ => Err(format!("Found character '{c}' in direction string, expected only L and R.").to_string())
    }).collect::<Result<Vec<_>, String>>()?;
    let map: HashMap<String, (String, String)> = map_str.trim().lines().map(|line| match parse_map_line(line) {
        Ok(v) => Ok(v.1),
        Err(e) => return Err("Could not parse map line.".to_string())
    }).collect::<Result<HashMap<String, (String, String)>, String>>()?;
    
    let mut pos = "AAA";
    let mut count = 0;
    for (i, direction) in dircetions.iter().cycle().enumerate() {
        println!("{:?}", pos);
        let (left_pos, right_pos) = match map.get(pos) {
            Some(p) => p,
            None => return Err("Did not find pos in map.".to_string())
        };
        pos = match direction {
            Direction::L => left_pos,
            Direction::R => right_pos
        };
        if pos == "ZZZ" {
            count = i + 1;
            break;
        }
    }
    return Ok(count.to_string());    
}



pub fn part2(input: &str) -> Result<String, String> {
    let (direction_str, map_str) = match input.trim().split_once("\n\n") {
        Some(s) => s,
        None => return Err("Could not find double newline to split the input int a direction and map part.".to_string())
    };
    let dircetions: Vec<Direction> = direction_str.trim().chars().map(|c| match c {
        'L' => Ok(Direction::L),
        'R' => Ok(Direction::R),
        _ => Err(format!("Found character '{c}' in direction string, expected only L and R.").to_string())
    }).collect::<Result<Vec<_>, String>>()?;
    let map: HashMap<String, (String, String)> = map_str.trim().lines().map(|line| match parse_map_line(line) {
        Ok(v) => Ok(v.1),
        Err(e) => return Err("Could not parse map line.".to_string())
    }).collect::<Result<HashMap<String, (String, String)>, String>>()?;
    
    let mut positions:  Vec<String> = map.keys().filter(|k| k.ends_with("A")).map(|k| k.to_string()).collect();
    let original = positions.clone();
    return Ok(format!("Num A: ends").to_string());
    let mut count = 0;
    for (i, direction) in dircetions.iter().cycle().enumerate() {
        let new_position = vec![];
        for pos in positions.iter() {
            let (left_pos, right_pos) = match map.get(pos) {
                Some(p) => p,
                None => return Err("Did not find pos in map.".to_string())
            };
            let new_pos = match direction {
                Direction::L => left_pos.to_string(),
                Direction::R => right_pos.to_string()
            };
            new_position.push(new_pos)
        }
        if i % dircetions.len() == 0 {
            if positions.iter().all(|k| k.ends_with("Ak")) {
                count = i + 1;
                break;
            }
        }
        
    }
    return Ok(count.to_string());    
}






