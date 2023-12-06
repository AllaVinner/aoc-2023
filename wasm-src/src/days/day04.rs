use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::u32 as str_u32,
    sequence::{
      tuple,
      preceded
    },
    multi::{
          many1_count,
          separated_list1
      }
  };
  use std::{collections::HashSet, iter::FromIterator, cmp};
  
  
  
  fn parse_line(input: &str) -> IResult<&str, (u32, HashSet<u32>, HashSet<u32>)> {
      let (input, _) = tag("Card")(input)?;
      let (input, _) = many1_count(tag(" "))(input)?;
      let (input, id) = str_u32(input)?;
      let (input, _) = tag(":")(input)?;
      let (input, _) = many1_count(tag(" "))(input)?;
      let (input, winners) = separated_list1(many1_count(tag(" ")), str_u32)(input)?;
      let (input, _) = many1_count(tag(" "))(input)?;
      let (input, _) = tag("|")(input)?;
      let (input, _) = many1_count(tag(" "))(input)?;
      let (input, tickets) = separated_list1(many1_count(tag(" ")), str_u32)(input)?;
      Ok((input, (id, HashSet::from_iter(winners.into_iter()), HashSet::from_iter(tickets.into_iter()))))
  }
  
  
  pub fn part1(input: &str) -> String {
      let val: u32 = input.lines().map(|line| {
          match parse_line(line) {
              Err(_) => panic!("Did not parse"),
              Ok((_, (id, winners, tickets))) => {
                  let count = winners.iter().filter(|winner| tickets.contains(winner)).count();
                  if count > 0 {
                      return 2_u32.pow(count as u32-1);
                  } else {
                      return 0;
                  }
              }
          }
      }).sum::<u32>();
      return val.to_string();
  }

  pub fn part2(input: &str) -> Result<String, String> {
    let num_cards = input.lines().count();
    let mut tracker = vec![1; num_cards];
    let mut val: u32 = 0;
    for (row, line) in input.lines().enumerate() {
        match parse_line(line) {
            Err(_) => return Err("Did not parse properly".to_string()),
            Ok((_, (id, winners, tickets))) => {
                let count = winners.iter().filter(|winner| tickets.contains(winner)).count();
                for i in 1..cmp::min(count+1, num_cards-row+2) {
                    tracker[row+i] += tracker[row];
                }
                if count > 0 {
                    val += 2_u32.pow(count as u32-1);
                }
            }
        }
    }
    return Ok(tracker.iter().sum::<u32>().to_string());
}