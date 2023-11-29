import { useState } from 'react'
import '../App.css'

function Header() {
    const [count, setCount] = useState(0)

    return (
        <>
            <div className={"header"}>
                Advent of Rust
            </div>
        </>
    )
}

export default Header
