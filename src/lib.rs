use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn read_entries<T: FromStr>(path: &str) -> impl Iterator<Item = T>
where
    T::Err: Debug,
{
    let file = File::open(path).expect("Could not open file.");
    let reader = BufReader::new(file);
    reader.lines().map(|s| {
        s.expect("Could not read line.")
            .parse::<T>()
            .expect("Failed to parse line.")
    })
}
