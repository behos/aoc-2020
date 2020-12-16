use anyhow::{Context, Error, Result};
use std::{
    collections::HashSet, fs::read_to_string, ops::RangeInclusive, str::FromStr,
};

type Ticket = Vec<usize>;

struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut parts = input.split(": ");
        let name = parts.next().context("Missing name section.")?.to_string();
        let ranges = parts
            .next()
            .context("Missing range section.")?
            .split(" or ")
            .map(|range_str| {
                let mut parts = range_str.split("-");
                let low = parts
                    .next()
                    .context("Missing low.")?
                    .parse()
                    .context("Unparseable low range.")?;
                let high = parts
                    .next()
                    .context("Missing high.")?
                    .parse()
                    .context("Unparseable high range.")?;
                Ok(low..=high)
            })
            .collect::<Result<_>>()?;
        Ok(Self { name, ranges })
    }
}

impl Rule {
    fn contains(&self, num: &usize) -> bool {
        self.ranges.iter().any(|range| range.contains(num))
    }
}

fn main() -> Result<()> {
    let input = read_to_string("./data/day-16.txt")?;
    let (rules, ticket, mut scanned_tickets) = parse_input(&input)?;

    println!(
        "Scanning error rate {}",
        get_scanning_error_rate(&rules, &scanned_tickets)
    );

    let valid_scanned_tickets = scanned_tickets
        .drain(..)
        .filter(|t| is_valid(&rules, &t))
        .collect::<Vec<_>>();

    let columns = identify_columns(&rules, &valid_scanned_tickets);

    println!(
        "Departure product {}",
        columns
            .iter()
            .enumerate()
            .filter(|(_, c)| c.starts_with("departure"))
            .map(|(i, _)| ticket[i])
            .product::<usize>()
    );
    Ok(())
}

fn parse_input(input: &str) -> Result<(Vec<Rule>, Ticket, Vec<Ticket>)> {
    let mut sections = input.split("\n\n");
    let rules = sections
        .next()
        .context("Missing rule section.")?
        .split("\n")
        .map(|s| s.parse::<Rule>())
        .collect::<Result<Vec<_>>>()?;

    let ticket = parse_ticket(
        sections
            .next()
            .context("Missing ticket section.")?
            .split("\n")
            .skip(1)
            .next()
            .context("Input ended before my ticket.")?,
    )?;

    let scanned_tickets = sections
        .next()
        .context("Missing scanned ticket section.")?
        .split("\n")
        .skip(1)
        .take_while(|&s| s != "")
        .map(parse_ticket)
        .collect::<Result<_>>()?;
    Ok((rules, ticket, scanned_tickets))
}

fn parse_ticket(input: &str) -> Result<Ticket> {
    input
        .split(",")
        .map(|s| s.parse::<usize>().context("Unparseable ticket number."))
        .collect()
}

fn get_scanning_error_rate(rules: &[Rule], tickets: &[Ticket]) -> usize {
    tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|num| rules.iter().all(|rule| !rule.contains(num)))
                .sum::<usize>()
        })
        .sum()
}

fn is_valid(rules: &[Rule], ticket: &[usize]) -> bool {
    ticket
        .iter()
        .all(|num| rules.iter().any(|rule| rule.contains(num)))
}

fn identify_columns<'a>(
    rules: &'a Vec<Rule>,
    tickets: &[Ticket],
) -> Vec<&'a str> {
    let width = rules.len();
    let mut candidate_columns = vec![HashSet::new(); width];
    let mut columns = vec![""; width];

    // A candidate column is one whose rule validates all tickets in that
    // column.
    for i in 0..width {
        for rule in rules {
            if tickets.iter().all(|ticket| rule.contains(&ticket[i])) {
                candidate_columns[i].insert(&rule.name);
            }
        }
    }

    // Trim the candidate columns by checking which ones can only have 1 value
    // and excluding it from everywhere else.
    while let Some((i, column)) = candidate_columns
        .iter_mut()
        .enumerate()
        .find(|(_, c)| c.len() == 1)
    {
        let rule = column.drain().next().expect("There is certainly one.");
        for i in 0..width {
            candidate_columns[i].remove(rule);
        }
        columns[i] = rule;
    }

    columns
}
