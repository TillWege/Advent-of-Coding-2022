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

        let range_2_lower: i32 = second_half.split('-').next().unwrap().parse().unwrap();
        let range_2_upper: i32 = second_half.split('-').last().unwrap().parse().unwrap();

        // prüfen ob range 1 innerhalb von range 2 liegt
        if (range_1_lower >= range_2_lower) && (range_1_upper <= range_2_upper) {
            total += 1;
        } else if (range_2_lower >= range_1_lower) && (range_2_upper <= range_1_upper) {
            total += 1;
        }
    }

    println!("found {} included ranges", total);

    Ok(())
}
