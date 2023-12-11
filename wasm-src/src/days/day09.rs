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


fn cumdiff(v: &Vec<i32>) -> Vec<i32> {
    v.iter().scan(0, |s, v| {
        let diff = *v - *s;
        *s = *v;
        Some(diff)
    }).skip(1).collect() // First diff is artificial
}


fn recursive_cumdiff(v: Vec<i32>) -> Vec<Vec<i32>> {
    let mut layers = vec![v];
    loop {
        let next_layer = cumdiff(&layers[layers.len() - 1]);
        layers.push(next_layer);
        if layers[layers.len()-1].iter().all(|v| *v == 0) {
            break;
        }
    }
    return layers;
}


fn parse_int_list(line:  &str) -> IResult<&str, Vec<i32>> {
    separated_list1(
        tag(" "),
        str_i32
    )(line)
}

fn predict_next(nums: Vec<i32>) -> i32 {
    let mut layers: Vec<Vec<i32>> = recursive_cumdiff(nums);
    let layer_len = layers.len();
    layers[layer_len - 1].push(0);
    for current_i in (1..layer_len).rev() {
        let next_i = current_i - 1;
        let mut current_len = layers[current_i].len();
        let mut next_len = layers[next_i].len();
        let current_val = layers[current_i][current_len-1];
        let next_val = layers[next_i][next_len-1];
        layers[next_i].push(current_val + next_val);
    }
    layers[0][layers[0].len()-1]
}

pub fn part1(input: &str) -> Result<String, String> {
    return Ok(
        input
        .trim()
        .lines()
        .map(|line| match parse_int_list(line) {
            Ok((s, v)) => Ok(v),
            Err(e) => return Err("Could not parse int list".to_string())
        })
    .map(|line: Result<Vec<i32>, String>| Ok(predict_next(line?)))
        .fold(Ok(0), |a: Result<i32, String>, x: Result<i32, String>| Ok(a? + x?))?
        .to_string()
    );
}


pub fn part22(input: &str) -> Result<String, String> {
    return Ok(
        input
        .trim()
        .lines()
        .map(|line| match parse_int_list(line) {
            Ok((s, v)) => Ok(v),
            Err(e) => return Err("Could not parse int list".to_string())
        })
        .map(|nums| Ok(nums?.into_iter().rev().collect::<Vec<_>>()))
        .map(|line: Result<Vec<i32>, String>| Ok(predict_next(line?)))
        .fold(Ok(0), |a: Result<i32, String>, x: Result<i32, String>| Ok(a? + x?))?
        .to_string()
    );
}

pub fn part2(input: &str) -> Result<String, String> {
    let mut sum = 0;
    for line in input.trim().lines() {
        let mut nums = match parse_int_list(line) {
            Ok((s, v)) => v.into_iter().rev().collect(),
            Err(e) => return Err("Could not parse int list".to_string())
        };
        sum += predict_next(nums);
    }
    Ok(sum.to_string())
}

