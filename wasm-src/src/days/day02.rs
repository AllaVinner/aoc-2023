use std::cmp;

struct Collection {
    red: u32,
    green: u32,
    blue: u32
}

struct Game {
    id: u32,
    collections: Vec<Collection>
}

impl Game {
    fn is_compatible(&self, other: &Collection) -> bool {
        return self.collections.iter().all(|my| {
            return my.red <= other.red && 
            my.green <= other.green &&
            my.blue <= other.blue       
        });
    }
}


fn parse_game(line: &str) -> Game {
    let (id_str, collections_str) = line.strip_prefix("Game ").expect("Input must start with 'Game'.").split_once(":").expect("':' shoud exist.");
    let id: u32 = id_str.parse().expect("U32 string.");
    let subsets: Vec<Collection> = collections_str.split(";").map(|collection_str| {
        collection_str.split(",").fold(Collection{red: 0, green: 0, blue: 0}, |mut collection, set| {
            if set.trim_start() == "" {
                return collection;
            }
            let (num_str, color_str) = set.trim_start().trim_end().split_once(" ").expect("Should be right format.");
            let num: u32 = num_str.parse().expect("U32 string.");
            if color_str == "red" {
                collection.red = num;
            } else if color_str == "green" {
                collection.green = num;
            } else if color_str == "blue"{
                collection.blue = num;
            } else {
                panic!("Shoulnd not get here");
            }
            return collection;
        })
    })
    .collect();
    return Game{
        id: id,
        collections: subsets
    }
}

pub fn part1(input: &str) -> String {
    let max_collection = Collection{ red: 12, green: 13, blue: 14};
    return input 
        .lines()
        .map(|line| parse_game(line))
        .filter(|game| game.is_compatible(&max_collection))
        .fold(0, |sum, game| sum + game.id)
        .to_string();
}


pub fn part2(input: &str) -> String {
    return input 
        .lines()
        .map(|line| parse_game(line))
        .map(|game| game.collections.into_iter().reduce(|min_set, current_set| Collection{
            red: cmp::max(min_set.red, current_set.red),
            green: cmp::max(min_set.green, current_set.green),
            blue: cmp::max(min_set.blue, current_set.blue)
        }).expect("Atleast one game.")) 
        .map(|min_set| min_set.red * min_set.green * min_set.blue) 
        .sum::<u32>()
        .to_string();
}
