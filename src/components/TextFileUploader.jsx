import { useState } from 'react'
import '../App.css'


function TextFileUpload({ setText }) {
    
    function handleFileUpload(e) {
        var fileReader = new FileReader();
        fileReader.addEventListener(
            "load",
            () => {
              // this will then display a text file
              setText(fileReader.result);
            },
            false,
          );
        
          if (e.target.files) {
            fileReader.readAsText(e.target.files[0], "UTF-8");
          }
    }
    return (
        <>
            <input type="file" id="myFile" multiple size="50" onChange={handleFileUpload}></input>
        </>
    )
}

export default TextFileUpload
