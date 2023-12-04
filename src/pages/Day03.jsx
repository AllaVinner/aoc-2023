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
                    Day 03
                </h1>
                <div>-----------------------------------------------------</div>
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
                <div>Hello</div>
            </div>
        </>
    )
}

export default Day03
