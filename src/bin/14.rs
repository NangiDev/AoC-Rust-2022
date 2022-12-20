#[derive(Debug)]
enum Node {
    Line((u32, u32), (u32, u32)),
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut max_y = 0;
    let mut taken_nodes: Vec<(u32, u32)> = Vec::new();
    for line in input.lines() {
        let coord = line.split(" -> ").collect::<Vec<&str>>();
        let coord = coord
            .iter()
            .map(|c| {
                c.split(',')
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        let mut coord = coord
            .windows(2)
            .map(|win| Node::Line((win[0][0], win[0][1]), (win[1][0], win[1][1])))
            .collect::<Vec<Node>>();

        for Node::Line(start, end) in &mut coord {
            if start.1 > max_y {
                max_y = start.1;
            }
            taken_nodes.push(*start);
            while start != end {
                if start.0 == end.0 {
                    if start.1 < end.1 {
                        start.1 += 1;
                    } else {
                        start.1 -= 1;
                    }
                } else if start.1 == end.1 {
                    if start.0 < end.0 {
                        start.0 += 1;
                    } else {
                        start.0 -= 1;
                    }
                }
                taken_nodes.push(*start);
            }
        }
    }

    let start_point = (500, 0);
    let mut sand = start_point.clone();

    let mut stone_count = 0;
    loop {
        if sand.1 >= max_y {
            break;
        }

        if !taken_nodes.contains(&(sand.0, sand.1 + 1)) {
            sand.1 += 1;
            continue;
        } else if !taken_nodes.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand.0 -= 1;
            sand.1 += 1;
            continue;
        } else if !taken_nodes.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand.0 += 1;
            sand.1 += 1;
            continue;
        } else {
            stone_count += 1;
            taken_nodes.push(sand);
            sand = start_point.clone();
        }
    }

    Some(stone_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut max_y = 0;
    let mut taken_nodes: Vec<(u32, u32)> = Vec::new();
    for line in input.lines() {
        let coord = line.split(" -> ").collect::<Vec<&str>>();
        let coord = coord
            .iter()
            .map(|c| {
                c.split(',')
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        let mut coord = coord
            .windows(2)
            .map(|win| Node::Line((win[0][0], win[0][1]), (win[1][0], win[1][1])))
            .collect::<Vec<Node>>();

        for Node::Line(start, end) in &mut coord {
            if start.1 > max_y {
                max_y = start.1;
            }
            taken_nodes.push(*start);
            while start != end {
                if start.0 == end.0 {
                    if start.1 < end.1 {
                        start.1 += 1;
                    } else {
                        start.1 -= 1;
                    }
                } else if start.1 == end.1 {
                    if start.0 < end.0 {
                        start.0 += 1;
                    } else {
                        start.0 -= 1;
                    }
                }
                taken_nodes.push(*start);
            }
        }
    }

    let start_point = (500, 0);
    let mut sand = start_point.clone();

    let mut stone_count = 0;
    loop {
        if sand.1 == max_y + 1 {
            stone_count += 1;
            taken_nodes.push(sand);
            sand = start_point.clone();
            continue;
        }

        if !taken_nodes.contains(&(sand.0, sand.1 + 1)) {
            sand.1 += 1;
            continue;
        } else if !taken_nodes.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand.0 -= 1;
            sand.1 += 1;
            continue;
        } else if !taken_nodes.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand.0 += 1;
            sand.1 += 1;
            continue;
        } else {
            stone_count += 1;
            taken_nodes.push(sand);
            if sand == start_point {
                break;
            }
            sand = start_point.clone();
        }
    }

    Some(stone_count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
