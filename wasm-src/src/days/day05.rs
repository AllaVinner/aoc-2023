use std::cmp;

pub fn part1(input: &str) -> String {
    let mut lowest = None;
    let (seed_input, map_input) = input.trim().split_once("\n\n").unwrap();
    let mut seed_iter = seed_input.trim().split(" ").skip(1).map(|s| s.parse::<u64>().unwrap());
    for seed in seed_iter {
        let mut val = seed;
        let mut map_iter = map_input.split("\n\n");
        for map in map_iter {
            let mut range_iter = map.split("\n").skip(1).map(|line| line.split(" ").map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>());
            for range in range_iter {
                if range[1] <= val && val < range[1] + range[2] {
                    val = val  - range[1] + range[0];
                    break;
                }
            }
        }
        match lowest {
            None => lowest = Some(val),
            Some(l) => lowest = Some(cmp::min(l, val)),
        }
    }
    return lowest.unwrap().to_string();
}


#[derive(Debug, Copy, Clone)]
struct Range {
    start: u64,
    end: u64
}

#[derive(Debug, Copy, Clone)]
enum IR {
    LHS(Range),
    Both(Range),
    RHS(Range),
    Neither(Range)
}



fn intersect(lhs: &Range, rhs: &Range) -> [IR; 3] {
    if lhs.end <= rhs.start {
        [
            IR::LHS(*lhs), 
            IR::Neither(Range{start: lhs.end, end: rhs.start}),
            IR::RHS(*rhs)
        ]
    } else if lhs.start <= rhs.start && rhs.start <= lhs.end && lhs.end <= rhs.end {
        [
            
            IR::LHS(Range{start: lhs.start, end: rhs.start}),
            IR::Both(Range{start: rhs.start, end: lhs.end}),
            IR::RHS(Range{start: lhs.end, end: rhs.end})
        ]
    } else if rhs.start <= lhs.start && lhs.end <= rhs.end {
        [
            
            IR::RHS(Range{start: rhs.start, end: lhs.start}),
            IR::Both(Range{start: lhs.start, end: lhs.end}),
            IR::RHS(Range{start: lhs.end, end: rhs.end})
        ]
    } else if lhs.start <= rhs.start && rhs.end <= lhs.end {
        [
            
            IR::LHS(Range{start: lhs.start, end: rhs.start}),
            IR::Both(Range{start: rhs.start, end: rhs.end}),
            IR::LHS(Range{start: rhs.end, end: lhs.end})
        ]
    } else if rhs.start <= lhs.start && lhs.start <= rhs.end && rhs.end <= lhs.end {
        [
            
            IR::RHS(Range{start: rhs.start, end: lhs.start}),
            IR::Both(Range{start: lhs.start, end: rhs.end}),
            IR::LHS(Range{start: rhs.end, end: lhs.end})
        ]
    } else if rhs.end <= lhs.start {
        [
            IR::RHS(*rhs), 
            IR::Neither(Range{start: rhs.end, end: lhs.start}),
            IR::LHS(*lhs)
        ]
    } else {
        panic!("Should not get here");
    }
}



pub fn part2(input: &str) -> String {
    let (seed_input, map_input) = input.trim().split_once("\n\n").unwrap();
    println!("{:?}", seed_input);
    let mut seed_iter = seed_input.trim().split(" ").skip(1).map(|s| s.parse::<u64>().unwrap());
    let mut seed_ranges = vec![];
    let mut start = 0;
    for (i, seed) in seed_iter.enumerate() {
        if i % 2 == 0{
            start = seed;
        } else {
            seed_ranges.push(Range{start: start, end: start + seed});
        }
    }
    for map in map_input.split("\n\n") {
        let mut mapped_ranges = vec![];
        for rv in map.split("\n").skip(1).map(|line| line.split(" ").map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>()) {
            let map_src_range = Range{start: rv[1], end: rv[1]+ rv[2]};
            let map_dst_range = Range{start: rv[0], end: rv[0]+rv[2]};
            let mut passed_ranges = vec![];
            for seed_range in seed_ranges {
                let intersect_ranges = intersect(&seed_range, &map_src_range);
                for inter_range in intersect_ranges {
                    match inter_range {
                        IR::LHS(sr) => {
                            if sr.end - sr.start > 0 {
                                passed_ranges.push(sr)   
                            }
                        },
                        IR::Both(r) => {
                            if r.end - r.start > 0 {
                                mapped_ranges.push(
                                    Range{
                                        start: r.start +map_dst_range.start - map_src_range.start, 
                                        end: r.end +map_dst_range.start - map_src_range.start
                                    }
                                )   
                            }
                        },
                        _ => ()
                    }
                }
            }
            seed_ranges = passed_ranges;
        }
        seed_ranges.append(& mut mapped_ranges)
    }
    let lowest = seed_ranges.iter().map(|r| r.start).min().unwrap();
    return lowest.to_string();
}

