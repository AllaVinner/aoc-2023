import { useState } from 'react'
import '../App.css'

function Header() {
    const [count, setCount] = useState(0)

    return (
        <>
            <div className={"header green-glow"}>
                Advent of Rust
            </div>
            <div>
                --- Learn Rust through Advent of Code ---
            </div>
        </>
    )
}

export default Header
