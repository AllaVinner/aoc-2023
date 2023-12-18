import { useEffect, useState } from 'react'
import '../App.css'
import "../assets/prism.css"
import Prism from "../assets/prism.js"
import * as wasm from "../../wasm-src/pkg/wasm_src.js";
import AoCInput from '../components/AoCInput.jsx';


function Day14() {
    const [inputContent, setInputContent] = useState("");
    const [part1Ans, setPart1Ans] = useState("");
    const [part2Ans, setPart2Ans] = useState("");

    useEffect(() => {
        Prism.highlightAll();
    }, []);

    useEffect(() => {
        if (inputContent !== "") {
            try {
                let result = wasm.day14_part1(inputContent);
                console.log("Result", result);
                setPart1Ans(result)
            } catch (error) {
                console.log("Error: ", error);
                setPart1Ans("<Invalid Input>")
            }
            try {
                let result = wasm.day14_part2(inputContent);
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
            <div id={'day14'}>
                <h1>
                    Day 14: Cosmic Expansion
                </h1>
                <div>----------------------------------------------------</div>
                <AoCInput
                    inputContent={inputContent}
                    setInputContent={setInputContent}
                />
                <div>
                    Part 1 Answer: {part1Ans}<br />
                    Part 2 Answer: {part2Ans}<br />
                    <a href="https://adventofcode.com/2023/day/14">Puzzle</a>
                    {' '}
                    <a href='https://github.com/AllaVinner/aoc-2023/blob/main/wasm-src/src/days/day14.rs'>solution</a>
                </div> <br />
                <h2>Part 1: The North</h2>
                
            </div>
        </>
    )
}

export default Day14
