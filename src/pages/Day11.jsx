import { useEffect, useState } from 'react'
import '../App.css'
import "../assets/prism.css"
import Prism from "../assets/prism.js"
import * as wasm from "../../wasm-src/pkg/wasm_src.js";
import AoCInput from '../components/AoCInput.jsx';

const cellEnum = `\
#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    EMPTY,
    GALAXY
}
`

const galaxyVec = `\
let galaxies: Vec<[usize; 2]> = sky
    .indexed_iter()
    .filter(|((r, c), v)| v == &&Cell::GALAXY)
    .map(|((r, c), v)| [r, c])
    .collect();
`


const emptyRowCol = `\
// sky: Array2<Cell>
let empty_rows: Vec<usize> = sky
    .rows()
    .into_iter()
    .enumerate()
    .filter(|(i, r)| r.iter().all(|c| c == &Cell::EMPTY))
    .map(|(i, r)| i)
    .collect();
let empty_columns: Vec<usize> = sky
    .columns()
    .into_iter()
    .enumerate()
    .filter(|(i, r)| r.iter().all(|c| c == &Cell::EMPTY))
    .map(|(i, r)| i)
    .collect();
`

const distanceCount = `\
let mut total = 0;
for (g1i, [g1r, g1c]) in galaxies.iter().take(galaxies.len()-1).enumerate() {
    for (g2i, [g2r, g2c]) in galaxies.iter().enumerate().skip(g1i+1) {
        let extra_rows = empty_rows.iter().filter(|row| val_in_range(row, g2r, g1r)).count();
        let extra_cols = empty_columns.iter().filter(|col| val_in_range(col, g2c, g1c)).count();
        let dist = abs(g2r, g1r) + abs(g2c, g1c) + extra_rows + extra_cols;
        total += dist;
    }
}
`
const bigDistanceCount = `\
let mut total = 0;
for (g1i, [g1r, g1c]) in galaxies.iter().take(galaxies.len()-1).enumerate() {
    for (g2i, [g2r, g2c]) in galaxies.iter().enumerate().skip(g1i+1) {
        let extra_rows = empty_rows.iter().filter(|row| val_in_range(row, g2r, g1r)).count();
        let extra_cols = empty_columns.iter().filter(|col| val_in_range(col, g2c, g1c)).count();
        let dist = abs(g2r, g1r) + abs(g2c, g1c) + (100000-1)*extra_rows + (100000-1)*extra_cols;
        total += dist;
    }
}
`

function Day11() {
    const [inputContent, setInputContent] = useState("");
    const [part1Ans, setPart1Ans] = useState("");
    const [part2Ans, setPart2Ans] = useState("");

    useEffect(() => {
        Prism.highlightAll();
    }, []);

    useEffect(() => {
        if (inputContent !== "") {
            try {
                let result = wasm.day11_part1(inputContent);
                console.log("Result", result);
                setPart1Ans(result)
            } catch (error) {
                console.log("Error: ", error);
                setPart1Ans("<Invalid Input>")
            }
            try {
                let result = wasm.day11_part2(inputContent);
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
            <div id={'day11'}>
                <h1>
                    Day 11: Cosmic Expansion
                </h1>
                <div>----------------------------------------------------</div>
                <AoCInput
                    inputContent={inputContent}
                    setInputContent={setInputContent}
                />
                <div>
                    Part 1 Answer: {part1Ans}<br />
                    Part 2 Answer: {part2Ans}<br />
                    <a href="https://adventofcode.com/2023/day/11">Puzzle</a>
                    {' '}
                    <a href='https://github.com/AllaVinner/aoc-2023/blob/main/wasm-src/src/days/day11.rs'>solution</a>
                </div> <br />
                <h2>Part 1: Slow Expansion</h2>
                <p>
                    Today we are dealing with a matrix, might be time to introduce&nbsp;
                    <a href="https://docs.rs/ndarray/latest/ndarray/">ndarray</a>.&nbsp;
                    This is a create that models multi-dimensional arrays in Rust and has many similarities with Numpy in Python.
                    The parsing is quite straight forward with the&nbsp;
                    <a href="https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#method.from_shape_vec"><code>from_shape_vec</code></a>&nbsp;
                    function, with a type that can hold our input values (empty and galaxy).
                </p>
                <code className='language-rust'>
                    {cellEnum}
                </code>
                <p> 
                    The tricky part here is to not think to much. We do not need to simulate the expansion.
                    Instead, we can calculated the original distance, and the add the expanded part.
                    The distance described is the so called&nbsp;
                    <a href="https://en.wikipedia.org/wiki/Taxicab_geometry">Manhattan distance</a>&nbsp;
                    and can be calculated as <code>|x2 - x1| + |y2 - y2|</code>.
                    To acount for the expansion, we must add the rows and columns added inbetween the points due to the expanssion.
                    So lets collect the empty rows and columns:
                </p>
                <code className='language-rust'>
                    {emptyRowCol}
                </code>
                <p>
                    To then be able to iterate over the pair of galaxies, we would need to collect them too.
                </p>
                <code className='language-rust'>
                    {galaxyVec}
                </code>
                <p>
                    Now we can loop over the indices of all galaxy pairs, calculated their Manhattan distance and add the 
                    expansion adjustments.
                </p>
                <code className='language-rust'>
                    {distanceCount}
                </code>
                <h2> Part 2: The Big Expansion</h2>
                So much much more expansion now... <br />
                Good that we didn't simulate the expansion. Now we can add a correction factor to each empty row and
                column we find and that should be it. <br />
                <code className='language-rust'>
                    {bigDistanceCount}
                </code>
            </div>
        </>
    )
}

export default Day11
