import { useEffect, useState } from 'react'
import '../App.css'
import "../assets/prism.css"
import Prism from "../assets/prism.js"
import * as wasm from "../../wasm-src/pkg/wasm_src.js";
import AoCInput from '../components/AoCInput.jsx';

const handStruct = `\
#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    rank: u32
}
`

const findRank = `\
fn find_rank(cards: &Vec<u32>) -> u32 {
    let mut counter = HashMap::new();
    for card in cards.iter() {
        if let Some(count) = counter.get_mut(card) {
            *count += 1;
        } else {
            counter.insert(card, 1);
        }
    }
    let mut values: Vec<u32> = counter.into_values().collect();
    return if values.contains(&5) {
        7
    } else if values.contains(&4) {
        6
    } else if values.contains(&3) && values.contains(&2) {
        5
    } else if values.contains(&3) {
        4
    } else if values.iter().filter(|v| *v == &2).count() == 2 {
        3
    } else if values.contains(&2) {
        2
    } else if values.iter().filter(|v| *v == &1).count() == values.len() {
        1
    } else {
        panic!("Got hand that did not fit one of the asserted options.".to_string())
    };
}
`


function Day07() {
    const [inputContent, setInputContent] = useState("");
    const [part1Ans, setPart1Ans] = useState("");
    const [part2Ans, setPart2Ans] = useState("");
    useEffect(() => {
        Prism.highlightAll();
    }, []);
    useEffect(() => {
        if (inputContent !== ""){
            try {
                let result = wasm.day07_part1(inputContent);
                console.log("Result", result);
                setPart1Ans(result)
            } catch (error) {
                console.log("Error", error);
                setPart1Ans("<Invalid Input>")
            }
            try {
                let result = wasm.day07_part2(inputContent);
                console.log("Result", result);
                setPart2Ans(result)
            } catch (error) {
                console.log("Error", error);
                setPart2Ans("<Invalid Input>")
            }
        }
    }, [inputContent])
    return (
        <>
            <div id={'day07'}>
                <h1>
                    Day 07: camelCards  🐫
                </h1>
                <div>------------------------------------------</div>
                <AoCInput
                    inputContent={inputContent}
                    setInputContent={setInputContent}
                />
                <div>
                    Part 1 Answer: {part1Ans}<br />
                    Part 2 Answer: {part2Ans}<br />
                    <a href="https://adventofcode.com/2023/day/7">Puzzle</a>
                    {' '}   
                    <a href='https://github.com/AllaVinner/aoc-2023/blob/main/wasm-src/src/days/day07.rs'>solution</a>
                </div>
                <h2> Part 1: J for Jack</h2>
                <p>
                    So basically, figure out the rank of each hand, sort by that rank, multiply by the bet, and then sum up. <br />
                    We got this! 😄 <br />
                    Let's start with a struct for our data: <br />
                </p>
                <code className='language-rust'>{handStruct}</code> <br />
                <p>
                    We can then parse each card to a number (its value), where T is 10, and A is 14.
                    Then we can figure out the rank where we can set a 5-pair to be rank 7, and all unique 1.
                    The numbers do not matter, we just need them to be in the right order.
                    However, since the order of the cards do not matter, we can create a counter, which counts the 
                    number of instances of each card in each hand.
                    This <a href='https://doc.rust-lang.org/std/collections/struct.HashMap.html'> HashMap </a> is then 
                    easier the check for the different types of hands.
                </p>
                <code className='language-rust'>{findRank}</code> <br />
                
            </div>
        </>
    )
}

export default Day07
