use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::string;
use std::vec;
fn main() -> std::io::Result<()> {
    // EACH BINARY IS 12 NUMBERS LONG GAMMA MOST COMMON EPSILON NOT COMMON
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let mut gamma_rate: Vec<String> = vec![];
    let mut epsilon_rate: Vec<String> = vec![];
    let mut temp_gamma = vec![];
    let mut temp_epsilon = vec![];
    buf_reader.read_to_string(&mut contents)?;
    for i in 0..12 {
        let mut amnt_of_zeroes = 0;
        let mut amnt_of_ones = 0;
        let mut common_bit = String::new();
        let mut least_common_bit = String::new();
        for lines in contents.lines() {
            if &lines[i..i+1] == "1" { amnt_of_ones += 1}
            else { amnt_of_zeroes +=1 }
        }
        if amnt_of_ones > amnt_of_zeroes { 
            common_bit = String::from("1");
            least_common_bit = String::from("0");
            temp_gamma.push(common_bit);
            temp_epsilon.push(least_common_bit);
        }
        else {
            common_bit = String::from("0");
            least_common_bit = String::from("1");
            temp_gamma.push(common_bit);
            temp_epsilon.push(least_common_bit);
        }
    }
    println!("{:?}", temp_epsilon);
    let new = temp_epsilon.join("");
    let new2 = temp_gamma.join("");
    epsilon_rate.push(new);
    gamma_rate.push(new2);
    let int_val_epsilon = isize::from_str_radix(epsilon_rate[0].as_str(), 2).unwrap();
    let int_gamma_epsilon = isize::from_str_radix(gamma_rate[0].as_str(), 2).unwrap();
    println!("{:?}", epsilon_rate[0]);
    println!("{int_val_epsilon}");
    println!("{:?}", gamma_rate[0]);
    println!("{int_gamma_epsilon}");
    let total = int_gamma_epsilon * int_val_epsilon;
    println!("{total}");
    let new1 =  part2(contents.clone(), 0);
    let new2 = part2_2(contents.clone(), 0);
    let new_int = isize::from_str_radix(new1.as_str(), 2).unwrap();
    let new_int2 = isize::from_str_radix(new2.as_str(), 2).unwrap();
    let total2 = new_int * new_int2;
    println!("{total2}");
    Ok(())
}

fn part2(contents: String, mut index: i32) -> String {
    let mut strings_vec: Vec<&str> = vec![];
    let mut zeros = 0;
    let mut ones = 0;
    let mut leading = "0";
    for lines in contents.lines() {
        if lines[index as usize..(index as usize) +1 ] == "0".to_string() {
            zeros += 1;
        }
        else { 
            ones += 1;
         }
    }
    if zeros > ones {  leading = "0";  }
    else { leading = "1"; }
    for lines in contents.lines() {
        if &lines[index as usize..(index as usize) +1] == leading {
            strings_vec.push(lines);
        }
    }
    index += 1;
    if index == 12 {
        return strings_vec[0].to_string();
    }
    return part2(strings_vec.join("\n"), index);
}

fn part2_2(contents: String, mut index: i32) -> String {
    let mut strings_vec: Vec<&str> = vec![];
    let mut zeros = 0;
    let mut ones = 0;
    let mut leading = "0";
    for lines in contents.lines() {
        if lines[index as usize..index as usize +1] == "0".to_string() {
            zeros += 1;
        }
        else { 
            ones += 1;
         }
    }
    if zeros <= ones { 
         leading = "0"; 
         }
    else { 
        leading = "1"; 
    }
    for lines in contents.lines() {
        if &lines[index as usize..index as usize +1] == leading {
            strings_vec.push(lines);
        }
    }
    println!("{:?}", strings_vec);
    index += 1;
    if strings_vec.len() == 1 {
        return strings_vec[0].to_string();
    }
    return part2_2(strings_vec.join("\n"), index);
}