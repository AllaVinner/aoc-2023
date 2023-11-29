import { useState } from 'react'
import '../App.css'

function Day01() {
    const [count, setCount] = useState(0)
    let s = `\
How to start learning web development?
fn main() {
   let hello = vec!["sdf"df]
}
`
    return (
        <>
            <div id={'day01'}>
                <h1>
                    Day 01
                </h1>
                <div>------------</div>
                <br />
                <br />
                <input type="file" id="myFile" multiple size="50" onChange={(e) => console.log(e)}></input>
                <div>
                    Hello
                    <y>From</y>
                    <br />
                    <code>
                        {s}
                    </code>
                </div>
                <textarea name="Text1" cols="40" rows="5"></textarea>
            </div>
        </>
    )
}

export default Day01
