fn parse_input(input: &str) -> Vec<(char, usize)> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut split = l.split(' ');
            (
                split.next().unwrap().parse::<char>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn build_grid(actions: &Vec<(char, usize)>) -> (Vec<Vec<char>>, Knot) {
    let origin = 100;
    let mut head: (usize, usize) = (origin, origin);
    let mut grid: Vec<Vec<char>> = Vec::new();

    for (dir, step) in actions {
        for _i in 0..*step {
            match dir {
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                _ => {}
            }

            while grid.len() <= head.0 || grid.len() <= head.1 {
                // Push rows
                grid.push(Vec::new());

                for row in &mut grid {
                    // Push cols
                    while row.len() <= head.1 || row.len() <= head.0 {
                        row.push('.');
                    }
                }
            }
        }
    }
    (
        grid,
        Knot {
            x: origin,
            y: origin,
        },
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Knot {
    x: usize,
    y: usize,
}

impl Knot {
    pub fn move_closer_to(&mut self, parent: &Knot) {
        let (dist_x, dist_y) = (
            self.x as i32 - parent.x as i32,
            self.y as i32 - parent.y as i32,
        );

        if dist_x.abs() > 1 {
            if dist_x > 0 {
                self.x -= 1;
            } else {
                self.x += 1;
            }

            if dist_y.abs() == 1 {
                if dist_y > 0 {
                    self.y -= 1;
                } else {
                    self.y += 1;
                }
            }
        }

        if dist_y.abs() > 1 {
            if dist_y > 0 {
                self.y -= 1;
            } else {
                self.y += 1;
            }

            if dist_x.abs() == 1 {
                if dist_x > 0 {
                    self.x -= 1;
                } else {
                    self.x += 1;
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let actions = parse_input(input);
    let (mut grid, mut head) = build_grid(&actions);

    let mut knots: Vec<Knot> = Vec::new();
    let number_of_knots = 1;
    for _i in 0..number_of_knots {
        knots.push(head);
    }

    let mut count = 0;
    for (dir, step) in actions {
        for _i in 0..step {
            match dir {
                'L' => head.x -= 1,
                'R' => head.x += 1,
                'U' => head.y += 1,
                'D' => head.y -= 1,
                _ => {}
            }

            let mut parent = head;

            for i in 0..knots.len() {
                if i > 0 {
                    parent = knots[i - 1];
                }

                let knot = &mut knots[i];
                knot.move_closer_to(&parent);
            }

            if grid[knots.last().unwrap().x][knots.last().unwrap().y] != '#' {
                count += 1;
                grid[knots.last().unwrap().x][knots.last().unwrap().y] = '#';
            }
        }
    }

    // for row in &grid {
    //     println!("{:?}", row);
    // }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let actions = parse_input(input);
    let (mut grid, mut head) = build_grid(&actions);

    let mut knots: Vec<Knot> = Vec::new();
    let number_of_knots = 9;
    for _i in 0..number_of_knots {
        knots.push(head);
    }

    let mut count = 0;
    for (dir, step) in actions {
        for _i in 0..step {
            match dir {
                'L' => head.x -= 1,
                'R' => head.x += 1,
                'U' => head.y += 1,
                'D' => head.y -= 1,
                _ => {}
            }

            let mut parent = head;

            for i in 0..knots.len() {
                if i > 0 {
                    parent = knots[i - 1];
                }

                let knot = &mut knots[i];
                knot.move_closer_to(&parent);
            }

            if grid[knots.last().unwrap().x][knots.last().unwrap().y] != '#' {
                count += 1;
                grid[knots.last().unwrap().x][knots.last().unwrap().y] = '#';
            }
        }
    }

    // for row in &grid {
    //     println!("{:?}", row);
    // }

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_not_move_closer_to() {
        let mut tail = Knot { x: 3, y: 3 };
        for x_offset in 2..5 {
            for y_offset in 2..5 {
                let head = Knot {
                    x: x_offset,
                    y: y_offset,
                };
                tail.move_closer_to(&head);
                assert_eq!(tail, tail);
            }
        }
    }

    #[test]
    fn test_should_move_closer_to() {
        let corners = |t_x: usize, t_y: usize, h_x: usize, h_y: usize, ex_x: usize, ex_y: usize| {
            let mut tail = Knot { x: t_x, y: t_y };
            let head = Knot { x: h_x, y: h_y };
            let expected = Knot { x: ex_x, y: ex_y };

            tail.move_closer_to(&head);
            assert_eq!(tail, expected);
        };

        // Corners
        corners(2, 2, 4, 4, 3, 3);
        corners(2, 6, 4, 4, 3, 5);
        corners(6, 2, 4, 4, 5, 3);
        corners(6, 6, 4, 4, 5, 5);

        // // Up, Down, Left, Right
        corners(2, 4, 4, 4, 3, 4);
        corners(4, 2, 4, 4, 4, 3);
        corners(6, 4, 4, 4, 5, 4);
        corners(4, 6, 4, 4, 4, 5);

        corners(2, 3, 4, 4, 3, 4);
        corners(3, 2, 4, 4, 4, 3);

        corners(3, 6, 4, 4, 4, 5);
        corners(6, 3, 4, 4, 5, 4);

        corners(5, 6, 4, 4, 4, 5);
        corners(6, 5, 4, 4, 5, 4);

        corners(5, 2, 4, 4, 4, 3);
        corners(2, 5, 4, 4, 3, 4);

        corners(2, 3, 5, 5, 3, 4);
        corners(3, 2, 5, 5, 4, 3);
        corners(5, 6, 3, 3, 4, 5);
        corners(6, 5, 3, 3, 5, 4);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13)); // First example input
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1)); // First example input
                                               //assert_eq!(part_two(&input), Some(36)); // Second example input
    }
}
