fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day7.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;
    Ok(())
}
