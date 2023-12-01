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

const basicPart1Code = `\
pub fn part1(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| extract_number(line))
        .sum();
}
`

const part1LineParsingCode = `\
fn extract_number(line: &str) -> u32 {
    let mut detector = line
        .chars()
        .filter_map(|c: char| c.to_digit(10));
    let first = detector.next().unwrap_or(0);
    let last = detector.last().unwrap_or(first);
    return first*10 + last;
}
`

const compilerMutCode = `\
error[E0596]: cannot borrow 'detector' as mutable, as it is not declared as mutable
 --> src\days\day01.rs:8:17
  |
8 |     let first = detector.next().unwrap_or(0);
  |                 ^^^^^^^^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
5 |     let mut detector = line
  |         +++
`

const part2ExtractCode = `\
fn extract_number_including_written(line: &str) -> u32 {
    let mut detector = (0..line.len())
        .map(|i| &line[i..])
        .map(|remaining_line| {
            return if remaining_line.starts_with("one") {
                '1'
            } else if remaining_line.starts_with("two") {
                '2'
            } else if remaining_line.starts_with("three") {
                '3'
            } else if remaining_line.starts_with("four") {
                '4'
            } else if remaining_line.starts_with("five") {
                '5'
            } else if remaining_line.starts_with("six") {
                '6'
            } else if remaining_line.starts_with("seven") {
                '7'
            } else if remaining_line.starts_with("eight") {
                '8'
            } else if remaining_line.starts_with("nine") {
                '9'
            } else {
                remaining_line.chars().next().unwrap()
            }; 
        })
        .filter_map(|c: char| c.to_digit(10));    
    let first = detector.next().unwrap_or(0);
    let last = detector.last().unwrap_or(first);
    return first*10 + last;
}
`


function Day01() {
    const [inputContent, setInputContent] = useState("100");
    const [part1Ans, setPart1Ans] = useState("");
    const [part2Ans, setPart2Ans] = useState("");
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
        let result;
        try {
            console.log("Before")
            let result = wasm.day01(inputContent);
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
            <div id={'day01'} className='day'>
                <h1>
                    Day 01
                </h1>
                <div>------------</div>
                <br />
                <p className='day'>
                    It is the most wonderful time of the year! 🎅🎄🎅🎄<br />
                    Why not make use of the darkness and learn somethign new? Why not Rust🦀? 
                    I aim to make a solution in Rust which I then compile to <nobr>🟪WASM</nobr> which can be run here in the browser.
                    There will also be an explanation of the code, hopefully with some interactivity, and you can play around with the input as you wish.
                </p>
                <br />
                <p className='day'>Try it out!</p>
                <AoCInput
                    inputContent={inputContent}
                    setInputContent={setInputContent}
                />
                <div>
                    Part 1 Answer: {part1Ans}<br />
                    Part 2 Answer: {part2Ans}<br />
                    <a href="https://adventofcode.com/2023/day/1">Puzzle</a>
                    {' '}   
                    <a href='https://github.com/AllaVinner/aoc-2023/blob/main/wasm-src/src/days/day01.rs'>source code</a>
                </div>
                {"---------------------------"}
                <br />
                <h2>-- Setup --</h2>
                <p className='day'>
                    So how do we do this? <br />
                    I will not go into the details of how to setup rust but in short <a href='https://www.rust-lang.org/tools/install'>download</a>,
                    run <code>cargo new aoc-2023</code>, and open <code>./src/main.rs</code>.
                    It will look something like this: <br />
                    <code className='language-rust block'>
                        {helloRustCode}
                    </code>
                    <br />
                    To run the code, execute <code>cargo run</code> in your terminal. <br />
                    The compiler will look for a main function in the <code>main.rs</code> file to start the execution. <br />
                    There are four things we need to do.
                </p>
                <ol>
                    <li>Read in the data.</li>
                    <li>Parse data to sensible Rust type.</li>
                    <li>Calulate answer.</li>
                    <li>Return or print answer.</li>
                </ol>
                <p className='day'>
                    The first and last steps will more or less be the same each day, and can be done by e.g.
                    <br />
                    <code className='language-rust'>
                        {readWriteCode}
                    </code>
                    <br />
                    The actuall fun then starts with the TODO's, where we take a string (slice) and return something which can be printed.
                    So for now, just take the code as a template and let's start the implementation. <br />
                    <br />
                    <br />
                    <h2>-- Part 1: Digit Recovery</h2>
                    <br />
                    The first task is to, for each line in the input, concatenate the first and last digit, and then sum up all the concatenated values.
                    Rust is influenced by functional progamming, and hence iterators are a thing. 
                    In this case we would like to <b>iterate</b> over the lines of the input. 
                    Going to the <a href='https://doc.rust-lang.org/std/primitive.str.html'>documentation</a> of the <code>str</code> type, 
                    we find the function <code>lines</code> which seems to be doing just that. 
                    Now for each line, we would like to parse out the concatenated digit value, let's save that for later, 
                    and then sum the result of all values. 
                    Again going to the documentation, but this time for <a href='https://doc.rust-lang.org/std/iter/trait.Iterator.html'>iterators</a>,
                    we find the function <code>map</code>, that applies a function to each item in an iterator and returns a new iterator over the results.
                    Staying in the same documentation, we also find a <code>sum</code>, function.<br />
                    <code className='language-rust block'>{basicPart1Code}</code> <br />
                    The final step is then to create the <code>extract_number</code> function. 
                    We can again create an iterator, but this time an iterator over the line that returns the digits it find.
                    We can then ask the iterator for the first and last object before concatenating them.
                    So going back to the <code>str</code> documentation, we find the <code>chars</code> method,
                    that returns an iterator over the characers of the string. 
                    We then want to throw away the characters that are not digits, and convert the ones that are. 
                    Looking at the iterator documentation, the <code>filter_map</code> function seems promising.
                    It wants us to return <code>Some</code>... time to read up on <a href='https://doc.rust-lang.org/std/option/'>options</a><div className=""></div>
                    We can then take a deep dive into the <a href='https://doc.rust-lang.org/std/primitive.char.html'>char</a> documentation
                    where we find the <code>to_digit</code> function, which seems to play perfectly with our option type.
                    Putting the steps together, and looking abit more at the <code>Option</code> type, could result in something like: <br />
                    <code className='language-rust block'>{part1LineParsingCode}</code><br />
                    That is it!<br />
                    Some details was glossed over, but if the code is not compiling, <span className='yellow'>Read the Error Messages!</span>.
                    For example, if we where to remove the <code>mut</code> keyword, the compiler (borrow checker) would tell us what to do: <br />
                    <code className='language-rust block'>{compilerMutCode}</code>
                </p>
                <h2>-- Part 2: Written Too</h2>
                <p className='day'>
                    Part 2 is similar to part 1, and I think we can keep the general structure and just swap out the <code>extract_number</code>,
                    with a more general <code>extract_number_including_written</code>. 
                    This time, we cannot iterate over the characters since the written numbers may take up multiple. 
                    Instead, one more brute force approach is to itererativly take away the first letter and check if the resulting str starts 
                    with something which can be parsed to a digit. E.g.:<br />
                    <code className='language-rust'>(0..line.len()).map(|i| &lines[i..])</code> <br />
                    Then we can take this remaining line and check if it starts with a writting digit.
                    And if so, return the coresponding character, and if not so, simply return the next character.
                    This takes us back to our old situation where we can use a <code>filter_map</code> while trying to
                    convert the character to an <code>u32</code>. <br />
                    <code className='language-rust'>{part2ExtractCode}</code> <br />
                    <a href='https://github.com/AllaVinner/aoc-2023/blob/main/wasm-src/src/days/day01.rs'>Source Code</a>
                    <br />
                    <br />
                    <br />
                    <br />
                </p>
            </div>
        </>
    )
}

export default Day01
