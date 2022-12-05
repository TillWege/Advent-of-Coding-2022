use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/day4.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total_1 = 0;
    let mut total_2 = 0;
    for line in contents.lines() {
        let (first_half, second_half) = line.split_once(',').unwrap();

        let range_1_lower: i32 = first_half.split('-').next().unwrap().parse().unwrap();
        let range_1_upper: i32 = first_half.split('-').last().unwrap().parse().unwrap();

        let range_2_lower: i32 = second_half.split('-').next().unwrap().parse().unwrap();
        let range_2_upper: i32 = second_half.split('-').last().unwrap().parse().unwrap();

        if (range_1_lower >= range_2_lower) && (range_1_upper <= range_2_upper) {
            total_1 += 1;
        } else if (range_2_lower >= range_1_lower) && (range_2_upper <= range_1_upper) {
            total_1 += 1;
        }

        if ((range_1_lower >= range_2_lower) && (range_1_lower <=range_2_upper)) || 
            ((range_1_upper <= range_2_upper) && (range_1_upper >= range_2_lower)) {
                total_2 += 1;
        } else if ((range_2_lower >= range_1_lower) && (range_2_lower <= range_1_upper)) || 
            ((range_2_upper <= range_1_upper) && (range_2_upper >= range_1_lower)) {
                total_2 += 1;
        } 

    }

    println!("(part 1) found {} included ranges", total_1);
    println!("(part 2) found {} included ranges", total_2);

    Ok(())
}