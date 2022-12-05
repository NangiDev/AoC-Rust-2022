fn action_9000(stacks: &mut Vec<Vec<String>>, count: u32, from: usize, to: usize) {
    for _i in 0..count {
        let val = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(val);
    }
}

fn action_9001(stacks: &mut Vec<Vec<String>>, count: u32, from: usize, to: usize) {
    let mut temp_stack: Vec<String> = Vec::new();

    for _i in 0..count {
        let val = stacks[from - 1].pop().unwrap();
        temp_stack.push(val);
    }

    temp_stack.reverse();

    for ele in temp_stack {
        stacks[to - 1].push(ele);
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut stack: Vec<String> = input
        .split('\n')
        .filter(|f| f.contains('['))
        .map(|f| f.replace("   ", ""))
        .map(|f| f.replace('[', ""))
        .map(|f| f.replace(']', ""))
        .map(|f| f.replace(' ', ","))
        .collect();

    let ops: Vec<String> = input
        .split('\n')
        .filter(|f| f.contains("move"))
        .map(|f| f.replace("move ", ""))
        .map(|f| f.replace("from ", ""))
        .map(|f| f.replace("to ", ""))
        .map(|f| f.replace(' ', ","))
        .collect();

    stack.reverse();

    let mut stacks: Vec<Vec<String>> = Vec::new();
    for _ele in stack.first().unwrap().chars().filter(|f| f.is_alphabetic()) {
        stacks.push(Vec::new());
    }

    for row in &stack {
        let mut col_i = 0;
        let r: Vec<&str> = row.split(',').collect();
        for c in r {
            if c.is_empty() {
                col_i += 1;
                continue;
            }
            stacks[col_i].push(String::from(c));
            col_i += 1;
        }
    }

    for op in &ops {
        let o: Vec<usize> = op
            .split(',')
            .filter_map(|f| f.parse::<usize>().ok())
            .collect();

        action_9000(&mut stacks, o[0] as u32, o[1], o[2]);
    }

    let mut result = String::from("");

    for s in &stacks {
        let c = *s
            .last()
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
            .first()
            .unwrap();
        result.push(c);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stack: Vec<String> = input
        .split('\n')
        .filter(|f| f.contains('['))
        .map(|f| f.replace("   ", ""))
        .map(|f| f.replace('[', ""))
        .map(|f| f.replace(']', ""))
        .map(|f| f.replace(' ', ","))
        .collect();

    let ops: Vec<String> = input
        .split('\n')
        .filter(|f| f.contains("move"))
        .map(|f| f.replace("move ", ""))
        .map(|f| f.replace("from ", ""))
        .map(|f| f.replace("to ", ""))
        .map(|f| f.replace(' ', ","))
        .collect();

    stack.reverse();

    let mut stacks: Vec<Vec<String>> = Vec::new();
    for _ele in stack.first().unwrap().chars().filter(|f| f.is_alphabetic()) {
        stacks.push(Vec::new());
    }

    for row in &stack {
        let mut col_i = 0;
        let r: Vec<&str> = row.split(',').collect();
        for c in r {
            if c.is_empty() {
                col_i += 1;
                continue;
            }
            stacks[col_i].push(String::from(c));
            col_i += 1;
        }
    }

    for op in &ops {
        let o: Vec<usize> = op
            .split(',')
            .filter_map(|f| f.parse::<usize>().ok())
            .collect();

        action_9001(&mut stacks, o[0] as u32, o[1], o[2]);
    }

    let mut result = String::from("");

    for s in &stacks {
        let c = *s
            .last()
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
            .first()
            .unwrap();
        result.push(c);
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }

    #[test]
    fn action_part_one() {
        let mut expected: Vec<Vec<String>> = Vec::new();
        let mut stacks: Vec<Vec<String>> = Vec::new();

        let col1: Vec<String> = vec![String::from('Z'), String::from('N')];
        let col2: Vec<String> = vec![String::from('M'), String::from('C'), String::from('D')];
        let col3: Vec<String> = vec![String::from('P')];

        stacks.push(col1);
        stacks.push(col2);
        stacks.push(col3);

        let col1: Vec<String> = vec![String::from('Z'), String::from('N'), String::from('D')];
        let col2: Vec<String> = vec![String::from('M'), String::from('C')];
        let col3: Vec<String> = vec![String::from('P')];

        expected.push(col1);
        expected.push(col2);
        expected.push(col3);

        let count = 1;
        let from = 2;
        let to = 1;
        action_9000(&mut stacks, count, from, to);
        assert_eq!(stacks, expected);
    }

    #[test]
    fn action_part_two() {
        let mut expected: Vec<Vec<String>> = Vec::new();
        let mut stacks: Vec<Vec<String>> = Vec::new();

        let col1: Vec<String> = vec![String::from('Z'), String::from('N')];
        let col2: Vec<String> = vec![String::from('M'), String::from('C'), String::from('D')];
        let col3: Vec<String> = vec![String::from('P')];

        stacks.push(col1);
        stacks.push(col2);
        stacks.push(col3);

        let col1: Vec<String> = vec![String::from('Z'), String::from('N')];
        let col2: Vec<String> = vec![String::from('M')];
        let col3: Vec<String> = vec![String::from('P'), String::from('C'), String::from('D')];

        expected.push(col1);
        expected.push(col2);
        expected.push(col3);

        let count = 2;
        let from = 2;
        let to = 3;
        action_9001(&mut stacks, count, from, to);
        assert_eq!(stacks, expected);
    }
}
