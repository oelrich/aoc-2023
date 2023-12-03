use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "schematic.pest"]
struct SchematicParser;

trait Neighbours {
    fn neighbours(&self) -> HashSet<Point>;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point (i32,i32);

impl Neighbours for Point {
    fn neighbours(&self) -> HashSet<Point> {
        HashSet::from([
            Point(self.0 - 1, self.1 - 1),
            Point(self.0,     self.1 - 1),
            Point(self.0 + 1, self.1 - 1),
            Point(self.0 - 1, self.1),
            // Point(self.0, self.1),
            Point(self.0 + 1, self.1),
            Point(self.0 - 1, self.1 + 1),
            Point(self.0,     self.1 + 1),
            Point(self.0 + 1, self.1 + 1)])
    }
}

impl Neighbours for Vec<Point> {
    fn neighbours(&self) -> HashSet<Point> {
      self.iter().fold(HashSet::new(), |mut a,p| {a.extend(p.neighbours()); a})
    }
}

impl TryFrom<Pair<'_, Rule>> for Point {
    type Error = anyhow::Error;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let (l,c) = value.line_col();
        Ok(Point(c.try_into()?, l.try_into()?))
    }
}

#[derive(Debug)]
enum Entry { Part(String, Point), PartNumber(i32, Vec<Point>) }

impl TryFrom<Pair<'_, Rule>> for Entry {
    type Error = anyhow::Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        
        match value.as_rule() {
            Rule::part => {
                Ok(Entry::Part(value.as_str().to_owned(),
                               Point::try_from(value)?))
            }
            Rule::part_number => {
                let part_number_text = value.as_str();
                let Point(x,y) = value.try_into()?; 
                let len : i32 = part_number_text.len().try_into()?;
                let points = (0..len).map(|o| Point(x+o, y)).collect();
                let part_number = part_number_text.parse()?;

                Ok(Entry::PartNumber(part_number,  points))
            }
            _ => Err(anyhow!("Not an Entry Pair"))
        }
    }
}

#[derive(Debug)]
struct Schematic {
    parts: HashMap<Point, String>,
    neighbour_number: HashMap<Point, HashSet<(i32, Vec<Point>)>>,
    part_numbers: Vec<(i32, Vec<Point>)>,
}

impl Schematic {
    fn get_gear_powers(&self) -> Vec<(Point, i32)> {
        self.parts.iter()
            .filter(|(_p, s)| *s == "*")
            .filter_map(|(p, s)|
            { self.neighbour_number
                  .get(p)
                  .filter(|h| h.len() == 2)
                  .map(|h| {
                let ps = h.iter().fold(1, |a,b| a * b.0);
                (*p, ps) })})
            .collect()
    }
    fn get_part_numbers(&self) -> Vec<i32> {
        self.part_numbers.iter()
        .filter_map(|(pn, ps)| {
                                     let ns = ps.neighbours();
                                     if ns.iter().any(|p| self.parts.contains_key(p)) {
                                        Some(*pn)
                                    } else { None } }).collect()
    }
}

impl TryFrom<&str> for Schematic {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parser = SchematicParser::parse(Rule::schematic, value)?;
        let entries : Vec<Entry> = parser.into_iter().map(Entry::try_from).collect::<Result<Vec<Entry>, anyhow::Error>>()?;
        let mut parts = HashMap::new();
        let mut neighbour_number = HashMap::new();
        let mut part_numbers = Vec::new();
        for entry in entries {
            match entry {
                Entry::Part(part, pos) => {parts.insert(pos, part);}
                Entry::PartNumber(part_number, positions) => {
                    let pn = (part_number, positions.clone());
                    for neighbour in positions.neighbours() {
                        neighbour_number.entry(neighbour).or_insert(HashSet::new()).insert(pn.clone());
                    }
                        part_numbers.push((part_number, positions));
                    }
            }
        }

        Ok(Schematic{parts, neighbour_number, part_numbers})
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a_test() {
        let schema : Schematic = include_str!("../data/day_03.test.data").try_into().unwrap();
        let part_sum : i32 = schema.get_part_numbers().iter().sum();
        assert_eq!(4361, part_sum)
    }

    #[test]
    fn part_a() {
        let schema : Schematic = include_str!("../data/day_03.data").try_into().unwrap();
        let part_sum : i32 = schema.get_part_numbers().iter().sum();
        assert_eq!(520135, part_sum)
    }

    #[test]
    fn part_b_test() {
        let schema : Schematic = include_str!("../data/day_03.test.data").try_into().unwrap();
        let part_sum : i32 = schema.get_gear_powers().iter().map(|(_p, gp)| *gp).sum();
        assert_eq!(467835, part_sum)
    }

    #[test]
    fn part_b() {
        let schema : Schematic = include_str!("../data/day_03.data").try_into().unwrap();
        let part_sum : i32 = schema.get_gear_powers().iter().map(|(_p, gp)| *gp).sum();
        assert_eq!(72514855, part_sum)
    }
}
