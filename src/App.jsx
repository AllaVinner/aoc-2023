import { useState } from 'react'
import './App.css'
import './index.css'
import Sidebar from "./components/Sidebar"
import Header from "./components/Header"

import Day01 from "./pages/Day01"
import Day02 from "./pages/Day02"
import Day03 from "./pages/Day03"
import Day04 from "./pages/Day04"


function Dummy({text}) {
  return <div>Dummy {text}</div>
}


function App() {
  let pages = [
    { title: '01', content: <Day01/> },
    { title: '02', content: <Day02 /> },
    { title: '03', content: <Day03 /> },
    { title: '04', content: <Day04 /> }
  ]
  const [selectedPage, selectPage] = useState(pages[0].title)

  return (
    <>
      <div id={'dashoard'}>
        <div id={"header"} className={'green-glow'}>
          <Header />
        </div>
        <div id={'sidebar'}>
          <Sidebar pageTitles={pages.map((v) => v.title)} selectedPage={selectedPage} selectPage={selectPage}/>
        </div>
        <div id={'content'}>
          {
            pages.find((p) => p.title == selectedPage).content
          }
        </div>
      </div>
    </>
  )
}

export default App
