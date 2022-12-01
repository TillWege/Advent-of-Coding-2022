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

    println!("Elve {} carries {} calories", largest_index, largest_cal_count);   

    Ok(())
}