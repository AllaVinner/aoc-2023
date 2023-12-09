import { useEffect, useState } from 'react'
import '../App.css'
import "../assets/prism.css"
import Prism from "../assets/prism.js"
import * as wasm from "../../wasm-src/pkg/wasm_src.js";
import AoCInput from '../components/AoCInput.jsx';



function Day03() {
    const [inputContent, setInputContent] = useState("");
    const [part1Ans, setPart1Ans] = useState("");
    const [part2Ans, setPart2Ans] = useState("");
    useEffect(() => {
        Prism.highlightAll();
    }, []);
    useEffect(() => {
        if (inputContent !== ""){
            try {
                let result = wasm.day03_part1(inputContent);
                console.log("Result", result);
                setPart1Ans(result)
            } catch (error) {
                console.log("Error", error);
                setPart1Ans(error)
            }
            try {
                let result = wasm.day03_part2(inputContent);
                console.log("Result", result);
                setPart2Ans(result)
            } catch (error) {
                console.log("Error", error);
                setPart2Ans(error)
            }
        }
    }, [inputContent])
    return (
        <>
            <div id={'day01'}>
                <h1>
                    Day 3: Gear Ratios
                </h1>
                <div>------------------------------------</div>
                <AoCInput
                    inputContent={inputContent}
                    setInputContent={setInputContent}
                />
                <div>
                    Part 1 Answer: {part1Ans}<br />
                    Part 2 Answer: {part2Ans}<br />
                    <a href="https://adventofcode.com/2023/day/2">Puzzle</a>
                    {' '}   
                    <a href='https://github.com/AllaVinner/aoc-2023/blob/main/wasm-src/src/days/day02.rs'>solution</a>
                </div>
                <h2>-- Part 1: Part Numbers</h2>
                <p className='day' >
                    In this puzzle, we want to go throuh a map and find numbers which borders symboles (non '.' characters.).
                    The formulation makes it seems like iterators would be a good idea.
                    However, what makes this more challanging is that the processing of the items in our iterator will
                    depend on parts of the input "far away" from the actual item.
                    For example, if we find a number and want to look at the characters above that number, 
                    we would have to go back a whole row of characters.
                    This makes the puzzle not straight forward to solve with the standard iterator tools.
                    So to create an iterator which provides us with the context we need, let's make our own 😊.
                </p>
                <br />
                <h3>Custom Iterator</h3>
                <p>
                    Technically, an iterator in Rust 🦀 is any struct that implements the <a href='https://doc.rust-lang.org/std/iter/index.html#implementing-iterator'>Iterator</a> Trait.
                    
                </p>
                <br />
                <br />
                <br />
                <br />
                <br />
                <br />
                <br />
                <br />
            </div>
        </>
    )
}

export default Day03
