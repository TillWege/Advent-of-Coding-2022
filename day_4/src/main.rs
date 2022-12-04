use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/day4.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total = 0;

    for line in contents.lines() {
        let (first_half, second_half) = line.split_once(',').unwrap();

        let range_1_lower: i32 = first_half.split('-').next().unwrap().parse().unwrap();
        let range_1_upper: i32 = first_half.split('-').last().unwrap().parse().unwrap();

        let range_1 = range_1_lower..range_1_upper;

        let range_2_lower: i32 = second_half.split('-').next().unwrap().parse().unwrap();
        let range_2_upper: i32 = second_half.split('-').last().unwrap().parse().unwrap();

        let range_2 = range_2_lower..range_2_upper;

        if (range_1.contains(&range_2_lower) && range_1.contains(&range_2_upper))
            || (range_2.contains(&range_1_lower) && range_2.contains(&range_1_upper))
        {
            total += 1;
        }
    }

    println!("found {} included ranges", total);

    Ok(())
}
