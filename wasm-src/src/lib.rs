mod utils;
mod days;
use wasm_bindgen::prelude::*;


macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen(js_name=day01)]
pub fn day01(data: String) -> Vec<String> {
    return vec![
        days::day01::part1(&data),
        days::day01::part2(&data)
    ];
}


#[wasm_bindgen(js_name=day02)]
pub fn day02(data: String) -> Vec<String> {
    return vec![
        days::day02::part1(&data),
        days::day02::part2(&data)
    ];
}


#[wasm_bindgen(js_name=day03_part1)]
pub fn day03_part1(input: &str) -> String {
    return days::day03::part1(&input);
}

#[wasm_bindgen(js_name=day03_part2)]
pub fn day03_part2(input: &str) -> String {
    return days::day03::part2(&input);
}

#[wasm_bindgen(js_name=day04_part1)]
pub fn day04_part1(input: &str) -> String {
    return days::day04::part1(&input);
}

#[wasm_bindgen(js_name=day04_part2)]
pub fn day04_part2(input: &str) -> Result<String, String> {
    return days::day04::part2(&input);
}

#[wasm_bindgen(js_name=day05_part1)]
pub fn day05_part1(input: &str) -> String {
    return days::day05::part1(&input);
}

#[wasm_bindgen(js_name=day05_part2)]
pub fn day05_part2(input: &str) -> String {
    return days::day05::part2(&input);
}

#[wasm_bindgen(js_name=day06_part1)]
pub fn day06_part1(input: &str) -> Result<String, String> {
    return days::day06::part1(&input);
}

#[wasm_bindgen(js_name=day06_part2)]
pub fn day06_part2(input: &str) -> Result<String, String> {
    return days::day06::part2(&input);
}

#[wasm_bindgen(js_name=day07_part1)]
pub fn day07_part1(input: &str) -> Result<String, String> {
    return days::day07::part1(&input);
}

#[wasm_bindgen(js_name=day07_part2)]
pub fn day07_part2(input: &str) -> Result<String, String> {
    if input == "ABA" {
        return Err("THIS WAS ALMOST ABBA.".to_string());
    }
    return days::day07::part2(&input);
}
