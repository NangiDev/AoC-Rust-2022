use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<u32> {
    let pairs: Vec<&str> = input.split('\n').filter(|e| !e.is_empty()).collect();
    let mut num_pairs = 0;

    for p in &pairs {
        let i = p.find(',').unwrap();
        let l = &p[0..i];
        let r = &p[i+1..];

        let l_i = l.find('-').unwrap();
        let l_low = l[0..l_i].parse::<u32>().unwrap();
        let l_high = l[l_i+1..].parse::<u32>().unwrap();

        let r_i = r.find('-').unwrap();
        let r_low = r[0..r_i].parse::<u32>().unwrap();
        let r_high = r[r_i+1..].parse::<u32>().unwrap();

        match l_low.cmp(&r_low) {
            Ordering::Greater => {
                if l_high <= r_high {
                    num_pairs += 1;
                }
            }
            Ordering::Less => {
                if l_high >= r_high {
                    num_pairs += 1;
                }
            }
            Ordering::Equal => num_pairs += 1,
        }
    }

    Some(num_pairs)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs: Vec<&str> = input.split('\n').filter(|e| !e.is_empty()).collect();
    let mut num_pairs = 0;

    for p in &pairs {
        let i = p.find(',').unwrap();
        let l = &p[0..i];
        let r = &p[i+1..];

        let l_i = l.find('-').unwrap();
        let l_low = l[0..l_i].parse::<u32>().unwrap();
        let l_high = l[l_i+1..].parse::<u32>().unwrap();

        let r_i = r.find('-').unwrap();
        let r_low = r[0..r_i].parse::<u32>().unwrap();
        let r_high = r[r_i+1..].parse::<u32>().unwrap();

        match l_low.cmp(&r_low) {
            Ordering::Greater => {
                if r_high >= l_low {
                    num_pairs += 1;
                }
            }
            Ordering::Less => {
                if l_high >= r_low {
                    num_pairs += 1;
                }
            }
            Ordering::Equal => num_pairs += 1,
        }
    }
    Some(num_pairs)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
