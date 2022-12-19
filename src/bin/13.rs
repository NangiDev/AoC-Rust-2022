use std::{cmp::Ordering, str::Chars};

#[derive(Debug)]
enum Val {
    Num(i32),
    List(Vec<Self>),
}

impl Val {
    fn parse(s: &str) -> Self {
        let mut c = s.chars();
        if c.next().unwrap() != '[' {
            panic!("bad input");
        }
        Self::parse_into(&mut c)
    }

    fn parse_into(c: &mut Chars) -> Self {
        let mut result = Vec::new();
        let mut num = -1;

        while let Some(ch) = c.next() {
            match ch {
                '[' => result.push(Self::parse_into(c)),
                ',' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                        num = -1;
                    }
                }
                ']' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                    }
                    return Self::List(result);
                }
                '0'..='9' => {
                    if num == -1 {
                        num = (ch as u8 - b'0') as i32;
                    } else {
                        num = (num * 10) + (ch as u8 - b'0') as i32;
                    }
                }
                _ => panic!("bad char '{ch}'"),
            }
        }
        Self::List(result)
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Val::List(left), Val::List(right)) => {
                let mut idx = 0;
                loop {
                    if left.len() <= idx || right.len() <= idx {
                        if left.len() < right.len() {
                            return Ordering::Less;
                        } else if left.len() == right.len() {
                            return Ordering::Equal;
                        } else {
                            return Ordering::Greater;
                        }
                    }
                    match (&left[idx], &right[idx]) {
                        (Val::Num(l), Val::Num(r)) => {
                            if l < r {
                                return Ordering::Less;
                            } else if l > r {
                                return Ordering::Greater;
                            }
                        }
                        (Val::List(_), Val::Num(r)) => {
                            let check = left[idx].compare(&Val::List(vec![Val::Num(*r)]));
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Val::Num(l), Val::List(_)) => {
                            let check = Val::List(vec![Val::Num(*l)]).compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Val::List(_), Val::List(_)) => {
                            let check = left[idx].compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                    }
                    idx += 1;
                }
            }
            _ => panic!("bad input"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pairs: Vec<(Val, Val)> = Vec::new();

    for line in input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .chunks(2)
    {
        let left = Val::parse(&line[0]);
        let right = Val::parse(&line[1]);

        pairs.push((left, right));
    }

    let mut sum = 0;
    for (index, pair) in pairs.iter().enumerate() {
        if pair.0.compare(&pair.1) == Ordering::Less {
            sum += index + 1;
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

// Over 2456, under 5088,5089,5176,5761,5793
fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
