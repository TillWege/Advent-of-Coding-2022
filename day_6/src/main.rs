use std::{fs::File, io::{Read}};

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/day6.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Aufgabe 1: {}", task(&contents, 4));
    println!("Aufgabe 2: {}", task(&contents, 14));

    Ok(())
}

fn task(str: &String, uniq_char_len: usize) -> usize {
    let mut chars: Vec<char> = Vec::new();


    for (index, c) in str.chars().enumerate() {
        chars.push(c);

        let len = chars.len();

        if len >= uniq_char_len {
            let last4 = chars.as_slice()[chars.len()-uniq_char_len..].to_vec();
            if !(1..last4.len()).any(|i| last4[i..].contains(&last4[i - 1])) {
                return index + 1;
            }
        }
    }

    0
}