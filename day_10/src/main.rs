use std::vec;

struct CPU {
    cycle: isize,
    reg_x: isize,
    output_line: Vec<String>
}

impl CPU {
    fn tick(& mut self) -> isize {
        if self.cycle == 1 {
            self.output_line.push("#".to_string());
        }

        let char_pos = self.cycle % 40;
        if (char_pos - self.reg_x).abs() <= 1 {
            self.output_line.push("#".to_string())
        } else {
            self.output_line.push(".".to_string())
        }


        self.cycle += 1;
        if (self.cycle - 20) % 40 == 0 {
            let sig_strength = self.cycle * self.reg_x;
            //println!("at cycle {} signal strength is {}", self.cycle, sig_strength);
            return sig_strength;
        }

        if (self.cycle % 40) == 0 {
            println!("{}", self.output_line.join(""));
            self.output_line = vec![];
        }
        0
    }
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day10.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;

    let mut cpu = CPU {
        cycle: 1,
        reg_x: 1,
        output_line: vec![]
    };


    let mut total = 0;

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let ins = parts.next().unwrap();
        match ins {
            "addx" => {
                let arg: isize = parts.next().unwrap().parse().unwrap();
                total += cpu.tick();
                cpu.reg_x += arg;
                total += cpu.tick();
            },
            "noop" => {

                total += cpu.tick();
            }
            _ => panic!()
        }
    }

    println!("total: {}", total);



    Ok(())
}