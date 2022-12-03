use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks: Vec<(&str, &str)> = input
        .split('\n')
        .filter(|e| !e.is_empty())
        .map(|e| e.split_at(e.len() / 2))
        .collect();

    let mut letters: HashMap<char, u32> = HashMap::new();
    let alphabets = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let mut i = 1;

    for c in alphabets.chars() {
        letters.insert(c, i);
        i += 1;
    }

    let mut prio: u32 = 0;

    for c in &rucksacks {
        for l in c.0.chars() {
            let mut found = false;

            for r in c.1.chars() {
                if l.eq(&r) {
                    prio += letters.get(&l).unwrap();
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }
    }

    Some(prio)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<&str> = input.split('\n').filter(|e| !e.is_empty()).collect();
    let mut letters: HashMap<char, u32> = HashMap::new();
    let alphabets = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let mut i = 1;

    for c in alphabets.chars() {
        letters.insert(c, i);
        i += 1;
    }

    let mut prio: u32 = 0;

    for x in (0..rucksacks.len()).step_by(3) {
        let first = *rucksacks.get(x).unwrap();
        let second = *rucksacks.get(x + 1).unwrap();
        let third = *rucksacks.get(x + 2).unwrap();

        for f in first.chars() {
            let mut found = false;
            for s in second.chars() {
                for t in third.chars() {
                    if f.eq(&s) && s.eq(&t) {
                        prio += letters.get(&f).unwrap();
                        found = true;
                        break;
                    }
                }

                if found {
                    break;
                }
            }

            if found {
                break;
            }
        }
    }

    Some(prio)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
