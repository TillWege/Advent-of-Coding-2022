use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/day3.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total_1 = 0;

    for line in contents.lines() {
        let (first_half, second_half) = line.split_at(line.chars().count() / 2);
        for c in first_half.chars() {
            if second_half.contains(c) {
                let val = (c as u32) - (if c.is_uppercase() {38} else {96});
                println!("{} = {}", c, val);
                total_1 += val;
                break;
            }
        }        
    }

    let mut line_1 = "";
    let mut line_2 = "";
    let mut total_2 = 0;
    let mut group_count = 0;
        
    for line in contents.lines() {
        if line_1.is_empty() {
            line_1 = line
        } else if line_2.is_empty() {
            line_2 = line
        } else {
            for c in line.chars() {
                if line_1.contains(c) && line_2.contains(c) {
                    let val = (c as u32) - (if c.is_uppercase() {38} else {96});
                    println!("{} = {}", c, val);
                    total_2 += val;
                    group_count += 1;
                    break;
                }
            }


            line_1 = "";
            line_2 = "";
        }
    }

    println!("Total (part 1): {}", total_1);
    println!("Total (part 2): {} groups = {}", group_count, total_2);

    Ok(())
}