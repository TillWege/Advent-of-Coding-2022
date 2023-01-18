use std::vec;

#[derive(Debug)]
enum Operator {
    Add,
    Mul
}

#[derive(Debug)]
enum Operand {
    OldValue,
    Number(usize)
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operator: Operator,
    operand: Operand,
    test_div: usize,
    true_target: usize,
    false_target: usize,
    inspect_count: usize,
}

impl Monkey {
    fn inspect_item(&mut self, item_id: usize) {
        match self.operator {
            Operator::Add => {
                match self.operand {
                    Operand::OldValue => self.items[item_id] += self.items[item_id],
                    Operand::Number(num) => self.items[item_id] += num,
                }
            },
            Operator::Mul => {
                match self.operand {
                    Operand::OldValue => self.items[item_id] *= self.items[item_id],
                    Operand::Number(num) => self.items[item_id] *= num,
                }
            }
        };
        //self.items[item_id] /= 3;
        self.inspect_count += 1;
    }

    fn check_worry_level(&mut self, item_id: usize) -> (usize, usize) {
        let item = self.items.remove(item_id);
        if item % self.test_div == 0 {
            return (self.true_target, item);
        } else {
            return (self.false_target, item);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day11.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;

    let mut lines = contents.lines().filter(|s| !s.is_empty()).peekable();

    let mut monkeys: Vec<Monkey> = vec![];

    while lines.peek().is_some() {
        let _monkey_number = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .replace(":", "")
            .parse::<usize>()
            .unwrap();
        let items = lines
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .to_string()
            .replace(",", "")
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut op_text = lines.next().unwrap().split("old ").last().unwrap().split_whitespace();
        let operator = match op_text.next().unwrap() {
            "+" => Operator::Add,
            "*" => Operator::Mul,
            _ => panic!("Unknown operand"),
        };

        let operand = match op_text.next().unwrap() {
            "old" => Operand::OldValue,
            val => Operand::Number(val.parse::<usize>().unwrap()),
        };
        
        let test_div = lines
            .next()
            .unwrap()
            .split("by ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let true_target = lines
            .next()
            .unwrap()
            .split("monkey ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let false_target = lines
            .next()
            .unwrap()
            .split("monkey ")
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let new_monkey = Monkey {
            items,
            operator,
            operand,
            test_div,
            true_target,
            false_target,
            inspect_count: 0,
        };

        monkeys.push(new_monkey);
    }

    let monkey_count = monkeys.len();

    
    let mut test_divs: Vec<usize> = vec![];
    for monkey in &monkeys {
        test_divs.push(monkey.test_div);
    }
    let mut test_div_lcm = test_divs[0];
    for i in 1..test_divs.len() {
        test_div_lcm *= test_divs[i];
    }
    
    for _ in 1..=10000 {
        for i in 0..monkey_count {
            let monkey = monkeys.get_mut(i).unwrap();
            let mut task_q: Vec<(usize, usize)> = vec![];
                
            for item_id in (0..monkey.items.len()).rev() {
                monkey.inspect_item(item_id);
                task_q.push(monkey.check_worry_level(item_id));
            }

            for (target, item) in task_q {
                let new_item = item % test_div_lcm;
                monkeys.get_mut(target).unwrap().items.push(new_item);
            }
        }
        print!("");
    }


    let mut monkey_activity: Vec<usize> = vec![];
    for monkey in &monkeys {
        monkey_activity.push(monkey.inspect_count);
    }

    monkey_activity.sort();

    let val1 = monkey_activity.pop().unwrap();
    let val2 = monkey_activity.pop().unwrap();

    let monkey_buisness =  val1 * val2;

    for i in 0..monkey_count {
        println!("Monkey {} inspected {} items", i, monkeys[i].inspect_count);
    }

    dbg!(monkey_buisness);

    Ok(())
}