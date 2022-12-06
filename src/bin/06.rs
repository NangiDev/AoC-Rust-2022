pub fn part_one(input: &str) -> Option<u32> {
    let signals: Vec<&str> = input.split('\n').filter(|f| !f.is_empty()).collect();

    let mut sum: usize = 0;

    let is_uniq = |interval: &str| -> bool {
        let mut uniq = true;
        for (i, c1) in interval.chars().enumerate() {
            for n in i + 1..interval.len() {
                if c1.eq(interval.chars().collect::<Vec<char>>().get(n).unwrap()) {
                    uniq = false;
                    break;
                }
            }
        }

        uniq
    };

    for signal in &signals {
        let marker_len = 4;
        for start in 0..signal.len() - marker_len - 1 {
            let interval = &signal[start..start + marker_len];
            if is_uniq(interval) {
                sum += start + marker_len;
                break;
            }
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let signals: Vec<&str> = input.split('\n').filter(|f| !f.is_empty()).collect();

    let mut sum: usize = 0;

    let is_uniq = |interval: &str| -> bool {
        let mut uniq = true;
        for (i, c1) in interval.chars().enumerate() {
            for n in i + 1..interval.len() {
                if c1.eq(interval.chars().collect::<Vec<char>>().get(n).unwrap()) {
                    uniq = false;
                    break;
                }
            }
        }

        uniq
    };

    for signal in &signals {
        let marker_len = 14;
        for start in 0..signal.len() - marker_len - 1 {
            let interval = &signal[start..start + marker_len];
            if is_uniq(interval) {
                sum += start + marker_len;
                break;
            }
        }
    }

    Some(sum as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
