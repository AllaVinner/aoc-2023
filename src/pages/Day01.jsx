import { useEffect, useState } from 'react'
import '../App.css'
import TextFileUpload from '../components/TextFileUploader'

import * as wasm from "../../wasm-src/pkg/wasm_src.js";
import AoCInput from '../components/AoCInput.jsx';

function Day01() {
    const [inputContent, setInputContent] = useState("");
    const [answere, setAnswere] = useState("");
    
    let s = `\
How to start learning web development?
fn main() {
   let hello = vec!["sdf"df]
} 
`   
    useEffect(() => {
        try {
            let new_ans = wasm.day01(inputContent);
            console.log(new_ans)
        } catch (error) {
            setAnswere("<Invalid Input>")
        }
    }, [inputContent])
    return (
        <>
            <div id={'day01'}>
                <h1>
                    Day 01
                </h1>
                <div>------------</div>
                <br />
                <AoCInput inputContent={inputContent} setInputContent={setInputContent}/>
                <div>
                    Todays answere was: {answere}
                </div>
                <div>
                    Hello
                    <y>From</y>
                    <br />
                    <code>
                        {s}
                    </code>
                </div>
            </div>
        </>
    )
}

export default Day01
