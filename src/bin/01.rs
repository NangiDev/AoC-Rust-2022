pub fn part_one(input: &str) -> Option<u32> {
    let calories: Vec<&str> = input.split('\n').collect();

    let mut sum_cal: Vec<u32> = Vec::new();
    let mut tmp_sum = 0;
    
    for c in &calories {
        if c.is_empty() {
            sum_cal.push(tmp_sum);
            tmp_sum = 0;
            continue;
        }

        tmp_sum += c.parse::<u32>().unwrap();
    }

    Some(*sum_cal.iter().max().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let calories: Vec<&str> = input.split('\n').collect();

    let mut sum_cal: Vec<u32> = Vec::new();
    let mut tmp_sum = 0;
    
    for c in &calories {
        if c.is_empty() {
            sum_cal.push(tmp_sum);
            tmp_sum = 0;
            continue;
        }

        tmp_sum += c.parse::<u32>().unwrap();
    }

    sum_cal.sort();
    sum_cal.reverse();

    Some(sum_cal[..3].iter().sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
