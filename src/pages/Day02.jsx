import { useEffect, useState } from 'react'
import '../App.css'
import "../assets/prism.css"
import Prism from "../assets/prism.js"
import * as wasm from "../../wasm-src/pkg/wasm_src.js";
import AoCInput from '../components/AoCInput.jsx';


function Day02() {
    const [inputContent, setInputContent] = useState("100");
    const [part1Ans, setPart1Ans] = useState("");
    const [part2Ans, setPart2Ans] = useState("");
    useEffect(() => {
        Prism.highlightAll();
    }, []);
    useEffect(() => {
        let result;
        try {
            console.log("Before")
            let result = wasm.day02(inputContent);
            if (result.length != 2) {
                console.log("Got bad result.")
                console.log(result)
                throw new Error('Result type not what expected.')
            }
            setPart1Ans(result[0])
            setPart2Ans(result[1])
        } catch (error) {
            console.log('In error part')
            setPart1Ans("<Invalid Input>")
            setPart2Ans("<Invalid Input>")
        }
    }, [inputContent])
    return (
        <>
            <div id={'day02'} className='day'>
                <h1>
                    Day 02 - The Colored Cubes
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
                {"--------------------"}<br />
                <h2>Part 1 - Possible Games</h2>
                <p className='day'>
                    We want to know which games would be possible given the number of red, green, and blue cubes.
                    When we know which games are of interest, we simply add up the games. 
                    
                </p>
                
            </div>
        </>
    )
}

export default Day02
