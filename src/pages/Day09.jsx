import { useEffect, useState } from 'react'
import '../App.css'
import "../assets/prism.css"
import Prism from "../assets/prism.js"
import * as wasm from "../../wasm-src/pkg/wasm_src.js";
import AoCInput from '../components/AoCInput.jsx';

const cumdiffCode = `\
fn cumdiff(v: &Vec<i32>) -> Vec<i32> {
    v.iter().scan(0, |s, v| {
        let diff = *v - *s;
        *s = *v;
        Some(diff)
    }).skip(1).collect() // First diff is artificial
}
`

const recursiveCumdiffCode = `\
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
`

const predictNextCode = `\
fn predict_next(v: Vec<i32>) -> Vec<Vec<i32>> {
    let mut layers: Vec<Vec<i32>> = recursive_cumdiff(v);
    let layer_len = layers.len();
    layers[layer_len - 1].push(0); // Add zero
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
`

const part1MainCode = `\
fn part1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|line| parse_int_list(line).unwrap().1)
        .map(|line: Vec<i32>| predict_next(line))
        .sum::<i32>()
}
`

const parseCode = `\
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

fn parse_int_list(line:  &str) -> IResult<&str, Vec<i32>> {
    separated_list1(
        tag(" "),
        str_i32
    )(line)
}
`

const part2MainCode = `\
fn part1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|line| parse_int_list(line).unwrap().1)
        .map(|line: Vec<i32>| line.into_iter().rev().collect::<Vec<_>>())
        .map(|line: Vec<i32>| predict_next(line))
        .sum::<i32>()
}
`


function Day09() {
    const [inputContent, setInputContent] = useState("");
    const [part1Ans, setPart1Ans] = useState("");
    const [part2Ans, setPart2Ans] = useState("");
    
    useEffect(() => {
        Prism.highlightAll();
    }, []);

    useEffect(() => {
        if (inputContent !== "") {
            try {
                let result = wasm.day09_part1(inputContent);
                console.log("Result", result);
                setPart1Ans(result)
            } catch (error) {
                console.log("Error: ", error);
                setPart1Ans("<Invalid Input>")
            }
            try {
                let result = wasm.day09_part2(inputContent);
                console.log("Result", result);
                setPart2Ans(result)
            } catch (error) {
                console.log("Error: ", error);
                setPart2Ans("<Invalid Input>")
            }
        }
    }, [inputContent])

    return (
        <>
            <div id={'day09'}>
                <h1>
                    Day 09: Mirage Maintenance
                </h1>
                <div>----------------------------------------------------</div>
                <AoCInput
                    inputContent={inputContent}
                    setInputContent={setInputContent}
                />
                <div>
                    Part 1 Answer: {part1Ans}<br />
                    Part 2 Answer: {part2Ans}<br />
                    <a href="https://adventofcode.com/2023/day/9">Puzzle</a>
                    {' '}
                    <a href='https://github.com/AllaVinner/aoc-2023/blob/main/wasm-src/src/days/day09.rs'>solution</a>
                </div> <br />
                <p>
                    Ohh!! This is a fun version of <y>What comes next?</y>. For a VERY thorough exploration, check out <a href='https://www.youtube.com/watch?v=iJ8pnCO0nTY'>this Mathologer video</a>.
                    But we will not make it too complicated. Let's collect the integer vec in each line, compute all the sub-vecs by taking the difference, and 
                    finally recreate the next value in each array.<br /><br />
                </p>
                <h2> Part 1: What comes next?</h2>
                <p>
                    To create the next sub-vec, we need to take the running difference, maybe use <a href='https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan'>scan</a>?
                </p>
                <code className='language-rust'>{cumdiffCode}</code> <br />
                <p> 
                    Now lets do that until we only have zeros.
                </p>
                <code className='language-rust'>{recursiveCumdiffCode}</code> <br />
                <p>
                    Finally, add the zero to the last vec, and then add up the last values of each 
                    consecutive vec, and return the final sum.
                </p>
                <code className='language-rust'>{predictNextCode}</code> <br />
                <p>
                    Now parse each line and pass it through this function before summing up the values.
                </p>
                <code className='language-rust'>{part1MainCode}</code> <br />
                <p>
                    Where the parsing can easily be done with <a href='https://docs.rs/nom/latest/nom/' >nom</a>.
                </p>
                <code className='language-rust'>{parseCode}</code> <br />
                
                <h2> Part 2: What comes before?</h2>
                <p>
                    Same setup, but now we will predict what comes before. This is simple as it is still fundamenally the
                    same <y>What comes next?</y> problem. So lets reverse the lists and call it a day 
                </p>
                <code className='language-rust'>{part2MainCode}</code> <br />
                
            </div>
        </>
    )
}

export default Day09
