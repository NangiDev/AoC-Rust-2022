pub fn part_one(input: &str) -> Option<u32> {
    let mut visible_trees = 0;

    let trees = input
        .lines()
        .filter(|f| !f.is_empty())
        .map(|t| {
            t.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    for (y, row) in trees.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if x == 0 || y == 0 || x == row.len() - 1 || y == trees.len() - 1 {
                visible_trees += 1;
                continue;
            }

            let mut left = true;
            (0..x).rev().for_each(|li| {
                if row[li] >= *height {
                    left = false;
                }
            });

            let mut right = true;
            (x + 1..row.len()).for_each(|ri| {
                if row[ri] >= *height {
                    right = false;
                }
            });

            let mut up = true;
            (0..y).rev().for_each(|ui| {
                if trees[ui][x] >= *height {
                    up = false;
                }
            });

            let mut down = true;
            (y + 1..trees.len()).for_each(|di| {
                if trees[di][x] >= *height {
                    down = false;
                }
            });

            if left || right || up || down {
                visible_trees += 1;
            }
        }
    }

    Some(visible_trees)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut scenic_score = 0;

    let trees = input
        .lines()
        .filter(|f| !f.is_empty())
        .map(|t| {
            t.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    for (y, row) in trees.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if x == 0 || y == 0 || x == row.len() - 1 || y == trees.len() - 1 {
                continue;
            }

            let mut left = 0;
            for li in (0..x).rev() {
                left += 1;
                if row[li] >= *height {
                    break;
                }
            }

            let mut right = 0;
            for ri in x + 1..row.len() {
                right += 1;
                if row[ri] >= *height {
                    break;
                }
            }

            let mut up = 0;
            for ui in (0..y).rev() {
                up += 1;
                if trees[ui][x] >= *height {
                    break;
                }
            }

            let mut down = 0;
            for di in (y + 1..trees.len()) {
                down += 1;
                if trees[di][x] >= *height {
                    break;
                }
            }

            if (left * right * down * up) > scenic_score {
                scenic_score = left * right * down * up;
            }
        }
    }

    Some(scenic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
