use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/day3.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total = 0;

    for line in contents.lines() {
        let (first_half, second_half) = line.split_at(line.chars().count() / 2);
        for c in first_half.chars() {
            if second_half.contains(c) {
                let val = (c as u32) - (if c.is_uppercase() {38} else {96});
                println!("{} = {}", c, val);
                total += val;
                break;
            }
        }        
    }

    println!("Total: {}", total);


    Ok(())
}