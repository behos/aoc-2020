use anyhow::{Context, Error, Result};
use aoc_2020::read_entries;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type RuleIndex = HashMap<String, Vec<(String, usize)>>;
type ColorIndex = HashMap<String, Vec<String>>;

struct Rule((String, Vec<(String, usize)>));
impl FromStr for Rule {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut main_color_split = input.splitn(2, " bags contain ");
        let main_color =
            main_color_split.next().context("Missing main color.")?;
        let rules_str =
            main_color_split.next().context("Missing rules def.")?;
        let rules = rules_str
            .split(", ")
            .filter(|s| s != &"no other bags.")
            .map(|rule_str| {
                let mut split = rule_str.splitn(2, " ");
                let count = split
                    .next()
                    .context("Missing count.")?
                    .parse::<usize>()
                    .context("Count should be a number.")?;
                let color = split
                    .next()
                    .context("Missing color def.")?
                    .rsplitn(2, " ")
                    .last()
                    .context("Missing color name.")?;
                Ok((color.to_string(), count))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self((main_color.to_string(), rules)))
    }
}

fn main() {
    let (rule_index, color_index) = read_entries::<Rule>("./data/day-07.txt")
        .fold(
            (RuleIndex::new(), ColorIndex::new()),
            |(mut rules, mut colors), rule| {
                let Rule((main_color, contains)) = rule;
                for (color, _) in &contains {
                    colors
                        .entry(color.clone())
                        .or_insert(vec![])
                        .push(main_color.clone());
                }
                rules.insert(main_color, contains);
                (rules, colors)
            },
        );
    let my_color = "shiny gold";
    let containers = find_containers(my_color, &color_index);
    println!("Found {} possible container colors.", containers.len() - 1);

    let must_contain = contain_count(my_color, &rule_index);
    println!("My bag must contain {} bags.", must_contain);
}

fn find_containers<'a>(
    color: &'a str,
    index: &'a ColorIndex,
) -> HashSet<&'a str> {
    let mut containers = HashSet::new();
    containers.insert(color);

    if let Some(container_colors) = index.get(color) {
        for color in container_colors {
            containers.extend(&find_containers(color, &index));
        }
    }
    containers
}

fn contain_count(color: &str, rule_index: &RuleIndex) -> usize {
    rule_index[color]
        .iter()
        .map(|(color, count)| count + count * contain_count(color, rule_index))
        .sum()
}
