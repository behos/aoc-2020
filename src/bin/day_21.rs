use anyhow::{Context, Error, Result};
use aoc_2020::read_entries;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Recipe {
    type Err = Error;

    fn from_str(input: &str) -> Result<Recipe> {
        let mut parts = input.split(" (contains ");
        let ingredients = parts
            .next()
            .context("Missing ingredients.")?
            .split(" ")
            .map(str::to_string)
            .collect();

        let allergens = parts
            .next()
            .context("Missing allergens.")?
            .strip_suffix(")")
            .context("Did not end with ).")?
            .split(", ")
            .map(str::to_string)
            .collect();

        Ok(Recipe {
            ingredients,
            allergens,
        })
    }
}

fn main() {
    let recipes =
        read_entries::<Recipe>("./data/day-21.txt").collect::<Vec<_>>();

    let mut allergen_index = find_allergens(&recipes);
    allergen_index.sort_by_key(|(a, _)| a.to_string());
    println!(
        "Canonical dangerous ingredient list: {}",
        allergen_index
            .iter()
            .map(|(_, i)| i.as_str())
            .collect::<Vec<_>>()
            .join(",")
    );

    let dangerous_ingredients_candidates = allergen_index
        .iter()
        .map(|(_, a)| a)
        .cloned()
        .collect::<HashSet<String>>();
    let safe_count = recipes
        .iter()
        .map(|recipe| {
            recipe
                .ingredients
                .iter()
                .filter(|&ing| !dangerous_ingredients_candidates.contains(ing))
                .count()
        })
        .sum::<usize>();
    println!("Safe ingredients appear {} times", safe_count);
}

fn find_allergens(recipes: &Vec<Recipe>) -> Vec<(String, String)> {
    let mut comprehensive_list =
        recipes.iter().fold(HashMap::new(), |mut index, recipe| {
            for allergen in &recipe.allergens {
                let entry =
                    index.entry(allergen).or_insert(recipe.ingredients.clone());
                *entry = entry
                    .intersection(&recipe.ingredients)
                    .cloned()
                    .collect::<HashSet<_>>()
            }
            index
        });

    let mut pairs = vec![];

    while let Some(allergen) = comprehensive_list
        .keys()
        .find(|allergen| comprehensive_list[*allergen].len() == 1)
        .map(|s| s.to_owned())
    {
        let mut ingredients = comprehensive_list.remove(&allergen).unwrap();
        let ingredient = ingredients.drain().next().unwrap();
        for ingredients in comprehensive_list.values_mut() {
            ingredients.remove(&ingredient);
        }
        pairs.push((allergen.clone(), ingredient));
    }
    pairs
}
