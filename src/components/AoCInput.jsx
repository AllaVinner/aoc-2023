import { useState, useEffect } from 'react'
import TextFileUpload from './TextFileUploader'
import '../App.css'

function AoCInput({ inputContent, setInputContent }) {
    const [fileContent, setFileContent] = useState("");
    useEffect(() => {
        setInputContent(fileContent)
    }, [fileContent])
    return (
        <>
            <div className='aoc-input'>
                Input text or upload a file:&nbsp;
                <TextFileUpload setText={setFileContent} /> 
                <br></br>
                <textarea name="Text1" cols="40" rows="5" value={inputContent} onChange={(e) => setInputContent(e.target.value)}></textarea>
            </div>
        </>
    )
}

export default AoCInput
