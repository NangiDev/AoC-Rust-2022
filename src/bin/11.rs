#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug)]
struct Item {
    worry: u32,
    holder: usize,
}

impl Item {
    pub fn perform_op(&mut self, op: &Operation, op_num: &Option<u32>, divider: u32) {
        let op_num = op_num.unwrap_or(self.worry);
        match op {
            Operation::Add => self.worry = (self.worry + op_num) / divider,
            Operation::Mul => self.worry = (self.worry * op_num) / divider,
        }
    }

    pub fn throw_item_true(&self, div: &u32) -> bool {
        self.worry % div == 0
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    op: Operation,
    op_num: Option<u32>,
    div: u32,
    true_throw_to: usize,
    false_throw_to: usize,
    inspect: u32,
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            id: 0,
            op: Operation::Add,
            op_num: None,
            div: 0,
            true_throw_to: 0,
            false_throw_to: 0,
            inspect: 0,
        }
    }
}

impl Monkey {
    pub fn extract_items(&mut self, line: &str) -> Vec<Item> {
        line.split_whitespace()
            .map(|l| l.replace(',', ""))
            .filter_map(|l| l.parse::<u32>().ok())
            .map(|worry| Item {
                worry,
                holder: self.id,
            })
            .collect()
    }
    pub fn extract_op(&mut self, line: &str) {
        if line.contains('+') {
            self.op = Operation::Add;
        } else if line.contains('*') {
            self.op = Operation::Mul;
        }
        self.op_num = line
            .split_whitespace()
            .filter_map(|l| l.parse::<u32>().ok())
            .last()
    }

    pub fn extract_div(&mut self, line: &str) {
        self.div = line
            .split_whitespace()
            .filter_map(|l| l.parse::<u32>().ok())
            .last()
            .unwrap();
    }

    pub fn extract_throw_to_true(&mut self, line: &str) {
        self.true_throw_to = line
            .split_whitespace()
            .filter_map(|l| l.parse::<usize>().ok())
            .last()
            .unwrap();
    }

    pub fn extract_throw_to_false(&mut self, line: &str) {
        self.false_throw_to = line
            .split_whitespace()
            .filter_map(|l| l.parse::<usize>().ok())
            .last()
            .unwrap();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let step = 7;

    let mut items: Vec<Item> = Vec::new();

    let mut monkeys: Vec<Monkey> = Vec::new();

    for x in (0..lines.len()).step_by(step) {
        let mut monkey = Monkey::default();
        monkey.id = x / step as usize;

        for n in x + 1..x + step - 1 {
            let line = lines[n];

            if line.contains("Starting items:") {
                for item in monkey.extract_items(line) {
                    items.push(item);
                }
            } else if line.contains("Operation:") {
                monkey.extract_op(line);
            } else if line.contains("Test:") {
                monkey.extract_div(line);
            } else if line.contains("If true:") {
                monkey.extract_throw_to_true(line);
            } else if line.contains("If false:") {
                monkey.extract_throw_to_false(line);
            }
        }

        monkeys.push(monkey);
    }

    for _i in 0..20 {
        for monkey in monkeys.iter_mut() {
            for item in items.iter_mut() {
                if monkey.id == item.holder {
                    monkey.inspect += 1;
                    item.perform_op(&monkey.op, &monkey.op_num, 3);
                    if item.throw_item_true(&monkey.div) {
                        item.holder = monkey.true_throw_to;
                    } else {
                        item.holder = monkey.false_throw_to;
                    }
                }
            }
        }
    }

    monkeys.sort_by(|c1, c2| c2.inspect.partial_cmp(&c1.inspect).unwrap());

    let mut result = 1;
    for i in 0..2 {
        result *= monkeys[i].inspect;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let step = 7;

    let mut items: Vec<Item> = Vec::new();

    let mut monkeys: Vec<Monkey> = Vec::new();

    for x in (0..lines.len()).step_by(step) {
        let mut monkey = Monkey::default();
        monkey.id = x / step as usize;

        for n in x + 1..x + step - 1 {
            let line = lines[n];

            if line.contains("Starting items:") {
                for item in monkey.extract_items(line) {
                    items.push(item);
                }
            } else if line.contains("Operation:") {
                monkey.extract_op(line);
            } else if line.contains("Test:") {
                monkey.extract_div(line);
            } else if line.contains("If true:") {
                monkey.extract_throw_to_true(line);
            } else if line.contains("If false:") {
                monkey.extract_throw_to_false(line);
            }
        }

        monkeys.push(monkey);
    }

    for _i in 0..1000 {
        for monkey in monkeys.iter_mut() {
            for item in items.iter_mut() {
                if monkey.id == item.holder {
                    monkey.inspect += 1;
                    item.perform_op(&monkey.op, &monkey.op_num, 1);
                    if item.throw_item_true(&monkey.div) {
                        item.holder = monkey.true_throw_to;
                    } else {
                        item.holder = monkey.false_throw_to;
                    }
                }
            }
        }
    }

    monkeys.sort_by(|c1, c2| c2.inspect.partial_cmp(&c1.inspect).unwrap());

    let mut result = 1;
    for i in 0..2 {
        result *= monkeys[i].inspect;
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
