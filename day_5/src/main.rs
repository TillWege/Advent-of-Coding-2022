use std::{fs::File, io::{Read}, vec, collections::VecDeque};

fn main() -> std::io::Result<()> {
    let mut file = File::open("./inputs/day5.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut stacks = vec![
        vec!['N','B','D','T','V','G','Z','J'],
        vec!['S','R','M','D','W','P','F'],
        vec!['V','C','R','S','Z'],
        vec!['R','T','J','Z','P','H','G'],
        vec!['T','C','J','N','D','Z','Q','F'],
        vec!['N','V','P','W','G','S','F','M'],
        vec!['G','C','V','B','P','Q'],
        vec!['Z','B','P','N'],
        vec!['W','P','J']
    ];

    let mut stacks_2 = stacks.clone();

    println!("Top Elements: ");
    for (ind, stack, ) in stacks.iter().enumerate() {
        println!("Stack {}: {}", ind, stack.last().unwrap());
    }

    for line in contents.lines() {
        if !line.starts_with("move") {continue;}

        let mut parts = line.split_whitespace();
        parts.next();
        let move_count_opt = parts.next();
        parts.next();
        let move_origin_opt = parts.next();
        parts.next();
        let move_target_opt = parts.next();

        if move_count_opt.is_some() && move_origin_opt.is_some() && move_target_opt.is_some() {
            let move_count: usize = move_count_opt.unwrap().parse().unwrap();
            let move_origin: usize = move_origin_opt.unwrap().parse().unwrap();
            let move_target: usize = move_target_opt.unwrap().parse().unwrap();
 
            let mut temp_stack: VecDeque<char> = VecDeque::new();
            let mut temp_stack_2: VecDeque<char> = VecDeque::new();
                
            for _ in 0..move_count {
                temp_stack.push_front(stacks[move_origin - 1].pop().unwrap());
                temp_stack_2.push_front(stacks_2[move_origin - 1].pop().unwrap());
            }

            for _ in 0..move_count {
                stacks[move_target - 1].push(temp_stack.pop_back().unwrap());
                stacks_2[move_target - 1].push(temp_stack_2.pop_front().unwrap());
            }

        }else {
            println!("Shit happend at line: {}", line);
        }


    }

    let mut res_1 = String::from("");
    println!("Top Elements: ");
    for (ind, stack, ) in stacks.iter().enumerate() {
        let last_elem = stack.last().unwrap();
        res_1.push(last_elem.clone());
        println!("Stack {}: {}", ind, last_elem);
    }
    println!("Top Container: {}", res_1);

    let mut res_2 = String::from("");
    println!("Top Elements: ");
    for (ind, stack, ) in stacks_2.iter().enumerate() {
        let last_elem = stack.last().unwrap();
        res_2.push(last_elem.clone());
        println!("Stack {}: {}", ind, last_elem);
    }
    println!("Top Container: {}", res_2);

    Ok(())

}