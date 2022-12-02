pub fn part_one(input: &str) -> Option<u32> {
    let turns: Vec<&str> = input.split('\n').collect();

    let mut score:u32 = 0;

    for turn in &turns {
        let c: Vec<&str> = turn.split(' ').collect();
        let op: &str = c.first().unwrap();
        let me: &str = c.last().unwrap();

        if me.eq("X") {
            score += 1;
            if op.eq("A") {
                score += 3;
            } else if op.eq("B") {
                score += 0;
            } else if op.eq("C") {
                score += 6;
            }
        }
        else if me.eq("Y") {
            score += 2;
            if op.eq("A") {
                score += 6;
            } else if op.eq("B") {
                score += 3;
            } else if op.eq("C") {
                score += 0;
            }
        }
        else if me.eq("Z") {
            score += 3;
            if op.eq("A") {
                score += 0;
            } else if op.eq("B") {
                score += 6;
            } else if op.eq("C") {
                score += 3;
            }
        }
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let turns: Vec<&str> = input.split('\n').collect();

    let mut score:u32 = 0;

    for turn in &turns {
        let c: Vec<&str> = turn.split(' ').collect();
        let op: &str = c.first().unwrap();
        let me: &str = c.last().unwrap();

        if me.eq("X") {
            if op.eq("A") {
                score += 3;
            } else if op.eq("B") {
                score += 1;
            } else if op.eq("C") {
                score += 2;
            }
        }
        else if me.eq("Y") {
            if op.eq("A") {
                score += 1+3;
            } else if op.eq("B") {
                score += 2+3;
            } else if op.eq("C") {
                score += 3+3;
            }
        }
        else if me.eq("Z") {
            if op.eq("A") {
                score += 2+6;
            } else if op.eq("B") {
                score += 3+6;
            } else if op.eq("C") {
                score += 1+6;
            }
        }
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
