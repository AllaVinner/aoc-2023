import { useState } from 'react'
import './App.css'
import './index.css'
import Sidebar from "./components/Sidebar"
import Header from "./components/Header"

import Day01 from "./pages/Day01";
import Day02 from "./pages/Day02";
import Day03 from "./pages/Day03";
import Day04 from "./pages/Day04";
import Day05 from "./pages/Day05";
import Day06 from "./pages/Day06";
import Day07 from "./pages/Day07";
import Day08 from "./pages/Day08";
import Day09 from "./pages/Day09";
import Day10 from "./pages/Day10";
import Day404 from "./pages/Day404";


function Dummy({text}) {
  return <div>Dummy {text}</div>
}


function App() {
  let pages = [
    { title: ' 1', content: <Day01/> },
    { title: ' 2', content: <Day02 /> },
    { title: ' 3', content: <Day03 /> },
    { title: ' 4', content: <Day04 /> },
    { title: ' 5', content: <Day05 /> },
    { title: ' 6', content: <Day06 /> },
    { title: ' 7', content: <Day07 /> },
    { title: ' 8', content: <Day08 /> },
    { title: ' 9', content: <Day404 /> },
    { title: '11', content: <Day404 /> },
    { title: '12', content: <Day404 /> },
    { title: '13', content: <Day404 /> },
    { title: '14', content: <Day404 /> },
    { title: '15', content: <Day404 /> },
    { title: '16', content: <Day404 /> },
    { title: '17', content: <Day404 /> },
    { title: '18', content: <Day404 /> },
    { title: '19', content: <Day404 /> },
    { title: '20', content: <Day404 /> },
    { title: '21', content: <Day404 /> },
    { title: '22', content: <Day404 /> },
    { title: '23', content: <Day404 /> },
    { title: '24', content: <Day404 /> },
    { title: '25', content: <Day404 /> }
  ]
  const [selectedPage, selectPage] = useState(pages[3 -1].title)

  return (
    <>
      <div id={'dashoard'}>
        <div id={"header"}>
          <Header />
        </div>
        <div id={'sidebar'}>
          <Sidebar pageTitles={pages.map((v) => v.title)} selectedPage={selectedPage} selectPage={selectPage}/>
        </div>
        <div id={'content'}>
          {
            pages.find((p) => p.title == selectedPage) ? pages.find((p) => p.title == selectedPage).content : <Day404 />
          }
        </div>
      </div>
    </>
  )
}

export default App
