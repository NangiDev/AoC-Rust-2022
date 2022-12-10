pub fn part_one(input: &str) -> Option<i32> {
    let instructions: Vec<(&str, i32)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            if l.eq("noop") {
                return ("noop", -1);
            }
            let temp: Vec<&str> = l.split(' ').collect();
            (
                temp.first().unwrap(),
                temp.last().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut value = 0;
    let mut register = 1;
    let mut cycle = 0;
    let mut mark = 20;

    for inst in &instructions {
        match inst.0 {
            "noop" => {
                cycle += 1;
                if cycle == mark {
                    value += register * mark;
                    mark += 40;
                }
            }
            "addx" => {
                cycle += 1;
                if cycle == mark {
                    value += register * mark;
                    mark += 40;
                }
                cycle += 1;
                if cycle == mark {
                    value += register * mark;
                    mark += 40;
                }
                register += inst.1;
            }
            _ => {}
        }
    }

    Some(value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions: Vec<(&str, i32)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            if l.eq("noop") {
                return ("noop", -1);
            }
            let temp: Vec<&str> = l.split(' ').collect();
            (
                temp.first().unwrap(),
                temp.last().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    let tick = |c: &mut i32, pos: &i32, screen: &mut Vec<char>| {
        let left: i32 = *pos - 1;
        let mid: i32 = *pos;
        let right: i32 = *pos + 1;

        let cyc = *c % 40;

        if left == cyc || mid == cyc || right == cyc {
            screen.insert(*c as usize, '#');
        }

        *c += 1;
    };

    let mut crt: Vec<char> = Vec::new();

    for _y in 0..(6 * 40) {
        crt.push(' ');
    }

    let mut _sprite_pos: i32 = 1;
    let mut cycle: i32 = 0;

    for inst in &instructions {
        match inst.0 {
            "noop" => {
                tick(&mut cycle, &_sprite_pos, &mut crt);
            }
            "addx" => {
                tick(&mut cycle, &_sprite_pos, &mut crt);
                tick(&mut cycle, &_sprite_pos, &mut crt);
                _sprite_pos += inst.1;
            }
            _ => {}
        }
    }

    for y in 0..6 {
        let mut line = String::new();
        for x in 0..40 {
            line.push(crt[(40 * y) + x]);
        }
        println!("{}", line);
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        part_two(&input);
    }
}
