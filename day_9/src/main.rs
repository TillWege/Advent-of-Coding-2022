use std::collections::HashSet;

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

    let mut head_pos = Vec2D::default();
    let mut tail_pos = Vec2D::default(); 


    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let amount = parts.next().unwrap().parse().unwrap();

        for _ in 0..amount {

            // move head
            match direction {
                "L" => {
                    head_pos.x -= 1;
                },
                "R" => {
                    head_pos.x += 1;
                },
                "U" => {
                    head_pos.y += 1;
                },
                "D" => {
                    head_pos.y -= 1;
                },
                _ => panic!()
            }

            // move tail
            {
                let dist_x = head_pos.x - tail_pos.x;
                let dist_y = head_pos.y - tail_pos.y;

                // check if neighbouring
                if (dist_x.abs() > 1) || (dist_y.abs() > 1) {
                    // update tail if not neighbouring anymore
                    if head_pos.y > tail_pos.y {
                        tail_pos.y += 1;
                    } else if head_pos.y < tail_pos.y{
                        tail_pos.y -= 1;
                    }

                    if head_pos.x > tail_pos.x {
                        tail_pos.x += 1;
                    } else if head_pos.x < tail_pos.x {
                        tail_pos.x -= 1;
                    }

                }
            }

            visited_pos.insert(tail_pos);
        }


    }


    println!("visited {} unique positions with tail", visited_pos.len());
    //dbg!(visited_pos);

    Ok(())
}
