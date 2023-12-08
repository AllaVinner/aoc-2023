use std::collections::HashMap;
use std::cmp::Ordering;


#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
    rank: u32
}


fn find_rank(cards: &Vec<u32>) -> Result<u32, String> {
    let mut counter = HashMap::new();
    for card in cards.iter() {
        if let Some(count) = counter.get_mut(card) {
            *count += 1;
        } else {
            counter.insert(card, 1);
        }
    }
    let mut values: Vec<u32> = counter.into_values().collect();
    return if values.contains(&5) {
        Ok(7)
    } else if values.contains(&4) {
        Ok(6)
    } else if values.contains(&3) && values.contains(&2) {
        Ok(5)
    } else if values.contains(&3) {
        Ok(4)
    } else if values.iter().filter(|v| *v == &2).count() == 2 {
        Ok(3)
    } else if values.contains(&2) {
        Ok(2)
    } else if values.iter().filter(|v| *v == &1).count() == values.len() {
        Ok(1)
    } else {
        Err("Got hand that did not fit one of the asserted options.".to_string())
    };
}


fn parse_hand(hand_str:  &str) -> Result<Hand, String> {
    let content = hand_str.chars().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                d => d.to_digit(10).expect("Digit to be in AKQJT or digit.")
            }
        }).collect::<Vec<u32>>();
    let rank = find_rank(&content)?;
    
    Ok(Hand {
        cards: content,
        rank: rank
    })
}


fn parse_input(input: &str) -> Result<Vec<(Hand, u32)>, String> {
    input.lines().map(|line| {
        let (hand_str, bet_str) = line.trim().split_once(" ").expect("Could not split string.");
        let hand = parse_hand(hand_str)?;
        let bet = bet_str.trim().parse::<u32>().expect("Bet could not be parsed to integer.");
        Ok((hand, bet))
    }).collect()
}


fn first_higest(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Ordering {
    for (l, r) in lhs.iter().zip(rhs.iter()) {
        if l > r {
            return Ordering::Greater;
        } else if r > l {
            return Ordering::Less;
        }
    }
    panic!("Both identical")   
}


pub fn part1(input: &str) -> Result<String, String> {
    let mut hands = parse_input(input.trim())?;
    hands.sort_by(|l, r| {
        if l.0.rank > r.0.rank {
            Ordering::Greater
        } else if l.0.rank < r.0.rank{
            Ordering::Less
        } else {
            first_higest(&l.0.cards, &r.0.cards)
        }
    });
    return Ok(hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32+1)*hand.1)
        .sum::<u32>()
        .to_string());
}

/*
Part 2
*/

fn find_rank2(cards: &Vec<u32>) -> Result<u32, String> {
    let mut counter = HashMap::new();
    for card in cards.iter() {
        if let Some(count) = counter.get_mut(card) {
            *count += 1;
        } else {
            counter.insert(card, 1);
        }
    }
    let num_jokers = counter.remove(&0).unwrap_or(0);
    let mut values: Vec<u32> = counter.into_values().collect();

    if values.len() <= 1 {
        return Ok(7);
    }
    values.sort_by(|a, b| b.cmp(a));
    return if values[0]+num_jokers >= 5 {
        Ok(7)
    } else if values[0]+num_jokers >= 4 {
        Ok(6)
    } else if 
        num_jokers >= 3 ||  
        ( num_jokers == 2 && values[0] >= 2 ) ||
        ( num_jokers == 1 && (values[0] == 3 ||  values[1] == 2) ) ||
        (values[0] >= 3 &&  values[1] >= 2)
    {
        Ok(5)
    } else if values[0]+num_jokers >= 3 {
        Ok(4)
    } else if        
        (num_jokers >= 2) ||
        (num_jokers == 1 && values[0] >= 2) ||
        (values[0] >= 2 &&  values[1] >= 2) 
    {
        Ok(3)
    } else if values[0]+num_jokers >= 2 {
        Ok(2)
    } else {
        Ok(1)
    };
}


fn parse_hand2(hand_str:  &str) -> Result<Hand, String> {
    let content = hand_str.chars().map(|c| { 
            let val = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 0,
                'T' => 10,
                d => match d.to_digit(10) {
                    Some(v) => v,
                    None => return Err("Digit to be in AKQJT or digit.".to_string())
                }
            };
            Ok(val)
        }).collect::<Result<Vec<u32>, String>>()?;
    let rank = find_rank2(&content)?;
    
    Ok(
        Hand {
            cards: content,
            rank: rank
    })
}


fn parse_input2(input: &str) -> Result<Vec<(Hand, u32)>, String> {
    input.lines().map(|line| {
        let (hand_str, bet_str) = match line.trim().split_once(" ") {
            Some(parsed) => parsed,
            None => return Err(format!("Could not split line {line} in two."))
        };
        let hand = match parse_hand2(hand_str) {
            Ok(v) => v,
            Err(msg) => return Err(format!("Could not parse hand in line '{line}', got err: {msg}."))
        };
        let bet = match bet_str.trim().parse::<u32>() {
            Ok(d) => d,
            Err(_) => return Err(format!("Could not parse bet '{bet_str}' to u32 in line '{line}'."))
        };
        Ok((hand, bet))
    }).collect()
}


pub fn part2(input: &str) -> Result<String, String> {
    let mut hands = parse_input2(input.trim())?;
    hands.sort_by(|l, r| {
        if l.0.rank > r.0.rank {
            Ordering::Greater
        } else if l.0.rank < r.0.rank{
            Ordering::Less
        } else {
            first_higest(&l.0.cards, &r.0.cards)
        }
    });
    return Ok(hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32+1)*hand.1)
        .sum::<u32>()
        .to_string());
}