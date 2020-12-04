use aoc_2020::read_entries;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref REQUIRED: HashSet<String> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .map(|s| s.to_string())
            .collect();
    static ref COLOR_REGEX: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new("^[0-9]{9}$").unwrap();
}

fn main() {
    let mut fields = HashSet::new();
    let mut valid = 0;
    let mut valid_fields = 0;
    for entry in read_entries::<String>("./data/day-04.txt") {
        fields = if entry != "" {
            let line_fields = entry
                .split(" ")
                .map(|field| {
                    let mut parts = field.split(":");
                    (
                        parts.next().expect("Missing tag name.").to_string(),
                        parts.next().expect("Missing tag value.").to_string(),
                    )
                })
                .collect();
            fields.union(&line_fields).cloned().collect()
        } else {
            let (f, v) = validate(fields);
            valid_fields += f;
            valid += v;
            HashSet::new()
        }
    }
    let (f, v) = validate(fields);
    valid_fields += f;
    valid += v;

    println!("Found {} passports with all fields.", valid_fields);
    println!("Found {} valid passports.", valid);
}

fn validate(fields: HashSet<(String, String)>) -> (usize, usize) {
    let mut found_fields = HashSet::new();
    let mut valid_fields = true;
    for (key, value) in fields {
        found_fields.insert(key.clone());
        valid_fields &= match key.as_str() {
            "byr" => validate_date(value, 1920, 2002),
            "iyr" => validate_date(value, 2010, 2020),
            "eyr" => validate_date(value, 2020, 2030),
            "hgt" => validate_height(value),
            "hcl" => validate_hair_color(value),
            "ecl" => validate_eye_color(value),
            "pid" => validate_pid(value),
            "cid" => true,
            _ => false,
        };
    }
    let all_required = REQUIRED.is_subset(&found_fields);
    let required_and_valid = all_required && valid_fields;
    (
        if all_required { 1 } else { 0 },
        if required_and_valid { 1 } else { 0 },
    )
}

fn validate_date(value: String, min: u16, max: u16) -> bool {
    match value.parse::<u16>() {
        Ok(val) => min <= val && val <= max,
        _ => false,
    }
}

fn validate_height(mut value: String) -> bool {
    let unit = value.split_off(value.len() - 2);
    match (value.parse::<u16>(), unit.as_str()) {
        (Ok(val), "cm") => 150 <= val && val <= 193,
        (Ok(val), "in") => 59 <= val && val <= 76,
        _ => false,
    }
}

fn validate_hair_color(value: String) -> bool {
    COLOR_REGEX.is_match(&value)
}

fn validate_eye_color(value: String) -> bool {
    match value.as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn validate_pid(value: String) -> bool {
    PID_REGEX.is_match(&value)
}
