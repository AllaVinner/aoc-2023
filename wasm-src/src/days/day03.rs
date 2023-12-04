use std::collections::{HashMap, HashSet};

struct NumberIterator<'a> {
    s: &'a str,
    cursor: usize,
}

impl<'a> Iterator for NumberIterator<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let start = self.cursor + self.s[self.cursor..].find(|c: char| c.is_ascii_digit())?;
        let num_to_next = self.s[start..].find(|c: char| !c.is_ascii_digit()).unwrap_or(self.s.len()-start);
        let end = start+num_to_next;
        self.cursor = end;
        return Some((start, end));
    }
}

fn iter_num<'a>(s: &'a str) -> NumberIterator<'a> {
    return NumberIterator{s, cursor: 0};
}

fn symbole_on_boundary<'a>(lines: &Vec<&'a str>, row: usize, start: usize, end: usize) -> bool {
    if row > 0 && lines[row-1][start..end].contains(|c: char| c != '.') {
        return true;
    }
    if row < lines.len()-1 && lines[row+1][start..end].contains(|c: char| c != '.') {
        return true;
    }
    if row > 0 && start > 0 && &lines[row-1][start-1..start] != "." {
        return true;
    }
    if row > 0 && end < lines[row].len()-1 && &lines[row-1][end..end+1] != "." {
        return true;
    }
    if row < lines.len() - 1 && start > 0 && &lines[row+1][start-1..start] != "." {
        return true;
    }
    if row < lines.len() - 1 && end < lines[row].len()-1 && &lines[row+1][end..end+1] != "." {
        return true;
    }
    if start > 0 && &lines[row][start-1..start] != "." {
        return true;
    }
    if end < lines[row].len()-1 && &lines[row][end..end+1] != "." {
        return true;
    }
    return false;
}


pub fn part1(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;
    for (row, line) in lines.iter().enumerate() {
        for (start, end) in iter_num(line) {
            let num = line[start..end].parse::<u32>().unwrap();
            if symbole_on_boundary(&lines, row, start, end) {
                sum += num;
            }
        }
    }
    return sum.to_string();
}

fn calculate_power(coord_to_num: &HashMap<(usize, usize), u32>, row: usize, col: usize) -> u32 {
    let mut parts = HashSet::new();
    if row > 0 && col > 0{
        match coord_to_num.get(&(row-1, col-1)) {
            Some(d) => parts.insert(*d),
            None => true
        };
    }
    if row > 0{
        match coord_to_num.get(&(row-1, col)) {
            Some(d) => parts.insert(*d),
            None => true
        };
        match coord_to_num.get(&(row-1, col+1)) {
            Some(d) => parts.insert(*d),
            None => true
        };
    }
    if col > 0{
        match coord_to_num.get(&(row, col-1)) {
            Some(d) => parts.insert(*d),
            None => true
        };
        match coord_to_num.get(&(row+1, col-1)) {
            Some(d) => parts.insert(*d),
            None => true
        };
    }
    match coord_to_num.get(&(row+1, col)) {
            Some(d) => parts.insert(*d),
            None => true
    };
    match coord_to_num.get(&(row, col+1)) {
            Some(d) => parts.insert(*d),
            None => true
    };
    match coord_to_num.get(&(row+1, col+1)) {
            Some(d) => parts.insert(*d),
            None => true
    };

    if parts.len() == 2 {
        return parts.iter().product::<u32>();
    } else {
            return 0;
    }
}


pub fn part2(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();
    let mut coord_to_num = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
         for (start, end) in iter_num(line) {
            let num = line[start..end].parse::<u32>().unwrap();
            for i in start..end {
                coord_to_num.insert((row, i), num);
            }
        }
    }
    let mut sum: u32 = 0;
    for (row, line) in lines.iter().enumerate() {
         for (col, _) in line.match_indices("*") {
            sum += calculate_power(&coord_to_num, row, col);
        }
    }
    return sum.to_string();
}