use anyhow::{Context, Error, Result};
use std::{fs::read_to_string, str::FromStr};

#[derive(Debug)]
enum Matcher {
    RuleRef(usize),
    Char(char),
}

impl Matcher {
    fn parse_all(input: &str) -> Result<Vec<Self>> {
        input
            .split(" ")
            .map(str::trim)
            .map(Matcher::from_str)
            .collect()
    }
}

impl FromStr for Matcher {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        input
            .parse::<usize>()
            .and_then(|n| Ok(Self::RuleRef(n)))
            .or_else(|_| {
                Ok(Self::Char(
                    input
                        .chars()
                        .nth(1)
                        .context("Expected a middle character.")?,
                ))
            })
    }
}

struct Rule {
    id: usize,
    matchers: Vec<Vec<Matcher>>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut parts = input.split(": ");
        let id = parts.next().context("Expected id part.")?.parse()?;
        let matchers = parts
            .next()
            .context("Expected rule part")?
            .split(" | ")
            .map(Matcher::parse_all)
            .collect::<Result<_>>()?;
        Ok(Self { id, matchers })
    }
}

fn rule_matches<'a>(
    input: &'a str,
    rule_id: usize,
    rules: &[Rule],
) -> Vec<&'a str> {
    rules[rule_id]
        .matchers
        .iter()
        .map(|seq| sequence_matches(input, seq, rules))
        .flatten()
        .collect()
}

fn sequence_matches<'a>(
    input: &'a str,
    seq: &[Matcher],
    rules: &[Rule],
) -> Vec<&'a str> {
    match seq {
        [] => vec![input],
        [Matcher::Char(c), rest @ ..] => match input.chars().next() {
            Some(inc) if inc == *c => {
                sequence_matches(&input[1..], rest, rules)
            }
            _ => vec![],
        },
        [Matcher::RuleRef(rule_id), rest @ ..] => {
            rule_matches(input, *rule_id, rules)
                .iter()
                .map(|remaining| sequence_matches(remaining, rest, rules))
                .flatten()
                .collect()
        }
    }
}

fn main() -> Result<()> {
    let input = read_to_string("./data/day-19.txt")?;
    let mut sections = input.split("\n\n");
    let mut rules = sections
        .next()
        .context("Expected rules section.")?
        .split("\n")
        .map(Rule::from_str)
        .collect::<Result<Vec<_>>>()?;
    rules.sort_by_key(|r| r.id);

    let inputs = sections
        .next()
        .context("Expected inputs")?
        .split("\n")
        .collect::<Vec<_>>();

    let match_rule_0 = inputs
        .iter()
        .filter(|i| rule_matches(i, 0, &rules).contains(&""))
        .count();

    println!("{} messages match rule 0 before update.", match_rule_0);
    // Updates
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31

    rules[8] = Rule {
        id: 8,
        matchers: vec![
            vec![Matcher::RuleRef(42)],
            vec![Matcher::RuleRef(42), Matcher::RuleRef(8)],
        ],
    };

    rules[11] = Rule {
        id: 11,
        matchers: vec![
            vec![Matcher::RuleRef(42), Matcher::RuleRef(31)],
            vec![
                Matcher::RuleRef(42),
                Matcher::RuleRef(11),
                Matcher::RuleRef(31),
            ],
        ],
    };

    let match_rule_0 = inputs
        .iter()
        .filter(|i| rule_matches(i, 0, &rules).contains(&""))
        .count();

    println!("{} messages match rule 0 post update.", match_rule_0);
    Ok(())
}
