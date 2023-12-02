use std::{cmp::max, collections::HashMap};

use anyhow::anyhow;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "game.pest"]
struct GameParser;

trait Limit {
    fn limit(&self, rgb: (u32, u32, u32)) -> bool;
}

trait Fewest {
    fn fewest(&self) -> (u32, u32, u32);
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl TryFrom<Pair<'_, Rule>> for Colour {
    type Error = anyhow::Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        if Rule::colour == value.as_rule() {
            match value.as_str() {
                "red" => Ok(Self::Red),
                "blue" => Ok(Self::Blue),
                "green" => Ok(Self::Green),
                _ => Err(anyhow!("Not a colour")),
            }
        } else {
            Err(anyhow!("Not a colour Pair"))
        }
    }
}

#[derive(Debug)]
struct Hand {
    cubes: HashMap<Colour, u32>,
}

impl Fewest for Hand {
    fn fewest(&self) -> (u32, u32, u32) {
        (
            self.cubes.get(&Colour::Red).cloned().unwrap_or_default(),
            self.cubes.get(&Colour::Green).cloned().unwrap_or_default(),
            self.cubes.get(&Colour::Blue).cloned().unwrap_or_default(),
        )
    }
}

impl Limit for Hand {
    fn limit(&self, rgb: (u32, u32, u32)) -> bool {
        let (r, g, b) = rgb;
        (self.cubes.get(&Colour::Red).cloned().unwrap_or_default() <= r)
            && (self.cubes.get(&Colour::Green).cloned().unwrap_or_default() <= g)
            && (self.cubes.get(&Colour::Blue).cloned().unwrap_or_default() <= b)
    }
}

fn try_cubes(value: Pair<'_, Rule>) -> anyhow::Result<(Colour, u32)> {
    if Rule::cubes == value.as_rule() {
        let mut inners = value.into_inner();
        let count: u32 = inners.next().unwrap().as_str().trim().parse()?;
        let colour: Colour = inners.next().unwrap().try_into()?;
        Ok((colour, count))
    } else {
        Err(anyhow::anyhow!("Not a cubes pair"))
    }
}

impl TryFrom<Pair<'_, Rule>> for Hand {
    type Error = anyhow::Error;

    fn try_from(value: Pair<Rule>) -> Result<Self, Self::Error> {
        if Rule::hand == value.as_rule() {
            let mut cubes = HashMap::new();
            for cubes_pair in value.into_inner() {
                let (colour, count) = try_cubes(cubes_pair)?;
                cubes.insert(colour, count);
            }
            Ok(Hand { cubes })
        } else {
            Err(anyhow!("Not a hand pair"))
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Fewest for Game {
    fn fewest(&self) -> (u32, u32, u32) {
        self.hands.iter().fold((0, 0, 0), |(r, g, b), h| {
            let (rh, gh, bh) = h.fewest();
            (max(rh, r), max(gh, g), max(bh, b))
        })
    }
}

impl Limit for Game {
    fn limit(&self, rgb: (u32, u32, u32)) -> bool {
        self.hands.iter().all(|h| h.limit(rgb))
    }
}

impl TryFrom<Pair<'_, Rule>> for Game {
    type Error = anyhow::Error;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut inners = value.into_inner();
        let id: u32 = inners.next().unwrap().as_str().trim().parse()?;
        let hands: Vec<Hand> = inners
            .next()
            .unwrap()
            .into_inner()
            .map(Hand::try_from)
            .collect::<Result<Vec<Hand>, anyhow::Error>>()?;
        Ok(Game { id, hands })
    }
}

#[derive(Debug)]
struct Games(Vec<Game>);

impl Games {
    fn get_power(&self) -> u32 {
        self.0
            .iter()
            .map(|g| {
                let (r, g, b) = g.fewest();
                r * g * b
            })
            .sum()
    }

    fn limit_by(&self, rgb: (u32, u32, u32)) -> Vec<u32> {
        self.0
            .iter()
            .filter_map(|gm| if gm.limit(rgb) { Some(gm.id) } else { None })
            .collect()
    }
}

impl TryFrom<&str> for Games {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let parser = GameParser::parse(Rule::games, input)?.next().unwrap();
        let games: Vec<Game> = parser
            .into_inner()
            .filter(|p| p.as_rule() == Rule::game)
            .map(Game::try_from)
            .into_iter()
            .collect::<Result<Vec<Game>, anyhow::Error>>()?;
        Ok(Games(games))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a_test() {
        let text = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let games: Games = text.try_into().unwrap();
        assert_eq!(8u32, games.limit_by((12, 13, 14)).into_iter().sum())
    }

    #[test]
    fn part_a() {
        let games: Games = include_str!("../data/day_02.data").try_into().unwrap();
        assert_eq!(2169u32, games.limit_by((12, 13, 14)).into_iter().sum())
    }

    #[test]
    fn part_b_test() {
        let text = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let games: Games = text.try_into().unwrap();
        assert_eq!(2286u32, games.get_power())
    }

    #[test]
    fn part_b() {
        let games: Games = include_str!("../data/day_02.data").try_into().unwrap();
        assert_eq!(60948u32, games.get_power())
    }
}
