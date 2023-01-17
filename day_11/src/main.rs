use std::vec;

struct Monkey {
    items: Vec<usize>,
    operation: fn(usize) -> usize,
    test_div: usize,
    true_target: usize,
    false_target: usize
}

impl Monkey {
    fn inspect_items(&mut self) {
        for mut item in &self.items {
            item = &(self.operation)(*item);
        }
    }

    fn dec_worry(&mut self) {

    }

    fn check_worry_levels(&mut self, monkeys: &mut Vec<Monkey>) {

    }
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day11.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;

    let mut monkeys = vec![
        Monkey {
            items: vec![85, 79, 63, 72],
            operation: |a: usize| return a * 17,
            test_div: 2,
            true_target: 2,
            false_target: 6  
        },
        Monkey {
            items: vec![53, 94, 65, 81, 93, 73, 57, 92],
            operation: |a: usize| return a * a,
            test_div: 7,
            true_target: 0,
            false_target: 2  
        },
        Monkey {
            items: vec![62, 63],
            operation: |a: usize| return a + 7,
            test_div: 13,
            true_target: 7,
            false_target: 6  
        },
        Monkey {
            items: vec![57, 92, 56],
            operation: |a: usize| return a + 4,
            test_div: 5,
            true_target: 4,
            false_target: 5  
        },
        Monkey {
            items: vec![67],
            operation: |a: usize| return a +5,
            test_div: 3,
            true_target: 1,
            false_target: 5  
        },
        Monkey {
            items: vec![85, 56, 66, 72, 57, 99],
            operation: |a: usize| return a + 6,
            test_div: 19,
            true_target: 1,
            false_target: 0  
        },
        Monkey {
            items: vec![86, 65, 98, 97, 69],
            operation: |a: usize| return a * 13,
            test_div: 1,
            true_target: 3,
            false_target: 7  
        },
        Monkey {
            items: vec![87, 68, 92, 66, 91, 50, 68],
            operation: |a: usize| return a + 2,
            test_div: 17,
            true_target: 4,
            false_target: 3  
        },
    ];

    
    for mut monkey in monkeys {
        monkey.inspect_items();
        monkey.dec_worry();
        //monkey.check_worry_levels(&mut monkeys);
    }


    Ok(())

}
