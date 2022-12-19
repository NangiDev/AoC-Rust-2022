use std::collections::BTreeMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut table: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut pairs: Vec<Vec<usize>> = Vec::new();

    for mut line in input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|p| p.to_string())
    {
        let mut node: Vec<usize> = Vec::new();
        while let Some(r_i) = line.find(']') {
            let mut l_i = r_i;
            while let Some(r_ch) = line.chars().nth(l_i) {
                if r_ch == '[' {
                    break;
                }
                l_i -= 1;
            }

            let arr = &line.clone()[l_i..=r_i];
            line = line.replace(arr, &(table.len() + 1000).to_string());
            let arr = &arr[1..arr.len() - 1];

            node = arr
                .split(',')
                .filter_map(|ch| ch.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            table.insert(table.len() + 1000, node.clone());
        }
        pairs.push(node);
    }

    let mut sum = 0;
    for p_i in (0..pairs.len() - 1).step_by(2) {
        let left = &pairs[p_i];
        let right = &pairs[p_i + 1];

        let mut correct_order = true;
        compare_vec(left, right, &table, &mut correct_order);
        if correct_order {
            sum += (p_i as u32 / 2) + 1;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

// Over 2456, under 5088,5176,5761,5793
fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn compare_vec(
    a: &Vec<usize>,
    b: &Vec<usize>,
    t: &BTreeMap<usize, Vec<usize>>,
    correct_order: &mut bool,
) {
    // println!("Left: {:?}", &a);
    // println!("Right: {:?}", &b);

    *correct_order = false;
    let range = a.len().max(b.len());

    if range == 0 {
        *correct_order = true;
        return;
    }

    for i in 0..range {
        if i >= a.len() {
            *correct_order = true;
            return;
        }
        let l_a = a[i];

        if i >= b.len() {
            *correct_order = false;
            return;
        }
        let l_b = b[i];

        if l_a > 1000 && l_b > 1000 {
            let mut l_a = t.get(&l_a).unwrap().clone();
            l_a.extend(&a[i + 1..].to_vec());
            let mut l_b = t.get(&l_b).unwrap().clone();
            l_b.extend(&b[i + 1..].to_vec());
            compare_vec(&l_a, &l_b, t, correct_order);
            return;
        } else if l_a > 1000 {
            let mut l_a = t.get(&l_a).unwrap().clone();
            l_a.extend(&a[i + 1..].to_vec());

            compare_vec(&l_a, &b[i..].to_vec(), t, correct_order);
            return;
        } else if l_b > 1000 {
            let mut l_b = t.get(&l_b).unwrap().clone();
            l_b.extend(&b[i + 1..].to_vec());

            compare_vec(&a[i..].to_vec(), &l_b, t, correct_order);
            return;
        }

        if l_a > l_b {
            *correct_order = false;
            return;
        }

        if l_a < l_b {
            *correct_order = true;
            return;
        }

        if i == a.len() - 1 && b.len() > a.len() {
            *correct_order = true;
            return;
        }

        if i == b.len() - 1 && a.len() > b.len() {
            *correct_order = false;
            return;
        }

        if l_a == l_b {
            continue;
        }
    }
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

    #[test]
    #[ignore]
    fn test_compare() {
        let mut table: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        table.insert(1001, vec![1]);
        table.insert(1002, vec![2, 3, 4]);
        table.insert(1003, vec![1]);
        table.insert(1004, vec![8, 7, 6]);
        table.insert(1005, vec![4, 4]);
        table.insert(1006, vec![1007]);
        table.insert(1007, vec![]);
        table.insert(1008, vec![2, 1009]);
        table.insert(1009, vec![3, 1010]);
        table.insert(1010, vec![4, 1011]);
        table.insert(1011, vec![5, 6, 7]);
        table.insert(1012, vec![2, 1013]);
        table.insert(1013, vec![3, 1014]);
        table.insert(1014, vec![4, 1015]);
        table.insert(1015, vec![5, 6, 0]);
        table.insert(1016, vec![0, 0, 0]);

        let mut correct_order = false;
        let left: Vec<usize> = vec![1, 1, 3, 1, 1];
        let right: Vec<usize> = vec![1, 1, 5, 1, 1];
        compare_vec(&left, &right, &table, &mut correct_order);
        assert_eq!(correct_order, true);

        let mut correct_order = false;
        let left: Vec<usize> = vec![1001, 1002];
        let right: Vec<usize> = vec![1003, 4];
        compare_vec(&left, &right, &table, &mut correct_order);
        assert_eq!(correct_order, true);

        let mut correct_order = true;
        let left: Vec<usize> = vec![9];
        let right: Vec<usize> = vec![1004];
        compare_vec(&left, &right, &table, &mut correct_order);
        assert_eq!(correct_order, false);

        let mut correct_order = false;
        let left: Vec<usize> = vec![1005, 4, 4];
        let right: Vec<usize> = vec![1005, 4, 4, 4];
        compare_vec(&left, &right, &table, &mut correct_order);
        assert_eq!(correct_order, true);

        let mut correct_order = true;
        let left: Vec<usize> = vec![7, 7, 7, 7];
        let right: Vec<usize> = vec![7, 7, 7];
        compare_vec(&left, &right, &table, &mut correct_order);
        assert_eq!(correct_order, false);

        let mut correct_order = false;
        let left: Vec<usize> = vec![];
        let right: Vec<usize> = vec![3];
        compare_vec(&left, &right, &table, &mut correct_order);
        assert_eq!(correct_order, true);

        let mut correct_order = true;
        let left: Vec<usize> = vec![1006];
        let right: Vec<usize> = vec![1007];
        compare_vec(&left, &right, &table, &mut correct_order);
        assert_eq!(correct_order, false);

        let mut correct_order = true;
        let left: Vec<usize> = vec![1, 1008, 8, 9];
        let right: Vec<usize> = vec![1, 1012, 8, 9];
        compare_vec(&left, &right, &table, &mut correct_order);
        assert_eq!(correct_order, false);

        let mut correct_order = false;
        let left: Vec<usize> = vec![1016];
        let right: Vec<usize> = vec![2];
        compare_vec(&left, &right, &table, &mut correct_order);
        assert_eq!(correct_order, true);

        let mut correct_order = false;
        let left: Vec<usize> = vec![];
        let right: Vec<usize> = vec![];
        compare_vec(&left, &right, &table, &mut correct_order);
        assert_eq!(correct_order, true);
    }
}
