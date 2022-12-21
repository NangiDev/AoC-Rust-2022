use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialOrd, Ord, PartialEq)]
struct Unit {
    x: i32,
    y: i32,
}

impl Unit {
    fn calc_dist(&self, beacon: &Self) -> i32 {
        (self.x - beacon.x).abs() + (self.y - beacon.y).abs()
    }
    pub fn is_closer(&self, beacon: &Self, other: &Self) -> bool {
        self.calc_dist(beacon) >= self.calc_dist(other)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;

    let mut sens_beac: BTreeMap<Unit, Unit> = BTreeMap::new();
    input.lines().for_each(|s| {
        let t = &s
            .replace("Sensor at x=", "")
            .replace(", y=", " ")
            .replace(": closest beacon is at x=", " ")
            .replace("Sensor at x=", "");

        for t in t.split_whitespace().collect::<Vec<&str>>().windows(4) {
            let s_x = t[0].parse::<i32>().unwrap();
            let s_y = t[1].parse::<i32>().unwrap();
            let b_x = t[2].parse::<i32>().unwrap();
            let b_y = t[3].parse::<i32>().unwrap();
            x_min = x_min.min(s_x.min(b_x));
            y_min = y_min.min(s_y.min(b_y));
            x_max = x_max.max(s_x.max(b_x));
            y_max = y_max.max(s_y.max(b_y));

            sens_beac.insert(Unit { x: s_x, y: s_y }, Unit { x: b_x, y: b_y });
        }
    });

    let y = 10;
    //let y = 2_000_000;
    let mut count = 0;

    for x in x_min - y..=x_max + y {
        let other = Unit { x, y };

        for (sensor, beacon) in &sens_beac {
            if sensor.is_closer(beacon, &other) && sensor != &other && beacon != &other {
                count += 1;
                break;
            }
        }
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;

    let mut sens_beac: BTreeMap<Unit, Unit> = BTreeMap::new();
    input.lines().for_each(|s| {
        let t = &s
            .replace("Sensor at x=", "")
            .replace(", y=", " ")
            .replace(": closest beacon is at x=", " ")
            .replace("Sensor at x=", "");

        for t in t.split_whitespace().collect::<Vec<&str>>().windows(4) {
            let s_x = t[0].parse::<i32>().unwrap();
            let s_y = t[1].parse::<i32>().unwrap();
            let b_x = t[2].parse::<i32>().unwrap();
            let b_y = t[3].parse::<i32>().unwrap();
            x_min = x_min.min(s_x.min(b_x));
            y_min = y_min.min(s_y.min(b_y));
            x_max = x_max.max(s_x.max(b_x));
            y_max = y_max.max(s_y.max(b_y));

            sens_beac.insert(Unit { x: s_x, y: s_y }, Unit { x: b_x, y: b_y });
        }
    });

    let mut count = 0;
    //let coord_cap = 20;
    let coord_cap = 4_000_000;

    for y in y_min.max(0)..=y_max.min(coord_cap) {
        let mut x = x_min.max(0);
        while x <= x_max.min(coord_cap) {
            let other = Unit { x, y };

            let mut found = true;

            for (sensor, beacon) in &sens_beac {
                if sensor.is_closer(beacon, &other) {
                    found = false;
                    x = (sensor.x - beacon.x).abs().max((sensor.x - beacon.x).abs()) + sensor.x;
                    // if sensor.x > other.x {
                    //     x += (sensor.x - beacon.x).abs() * 2;
                    // } else {
                    //     x += (sensor.x - beacon.x).abs() + sensor.x;
                    // }
                    break;
                }
            }

            if found {
                count = other.x * 4_000_000 + y;
                return Some(count as u32);
            }
        }
    }

    Some(count as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
