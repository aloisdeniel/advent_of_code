fn parse(input: &str) -> Vec<Vec<i32>> {
    // Map each line to an array of int
    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|x| {
            x.split(" ")
                .filter(|x| !x.is_empty())
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    lines
}

fn find_failure(report: &[i32]) -> Option<usize> {
    if report.is_empty() {
        return None;
    }
    let mut sig = 0;
    let mut previous = report.first().unwrap();
    for (i, v) in report.iter().skip(1).enumerate() {
        let new_step = v - previous;
        let new_sig = new_step.signum();
        if new_step.abs() > 3 || new_sig == 0 || (i > 0 && new_sig != sig) {
            return Some(i);
        }

        sig = new_sig;
        previous = v;
    }
    None
}

fn is_safe_with_failure(report: &[i32]) -> bool {
    if find_failure(report).is_some() {
        // Naive approach be removing every single item...
        for ignored in 0..report.len() {
            let fixture: Vec<i32> = report
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != ignored)
                .map(|(_, x)| *x)
                .collect();
            if find_failure(fixture.as_slice()).is_none() {
                return true;
            }
        }
        return false;
    }

    true
}

fn solve_a(reports: &Vec<Vec<i32>>) -> i32 {
    let mut result = reports.len() as i32;
    for report in reports {
        if find_failure(report).is_some() {
            result -= 1;
        }
    }
    result
}

fn solve_b(reports: &Vec<Vec<i32>>) -> i32 {
    let mut result = reports.len() as i32;
    for report in reports {
        if !is_safe_with_failure(report) {
            result -= 1;
        }
    }
    result
}
pub fn run() -> (i32, i32) {
    let input = include_str!("input/d2.txt");
    // Map each line to an array of int
    let reports = parse(input);
    let a = solve_a(&reports);
    let b = solve_b(&reports);
    (a, b)
}
