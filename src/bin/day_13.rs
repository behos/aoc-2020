use anyhow::{Context, Result};
use aoc_2020::read_entries;

fn main() -> Result<()> {
    let mut entries = read_entries::<String>("./data/day-13.txt");
    let timestamp = entries
        .next()
        .context("Missing timestamp.")?
        .parse::<usize>()
        .context("Unparseable timestamp.")?;
    let raw_ids = entries.next().context("Missing timetables.")?;
    let bus_ids: Vec<_> = raw_ids.split(",").collect();
    let res = next_departure(timestamp, &bus_ids)?;
    println!("Result part 1 {}", res);
    Ok(())
}

fn next_departure(timestamp: usize, bus_ids: &[&str]) -> Result<usize> {
    let (bus_id, time_to_wait) = bus_ids
        .iter()
        .filter(|&&bus_id| bus_id != "x")
        .map(|bus_str| bus_str.parse::<usize>().context("Unparseable bus id."))
        .flatten()
        .map(|bus_id| (bus_id, time_to_wait(timestamp, bus_id)))
        .min_by_key(|(_, time_to_wait)| *time_to_wait)
        .context("Iterator empty")?;
    println!("Bus id {} time to wait {}", bus_id, time_to_wait);
    Ok(bus_id * time_to_wait)
}

fn time_to_wait(timestamp: usize, bus_id: usize) -> usize {
    (timestamp as f32 / bus_id as f32).ceil() as usize * bus_id - timestamp
}
