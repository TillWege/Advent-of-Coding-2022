#[derive(Debug)]
enum Tile {
    Land(u32),
    Start,
    End
}

#[derive(Debug)]
struct Map {
    side_length: usize,
    map: Vec<Vec<Tile>>,
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day12_example.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;

    let map = read_input(contents);

    println!("{:?}", map);

    Ok(())
}

fn read_input(content: String) -> Map {
    const ASCII_OFFSET: u32 = 96;

    let side_length = content.lines().count();

    let mut result = Map {
        side_length,
        map: Vec::new(),
    };


    for line in content.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            if c == 'S' {
                row.push(Tile::Start);
            } else if c == 'E' {
                row.push(Tile::End);
            } else {
                row.push(Tile::Land((c as u32) - ASCII_OFFSET));
            }
        }
        result.map.push(row);
    }
    
    result
}
