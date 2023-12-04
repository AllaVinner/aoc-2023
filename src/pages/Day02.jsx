import { useEffect, useState } from 'react'
import '../App.css'
import "../assets/prism.css"
import Prism from "../assets/prism.js"
import * as wasm from "../../wasm-src/pkg/wasm_src.js";
import AoCInput from '../components/AoCInput.jsx';


const mainFunctionCode =`\
pub fn part1(input: &str) -> String {
    let max_collection = Collection{ red: 12, green: 13, blue: 14};
    return input 
        .lines()
        .map(|line| parse_game(line))
        .filter(|game| game.is_compatible(&max_collection))
        .fold(0, |sum, game| sum + game.id)
        .to_string();
}
`

const structDeclaration = `\
struct Collection {
    red: u32,
    green: u32,
    blue: u32
}
`

const gameStructCode = `\
struct Game {
    id: u32,
    collections: Vec<Collection>
}

impl Game {
    fn is_compatible(&self, other: &Collection) -> bool {
        return self.collections.iter().all(|my| {
            return my.red <= other.red && 
            my.green <= other.green &&
            my.blue <= other.blue       
        });
    }
}
`

const part2Code =`\
pub fn part2(input: &str) -> String {
    return input 
        .lines()
        .map(|line| parse_game(line))
        .map(|game| game.collections.into_iter().reduce(|min_set, current_set| Collection{
            red: cmp::max(min_set.red, current_set.red),
            green: cmp::max(min_set.green, current_set.green),
            blue: cmp::max(min_set.blue, current_set.blue)
        }).expect("Atleast one game."))
        .map(|min_set| min_set.red * min_set.green * min_set.blue)
        .sum::<u32>()
        .to_string();
}
`

function Day02() {
    const [inputContent, setInputContent] = useState("");
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

    useEffect(() => {
        if (Number(inputContent)) {
            try{
                let res = wasm.err_fun(Number(inputContent))
                console.log("Response was ", res)
            } catch(error){
                console.log(error)
            }
            
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
                    Let's start by creating a custom type <code>Collection</code>, which in Rust is declared using the
                    <code>struct</code> key word. This type will work as an inventory of how many red, green and blue cubes there are.<br />
                    <code className='language-rust block'>{structDeclaration}</code><br /> 
                </p>
                WHat we then can do, is to create a collection with the limiting number of cubes, parse each game we have been given,
                filter out the games that were not compatible with the limiting collection, and finaly add up the id's of the 
                remaining games. Look through the <a href='https://doc.rust-lang.org/std/primitive.str.html'>str</a>, and <a href='https://doc.rust-lang.org/std/iter/trait.Iterator.html'>iterator</a> documentation to discover the functions.<br />
                <code className='language-rust'>{mainFunctionCode}</code> <br />
                We can then create the <code>Game</code> struct, and add the <code>is_compatible</code> function to it.
                <code className='language-rust'>{gameStructCode}</code> <br />
                <h2>-- Part 2: Maxout</h2>
                Now we don't even have to compare the games with a limiting collection, we can simply take the max of each color in each game, 
                and go from there. <br />
                <code className='language-rust'>{part2Code}</code> <br />               
                
            </div>
        </>
    )
}

export default Day02
