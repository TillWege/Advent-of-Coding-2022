use std::{collections::HashSet, vec};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Vec2D {
    x: isize,
    y: isize
}

impl Default for Vec2D {
    fn default() -> Self {
        Vec2D { x: 0, y: 0 }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day9.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;

    let mut visited_pos: HashSet<Vec2D> = HashSet::new(); 
    let start_pos = Vec2D::default();
    visited_pos.insert(start_pos);

    let mut rope: Vec<Vec2D> = Vec::new();

    let rope_elements = 10;

    for _ in 0..rope_elements {
        rope.push(Vec2D::default())
    }


    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let amount = parts.next().unwrap().parse().unwrap();

        for _ in 0..amount {

            // move head
            match direction {
                "L" => {
                    rope[0].x -= 1;
                },
                "R" => {
                    rope[0].x += 1;
                },
                "U" => {
                    rope[0].y += 1;
                },
                "D" => {
                    rope[0].y -= 1;
                },
                _ => panic!()
            }

            // move tail
            for index in 1..rope.len() {
                let dist_x = rope[index - 1].x - rope[index].x;
                let dist_y = rope[index - 1].y - rope[index].y;

                // check if neighbouring
                if (dist_x.abs() > 1) || (dist_y.abs() > 1) {
                    // update tail if not neighbouring anymore
                    if rope[index - 1].y > rope[index].y {
                        rope[index].y += 1;
                    } else if rope[index - 1].y < rope[index].y{
                        rope[index].y -= 1;
                    }

                    if rope[index - 1].x > rope[index].x {
                        rope[index].x += 1;
                    } else if rope[index - 1].x < rope[index].x {
                        rope[index].x -= 1;
                    }

                }
            }

            visited_pos.insert(rope.last().unwrap().clone());
        }


    }


    println!("visited {} unique positions with tail", visited_pos.len());
    //dbg!(visited_pos);

    Ok(())
}
