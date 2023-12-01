pub fn add_first_last(line: &str) -> anyhow::Result<i32> {
    let clean = line.trim_matches(char::is_alphabetic);
    let f: i32 = clean[0..1].parse()?;
    let l: i32 = clean[(clean.len() - 1)..].parse()?;
    Ok(f * 10 + l)
}

fn text_to_num(text: &str) -> &str {
    match text {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "eno" => "1",
        "owt" => "2",
        "eerht" => "3",
        "ruof" => "4",
        "evif" => "5",
        "xis" => "6",
        "neves" => "7",
        "thgie" => "8",
        "enin" => "9",
        _ => ""
    }
}
fn mun_ot_txet(text: &str) -> &str {
    match text {
        "eno" => "1",
        "owt" => "2",
        "eerht" => "3",
        "ruof" => "4",
        "evif" => "5",
        "xis" => "6",
        "neves" => "7",
        "thgie" => "8",
        "enin" => "9",
        _ => ""
    }
}

pub fn add_first_last_text(line: &str) -> anyhow::Result<i64> {
    let re = regex::Regex::new(r"(?<number>one|two|three|four|five|six|seven|eight|nine)")?;
    let after = re.replace(line,|c: &regex::Captures| text_to_num(&c["number"]).to_string());
    let clean = after.trim_start_matches(char::is_alphabetic);
    let f: i64 = clean[0..1].parse()?;

    let er = regex::Regex::new(r"(?<number>eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)")?;
    let rev: String = line.chars().rev().collect();
    let after = er.replace(&rev, |c: &regex::Captures| mun_ot_txet(&c["number"]).to_string());
    let clean = after.trim_start_matches(char::is_alphabetic);
    let l: i64 = clean[0..1].parse()?;
    Ok(f * 10 + l)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let result: i32 = include_str!("../data/day_01.data")
            .lines()
            .map(|l| add_first_last(l).unwrap())
            .sum();
        assert_eq!(55108, result)
    }

    #[test]
    fn part_b() {
        let result: i64 = include_str!("../data/day_01.data")
            .lines()
            .map(|l| add_first_last_text(l).unwrap())
            .sum();
        assert_eq!(56324, result)
    }
}
