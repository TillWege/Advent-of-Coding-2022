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
    let side_len = (trees.len() as f64).sqrt() as usize;

    let mut total_visible_outside = 0;
    let mut total_visible_top = 0;
    let mut total_visible_right = 0;
    let mut total_visible_bottom = 0;
    let mut total_visible_left = 0;

    for y in 0..side_len {
        'outer: for x in 0..side_len {
            // check for trees on the border (always visible)
            if (x == 0) || (x == (side_len - 1)) || (y == 0) || (y == (side_len - 1)) {
                total_visible_outside += 1;
            } else {
                let tree = trees[get_index(x, y, side_len)];
                let mut temp_tree;
                
                // ---------------
                // Bottom
                
                for new_y in (y + 1)..side_len {
                    temp_tree = trees[get_index(x, new_y, side_len)];
                    if temp_tree >= tree {
                        break;
                    }
                    if new_y == (side_len - 1) {
                        total_visible_bottom += 1;
                        continue 'outer;
                    }
                }

                // ---------------
                // Right
                for new_x_2 in (x + 1)..side_len {
                    temp_tree = trees[get_index(new_x_2, y, side_len)];
                    if temp_tree >= tree {
                        break;
                    }
                    if new_x_2 == (side_len - 1) {
                        total_visible_right += 1;
                        continue 'outer;
                    }
                }

                // ---------------
                // Top
                for new_y_2 in 0..y {
                    temp_tree = trees[get_index(x, new_y_2, side_len)];
                    if temp_tree >= tree {
                        break;
                    }
                    if new_y_2 == (y - 1) {
                        total_visible_top += 1;
                        continue 'outer;
                    }
                }

                // ---------------
                // Left
                for new_x in 0..x {
                    temp_tree = trees[get_index(new_x, y, side_len)];
                    if temp_tree >= tree {
                        break;
                    }
                    if new_x == (x - 1) {
                        total_visible_left += 1;
                        continue 'outer;
                    }
                }
            }
        }
    }
   
    println!("Total Visible Top: {}", total_visible_top);
    println!("Total Visible Bottom: {}", total_visible_bottom);
    println!("Total Visible Right: {}", total_visible_right);
    println!("Total Visible Left: {}", total_visible_left);
    println!("Total Visible Outside: {}", total_visible_outside);
    println!("Total Visible: {}", total_visible_top + total_visible_bottom + total_visible_right + total_visible_left + total_visible_outside);

    Ok(())
}

fn get_index(x: usize, y: usize, side_len: usize) -> usize {
    x + (y * side_len)
}
