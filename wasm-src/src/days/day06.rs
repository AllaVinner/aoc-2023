use nom::{
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, line_ending,
    },
    branch::alt,
    multi::{separated_list1, many1_count},
    sequence::{preceded, separated_pair},
    IResult,
    combinator,
    character::complete::u128 as str_as_128
};


fn parse_input_part1(input: &str) -> IResult<&str, (Vec<u128>, Vec<u128>)> {
    let (input, s) = preceded(
        tag("Time:"),
        many1_count(tag(" "))
    )(input)?;
    let (input, times) = separated_list1(
        many1_count(alt((tag(" "), tag("\t")))),
        str_as_128
    )(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, s) = preceded(
        tag("Distance:"),
        many1_count(tag(" "))
    )(input)?;
    let (input, distances) = separated_list1(
        many1_count(alt((tag(" "), tag("\t")))),
        str_as_128
    )(input)?;
    return Ok((input, (times, distances)));
}


pub fn part1(input: &str) -> Result<String, String> {
    let (_, (times, distances)) = match parse_input_part1(input) {
        Err(e) => return Err("Could not parse Input.".to_string()),
        Ok(v) => v
    };
    let count: u128 = times.into_iter().zip(distances.into_iter()).map(|(t, d)| {
        (0..(t+1)).filter(|k| k*(t-k) > d).count() as u128
    }).product::<u128>();
    return Ok(count.to_string());
}



fn parse_input_part2(input: &str) -> IResult<&str, (u128, u128)> {
    let (input, s) = preceded(
        tag("Time:"),
        many1_count(tag(" "))
    )(input)?;
    let (input, times) = separated_list1(
        many1_count(alt((tag(" "), tag("\t")))),
        digit1
    )(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, s) = preceded(
        tag("Distance:"),
        many1_count(tag(" "))
    )(input)?;
    let (input, distances) = separated_list1(
        many1_count(alt((tag(" "), tag("\t")))),
        digit1
    )(input)?;
    
    return Ok((input, (
        times.into_iter().fold("".to_string(), |a, n| a + n).parse::<u128>().unwrap(),
        distances.into_iter().fold("".to_string(), |a, n| a + n).parse::<u128>().unwrap()
    )));
}


pub fn part2(input: &str) -> Result<String, String> {
    let (_, (time, distance)) = match parse_input_part2(input) {
        Err(e) => return Err("Could not parse Input.".to_string()),
        Ok(v) => v
    };
    let count = (0..(time+1)).filter(|k| k*(time-k) > distance).count() as u128;
    return Ok(count.to_string());
}

