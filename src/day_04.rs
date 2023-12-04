use std::collections::{HashSet, HashMap};

use anyhow::anyhow;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "day_04.pest"]
struct ScratcherParser;

struct Number(i32);

impl Into<i32> for Number {
    fn into(self) -> i32 {
        self.0
    }
}

impl TryFrom<Pair<'_, Rule>> for Number {
    type Error = anyhow::Error;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::number => Ok(Number(value.as_str().trim().parse()?)),
            _ => Err(anyhow!("Not a Number Pair")),
        }
    }
}

struct Numbers(Vec<Number>);

impl Into<Vec<i32>> for Numbers {
    fn into(self) -> Vec<i32> {
      self.0.iter().map(|n| n.0).collect()
    }
}

impl Into<HashSet<i32>> for Numbers {
    fn into(self) -> HashSet<i32> {
      self.0.iter().map(|n| n.0).collect()
    }
}

impl TryFrom<Pair<'_, Rule>> for Numbers {
    type Error = anyhow::Error;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::numbers => {
                let numbers = value.into_inner()
                                             .map(Number::try_from)
                                             .collect::<Result<Vec<Number>,anyhow::Error>>()?;
                
                Ok(Numbers(numbers))
            }
            _ => Err(anyhow!("Not a Numbers Pair {:?}", value)),
        }
    }
}

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    winning: HashSet<i32>,
    numbers: Vec<i32>,
}

impl Card {
    fn wins(&self) -> u32 {
        self.numbers.iter().filter(|n| self.winning.contains(n)).count() as u32
    }
}

impl TryFrom<Pair<'_, Rule>> for Card {
    type Error = anyhow::Error;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::card => {
                let mut parts = value.into_inner();
                let id_pair = parts.next().ok_or(anyhow!("No id"))?;
                let id: i32 = TryInto::<Number>::try_into(id_pair)?.into();
                let winning_pair = parts.next().ok_or(anyhow!("No winning numbers"))?;
                let winning : HashSet<i32> = match winning_pair.as_rule() {
                    Rule::winning_numbers => {
                        let numbers = winning_pair.into_inner().next().ok_or(anyhow!("No winning numbers"))?;
                        Ok(TryInto::<Numbers>::try_into(numbers)?)
                    }
                    _ => Err(anyhow!("Not a Winning Numbers Pair {:?}", winning_pair)),
                }?.into();
                let numbers_pair = parts.next().ok_or(anyhow!("No card numbers"))?;
                let numbers : Vec<i32> = match numbers_pair.as_rule() {
                    Rule::card_numbers => {
                        let numbers = numbers_pair.into_inner().next().ok_or(anyhow!("No card numbers"))?;
                        Ok(TryInto::<Numbers>::try_into(numbers)?)
                    }
                    _ => Err(anyhow!("Not a Card Numbers Pair {:?}", numbers_pair)),
                }?.into();

                Ok(Card {
                    id,
                    winning,
                    numbers,
                })
            }
            _ => Err(anyhow!("Not a Card Pair")),
        }
    }
}

#[derive(Debug)]
pub struct Cards {
    cards: Vec<Card>,
    wins: HashMap<i32, i32>,
}



impl Cards {
    
    fn extra_cards(&self, id: i32) -> Vec<i32> {
        let next = id + 1;
        self.wins.get(&id).map(|w| (next .. (next + *w)).collect::<Vec<i32>>()).unwrap_or_default()
    }

    fn score_for(&self, id: i32) -> i32 {
        let extras : i32 = self.extra_cards(id).iter().map(|i| self.score_for(*i)).sum();
        extras + 1
    }
    
    pub fn score_count(&self) -> i32 {
        self.cards.iter().map(|c| self.score_for(c.id)).sum()
    }

    pub fn score(&self) -> i32 {
        self.wins.values().map( |w| if *w > 0 {
            2i32.pow((*w - 1) as u32)
        } else {
            0
        }).sum()
    }
}

impl TryFrom<&str> for Cards {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parsed = ScratcherParser::parse(Rule::cards, value)?;
        let cards_pair = parsed.next().ok_or(anyhow!("No first Pair"))?;
        match cards_pair.as_rule() {
            Rule::cards => {
                let cards : Vec<Card> = cards_pair.into_inner().map(TryFrom::try_from).collect::<Result<Vec<Card>, anyhow::Error>>()?;
                let wins :HashMap<i32,i32> = cards.iter().map(|c| (c.id, c.wins() as i32)).collect();
                Ok(Cards{cards, wins})
            }
            _ => Err(anyhow!("No Cards Pair"))
        }
    }
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a_test() {
        let cards: Cards = include_str!("../data/day_04.test").try_into().unwrap();
        assert_eq!(13, cards.score())
    }

    #[test]
    fn part_a() {
        let cards: Cards = include_str!("../data/day_04.data").try_into().unwrap();
        assert_eq!(25571, cards.score())
    }
    
    #[test]
    fn part_b_test() {
        let cards: Cards = include_str!("../data/day_04.test").try_into().unwrap();
        assert_eq!(30, cards.score_count())
    }

    #[test]
    fn part_b() {
        let cards: Cards = include_str!("../data/day_04.data").try_into().unwrap();
        assert_eq!(8805731, cards.score_count())
    }
}
