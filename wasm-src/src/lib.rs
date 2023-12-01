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


fn process_part1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
        
    result.to_string()
}


#[wasm_bindgen(js_name=day01)]
pub fn day01(data: String) -> Vec<String> {
    if data == "" {
        return vec![];
    }
    return vec![
        days::day01::part1(&data),
        days::day01::part2(&data)
    ];
}

