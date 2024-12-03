fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    // Map each line to an array of int
    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|x| {
            x.split("   ")
                .filter(|x| !x.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // We sort the values in two different lists
    let count = lines.len();
    let mut left = vec![0; count];
    let mut right = vec![0; left.len()];
    for (li, line) in lines.iter().enumerate() {
        // Inserting each value at its correct position in its corresponding column
        if let [l_v, r_v, ..] = line[..] {
            let mut inserted = (false, false);
            for i in 0..li {
                if let (false, r_inserted) = inserted {
                    if *left.get(i).unwrap() > l_v {
                        left.insert(i, l_v);
                        inserted = (true, r_inserted);
                    }
                }
                if let (l_inserted, false) = inserted {
                    if *right.get(i).unwrap() > r_v {
                        right.insert(i, r_v);
                        inserted = (l_inserted, true);
                    }
                }
                if let (true, true) = inserted {
                    break;
                }
            }
            // If not inserted, we insert them at the end
            if let (false, _) = inserted {
                left.insert(li, l_v);
            }
            if let (_, false) = inserted {
                right.insert(li, r_v);
            }
        }
    }
    (left, right)
}

fn solve_a(columns: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (left, right) = columns;
    left.iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (right.get(i).unwrap() - x).abs())
}

fn solve_b(columns: &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut result = 0;
    let (left, right) = columns;
    let mut y = 0;
    for l in left {
        if y >= right.len() {
            break;
        }
        let mut r = *right.get(y).unwrap();
        while y < right.len() && r <= *l {
            if *l == r {
                result += r;
            }
            y += 1;
            if y < right.len() {
                r = *right.get(y).unwrap();
            }
        }
    }
    result
}
pub fn run() -> (i32, i32) {
    let input = include_str!("input/d1.txt");
    // Map each line to an array of int
    let columns = parse(input);
    let a = solve_a(&columns);
    let b = solve_b(&columns);
    (a, b)
}
