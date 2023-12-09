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
        let next_layer = cumdiff(layers[layers.len() - 1]);
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
    let mut layers: Vec<Vec<i32>> = recursive_cumdiff(v);
    let layer_len = layers.len();
    layers[layer_len - 1].push(0);
    for current_i in (1..layer_len).rev() {
        let next_i = layer_i - 1;
        let mut current_len = layers[current_i].len();
        let mut next_len = layers[next_i].len();
        let current_val = layers[i][current_len-1];
        let next_val = layers[i-1][next_len-1];
        layers[i-1].push(current_val + next_val);
    }
    layers[0][layers[0].len()-1]
}

pub fn part1(input: &str) -> String {
    return input
        .trim()
        .lines()
        .map(|line| parse_int_list(line).unwrap().1)
        .map(|line: Vec<i32>| line.into_iter().rev().collect::<Vec<_>>())
        .map(|line: Vec<i32>| predict(line))
        .sum::<i32>()
        .to_string();
}


pub fn part2(input: &str) -> String {
    return input
        .trim()
        .lines()
        .map(|line| parse_int_list(line).unwrap().1)
        .map(|line: Vec<i32>| predict(line))
        .sum::<i32>()
        .to_string();
}
