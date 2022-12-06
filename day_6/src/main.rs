use std::{fs::File, io::{Read}};

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/day6.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut chars: Vec<char> = Vec::new();


    for (index, c) in contents.chars().enumerate() {
        chars.push(c);

        let len = chars.len();

        if len >= 4 {
            let last4 = chars.as_slice()[chars.len()-4..].to_vec();
            if !(1..last4.len()).any(|i| last4[i..].contains(&last4[i - 1])) {
                println!("found 4 uniqs at pos: {}", index + 1);
                break;
            }
        }
    }
    Ok(())
}