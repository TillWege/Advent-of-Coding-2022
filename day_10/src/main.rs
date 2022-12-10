fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day10.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;

    let mut reg_x = 1;
    let mut cycle = 1;

    let mut total = 0;

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let ins = parts.next().unwrap();
        match ins {
            "addx" => {
                let arg: i32 = parts.next().unwrap().parse().unwrap();
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    let sig_strength = cycle * reg_x;
                    total += sig_strength;
                    println!("at cycle {} signal strength is {}", cycle, sig_strength);
                }
                reg_x += arg;
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    let sig_strength = cycle * reg_x;
                    total += sig_strength;
                    println!("at cycle {} signal strength is {}", cycle, sig_strength);
                }
            },
            "noop" => {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    let sig_strength = cycle * reg_x;
                    total += sig_strength;
                    println!("at cycle {} signal strength is {}", cycle, sig_strength);
                }
            }
            _ => panic!()
        }
    }

    println!("total: {}", total);



    Ok(())
}