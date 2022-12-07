use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let terminal: Vec<&str> = input.split('\n').filter(|t| !t.is_empty()).collect();

    let mut dir_tree: Vec<&str> = Vec::new();
    let mut map: HashMap<String, u32> = HashMap::new();

    for t_line in &terminal {
        if t_line.contains('$') {
            if t_line.contains("cd") {
                let cd_dir = t_line.split(' ').last().unwrap();
                if cd_dir.contains("..") {
                    dir_tree.pop();
                } else {
                    dir_tree.push(cd_dir);
                }
            } else {
                continue;
            }
            map.entry(dir_tree.concat()).or_insert(0);
            continue;
        }

        if t_line.contains("dir") {
            continue;
        }

        let size: Vec<u32> = t_line
            .split(' ')
            .filter_map(|t| t.parse::<u32>().ok())
            .collect();
        for end in 1..=dir_tree.len() {
            let i = dir_tree[0..end].concat();
            let mut v = *map.get(&i).unwrap();
            v += size.first().unwrap();
            map.insert(i, v);
        }
    }

    let mut result = 0;
    let cap = 100_000;
    for (_k, v) in map {
        if v <= cap {
            result += v;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let terminal: Vec<&str> = input.split('\n').filter(|t| !t.is_empty()).collect();

    let mut dir_tree: Vec<&str> = Vec::new();
    let mut map: HashMap<String, u32> = HashMap::new();

    for t_line in &terminal {
        if t_line.contains('$') {
            if t_line.contains("cd") {
                let cd_dir = t_line.split(' ').last().unwrap();
                if cd_dir.contains("..") {
                    dir_tree.pop();
                } else {
                    dir_tree.push(cd_dir);
                }
            } else {
                continue;
            }
            map.entry(dir_tree.concat()).or_insert(0);
            continue;
        }

        if t_line.contains("dir") {
            continue;
        }

        let size: Vec<u32> = t_line
            .split(' ')
            .filter_map(|t| t.parse::<u32>().ok())
            .collect();
        for end in 1..=dir_tree.len() {
            let i = dir_tree[0..end].concat();
            let mut v = *map.get(&i).unwrap();
            v += size.first().unwrap();
            map.insert(i, v);
        }
    }

    let tot_space = 70_000_000;
    let space_used = *map.get("/").unwrap();
    let space_left = tot_space - space_used;
    let space_needed = 30_000_000;

    let mut smallest = u32::MAX;
    for (_k, v) in map {
        if space_left + v >= space_needed && v < smallest {
            smallest = v;
        }
    }

    Some(smallest)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
