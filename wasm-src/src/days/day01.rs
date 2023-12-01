
pub fn part1(input: &str) -> String {
    return input
        .lines()
        .map(|line| extract_number(line))
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    return input
        .lines()
        .map(|line| extract_number_including_written(line))
        .sum::<u32>()
        .to_string()
}


fn extract_number(line: &str) -> u32 {
    let mut detector = line
        .chars()
        .filter_map(|c: char| c.to_digit(10));
    let first = detector.next().unwrap_or(0);
    let last = detector.last().unwrap_or(first);
    return first*10 + last;
}


fn extract_number_including_written(line: &str) -> u32 {
    let mut detector = (0..line.len())
        .map(|i| &line[i..])
        .map(|remaining_line| {
            return if remaining_line.starts_with("one") {
                '1'
            } else if remaining_line.starts_with("two") {
                '2'
            } else if remaining_line.starts_with("three") {
                '3'
            } else if remaining_line.starts_with("four") {
                '4'
            } else if remaining_line.starts_with("five") {
                '5'
            } else if remaining_line.starts_with("six") {
                '6'
            } else if remaining_line.starts_with("seven") {
                '7'
            } else if remaining_line.starts_with("eight") {
                '8'
            } else if remaining_line.starts_with("nine") {
                '9'
            } else {
                remaining_line.chars().next().unwrap()
            }; 
        })
        .filter_map(|c: char| c.to_digit(10));    
    let first = detector.next().unwrap_or(0);
    let last = detector.last().unwrap_or(first);
    return first*10 + last;
}
