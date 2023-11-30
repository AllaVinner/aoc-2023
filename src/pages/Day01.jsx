import { useEffect, useState } from 'react'
import '../App.css'
import "../assets/prism.css"
import Prism from "../assets/prism.js"
import * as wasm from "../../wasm-src/pkg/wasm_src.js";
import AoCInput from '../components/AoCInput.jsx';

let helloRustCode = `\
fn main() {
    println!("Hello, world!");
}
`

const readWriteCode = `\
use std::fs;

fn main() {
    // Read
    let file: String = fs::read_to_string("./input.txt")
        .expect("Could not read file")  // Panics if we cannot read the file.
        .replace("\\r", "");  // because typewritters ...
    // Do some calulations
    let answer_1 = day01_part1(&file);  // TODO
    let answer_2 = day01_part2(&file);  // TODO
    // Write
    println!("Answer to part 1: {}", answer_1);
    println!("Answer to part 2: {}", answer_2);
}
`

function Day01() {
    const [inputContent, setInputContent] = useState("100");
    const [answere, setAnswere] = useState("");
    useEffect(() => {
        Prism.highlightAll();
    }, []);
    let s = `\
How to start learning web development?
fn main() {
   let hello = vec!["sdf"df]
}
// The answere is
`   
    useEffect(() => {
        try {
            console.log('Before wasm')
            console.log(inputContent.charAt(0))
            console.log(inputContent.charAt(1))
            console.log(inputContent.charAt(2))
            let new_ans = wasm.day01_part1(inputContent);
            console.log('After wasm')
            console.log(new_ans);
            setAnswere(new_ans)
        } catch (error) {
            setAnswere("<Invalid Input>")
        }
    }, [inputContent])
    return (
        <>
            <div id={'day01'} className='day'>
                <h1>
                    Day 01
                </h1>
                <div>------------</div>
                <br />
                <p className='day'>
                    It is the most wonderful time of the year! 🎅🎄🎅🎄<br />
                    Why not make use of the darkness and learn somethign new? Why not Rust🦀? 
                    I aim to make a solution in Rust which I then compile to <nobr>🟪WASM</nobr> and can be run here in the browser.
                    There will also be an explination of the code, hopefully with some interactivity, and you can play around with the input as you wish.
                </p>
                <br />
                <p className='day'>Try it out!</p>
                <AoCInput
                    inputContent={inputContent}
                    setInputContent={setInputContent}
                />
                <div>
                    Part 1 Answer: {answere}<br />
                    Part 2 Answer: {answere}<br />
                    <a href="https://adventofcode.com/2022/day/9">Puzzle</a>
                    {' '}   
                    <a href='https://github.com/AllaVinner/Advent-Code-2022/blob/main/rust/day-02/src/main.rs'>source code</a>
                </div>
                {"---------------------------"}
                <br />
                <p className='day'>
                    So how do we do this? <br />
                    I will not go into the details of how to setup rust but in short <a href='https://www.rust-lang.org/tools/install'>download</a>,
                    run <code>cargo new aoc-2023</code>, and open <code>./src/main.rs</code>.
                    It will look something like this: <br />
                    <code className='language-rust'>
                        {helloRustCode}
                    </code>
                    <br />
                    To run the code, execute <code>cargo run</code> in your terminal. <br />
                    The compiler will look for a main function in the <code>main.rs</code> file to start the execution. <br />
                    There are four things we need to do.
                    <ol>
                        <li>Read in the data.</li>
                        <li>Parse data to sensible Rust type.</li>
                        <li>Calulate answer.</li>
                        <li>Return or print answer.</li>
                    </ol>
                    The first and last steps will more or less be the same each day, and can be done by e.g.
                    <br />
                    <code className='language-rust'>
                        {readWriteCode}
                    </code>
                    <br />
                    The actuall fun then starts with the TODO's, where we take a string (slice) and return something which can be printed.
                    So for now, just take the code as a template and let's start the implementation.
                </p>
                    
                <pre>

                </pre>

                <a href="https://adventofcode.com/2022/day/9">Today's challenge</a>
                <pre>
                    <code class="language-rust">
                        let variable = "Here's some JavaScript";
                        for a in 
                    </code>
                </pre>    
                <code class="language-rust">
                    {s}
                </code>
                <div>
                    Hello
                    <y>From</y>
                    <br />
                    <code>{s}</code>
                </div>
            </div>
        </>
    )
}

export default Day01
