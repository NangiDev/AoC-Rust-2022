use std::collections::BTreeMap;

fn find_shortest(input: &str, start: (usize, usize)) -> u32 {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut end: (usize, usize) = (0, 0);

    for (row, line) in input.lines().enumerate() {
        let mut grid_line = line.chars().map(|c| c as u8).collect::<Vec<u8>>();
        if let Some(start_point) = grid_line.iter().position(|&p| p == b'S') {
            grid_line[start_point] = b'a';
        }
        if let Some(end_point) = grid_line.iter().position(|&p| p == b'E') {
            end = (row, end_point);
            grid_line[end_point] = b'z';
        }
        grid.push(grid_line);
    }

    let width = grid[0].len();
    let height = grid.len();

    let mut shortest: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    shortest.insert(start, 0);

    let get_sorrounding_points =
        |pos: (usize, usize), width: usize, height: usize| -> Vec<(usize, usize)> {
            let ipos = (pos.0 as i32, pos.1 as i32);
            let width = width as i32;
            let height = height as i32;
            _DIR.iter()
                .map(|d| (ipos.0 + d.0, ipos.1 + d.1))
                .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < height && pos.1 < width)
                .map(|pos| (pos.0 as usize, pos.1 as usize))
                .collect()
        };

    let mut to_visit: Vec<(usize, usize)> = Vec::new();
    to_visit.extend(get_sorrounding_points(start, width, height));
    while let Some(loc) = to_visit.pop() {
        let cur_elevation = grid[loc.0][loc.1];
        let points = get_sorrounding_points(loc, width, height);
        let valid = points
            .iter()
            .filter(|pos| grid[pos.0][pos.1] + 1 >= cur_elevation)
            .map(|pos| *pos)
            .collect::<Vec<(usize, usize)>>();

        let new_path_dist = valid.iter().filter_map(|pos| shortest.get(pos)).min();
        if new_path_dist.is_none() {
            continue;
        }

        let new_path_dist = new_path_dist.unwrap() + 1;

        let cur_dist_path = shortest.entry(loc).or_insert(usize::MAX);
        if *cur_dist_path > new_path_dist {
            *cur_dist_path = new_path_dist;
            to_visit.extend(points.iter());
        }
    }

    *shortest.get(&end).unwrap_or(&usize::MAX) as u32
}

const _DIR: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
pub fn part_one(input: &str) -> Option<u32> {
    let mut start: (usize, usize) = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (row, col);
            }
        }
    }

    Some(find_shortest(input, start))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start_points: Vec<(usize, usize)> = Vec::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch as u8 == b'a' || ch as u8 == b'S' {
                start_points.push((row, col));
            }
        }
    }

    let mut shortest = u32::MAX;
    for start in start_points {
        let new_short = find_shortest(input, start);
        if new_short < shortest {
            shortest = new_short;
        }
    }

    Some(shortest)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
