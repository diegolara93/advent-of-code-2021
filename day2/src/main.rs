use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut position_map: HashMap<&str, i32>  = HashMap::new();
    position_map.insert("forward", 0);
    position_map.insert("down", 0);
    position_map.insert("up", 0);
    let mut depth = 0;
    let mut aim = 0;
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    for movements in contents.lines() {
        if movements.contains("forward") {
            let units = movements[8..].parse::<i32>().unwrap();
            position_map.insert("forward", position_map.get("forward").unwrap() + units);
            depth += aim * units;
        }
        if movements.contains("down") {
            let units = movements[5..].parse::<i32>().unwrap();
            // position_map.insert("down", position_map.get("down").unwrap() + units); // only needed for pt1
            aim += units;
        }
        if movements.contains("up") {
            let units = movements[3..].parse::<i32>().unwrap();
            // position_map.insert("up", position_map.get("up").unwrap() + units); // only needed for pt1
            aim -= units;
        }
    }
    let total = depth * position_map.get("forward").unwrap();
    println!("{:?}", total);
    Ok(())
}
