fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day8.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;

    let mut trees: Vec<u32> = Vec::new();
    for char in contents.chars() {
        if let Some(height) = char.to_digit(10) {
            trees.push(height);
        }
    }











    // let mut count = 1;
    // for tree in trees {
    //     print!("{}", tree);
    //     count += 1;
    //     if count == 100 {
    //         println!();
    //         count = 1;
    //     } 
    // }


    Ok(())
}

fn get_index(x: u32, y: u32) -> usize {
    (x + (100 * y)) as usize
}