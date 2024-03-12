use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Add;
// fn main() -> std::io::Result<()> {
//     let  input = File::open("input.txt")?;
//     let mut contents: Vec<i32> = vec![];
//     let mut content_string = String::new();
//     let mut buf_reader = BufReader::new(input);
//     let mut counter = 0;
//     buf_reader.read_to_string(&mut content_string)?;
//     for lines in content_string.lines() {
//         contents.push(lines.parse::<i32>().unwrap());
//     }
//     for items in 0..contents.len() {
//         let  temp = contents[items];
//         if items == 0 { continue };
//         if temp > contents[items.sub(1)] {
//             counter += 1;
//         }
//     }
//     println!("{counter}");
//     Ok(())
// }
fn main() -> std::io::Result<()> {
    let  input = File::open("input.txt")?;
    let mut contents: Vec<i32> = vec![];
    let mut content_string = String::new();
    let mut buf_reader = BufReader::new(input);
    let mut counter = 0;
    buf_reader.read_to_string(&mut content_string)?;
    for lines in content_string.lines() {
        contents.push(lines.parse::<i32>().unwrap());
    }
    for  items in 0..contents.len() {
        if items == contents.len() - 3 { break } ;
        let  first_iteration = contents[items] +
        contents[items.add(1)] + contents[items.add(2)];
        let next_iteration =  contents[items.add(1)] +  contents[items.add(2)] +  contents[items.add(3)];
        if first_iteration < next_iteration {
            counter += 1;
        }
    }
    println!("{counter}");
    Ok(())
}
