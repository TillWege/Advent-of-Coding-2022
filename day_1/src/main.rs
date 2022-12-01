use std::{fs::File, io::Read, usize};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    

    let mut elves: Vec<usize> = vec![0];

    for line in contents.lines() {
        if line.is_empty() {
            elves.push(0);
        } else {
            let current_cals: usize = line.parse().unwrap();
            
            let current_elve = elves.last_mut().unwrap();
            *current_elve += current_cals;
        }
    }

    let mut largest_index: usize = 0;
    let mut largest_cal_count: usize = 0;
    for (index, cal_count) in elves.iter().enumerate() {
        if *cal_count > largest_cal_count {
            largest_index = index;
            largest_cal_count = *cal_count
        }
    }


    let mut top_1: usize = 0;
    let mut top_2: usize = 0;
    let mut top_3: usize = 0;

    for cal_count in elves.iter() {
        if *cal_count > top_1 {
            top_3 = top_2;
            top_2 = top_1;
            top_1 = *cal_count;
        } else if *cal_count > top_2 {
            top_3 = top_2;
            top_2 = *cal_count;
        } else if *cal_count > top_3 {
            top_3 = *cal_count;
        }
    }

    println!("Elve with most calories:");
    println!("Elve {} carries {} calories", largest_index, largest_cal_count);   

    println!("-----------------");
    println!("Top 3 Elves:");
    println!("Top 1: {}", top_1);
    println!("Top 2: {}", top_2);
    println!("Top 3: {}", top_3);
    println!("Top 3 Total: {}", top_1 + top_2 + top_3);

    Ok(())
}