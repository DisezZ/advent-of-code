fn build(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn count_safe_report(reports: &Vec<Vec<u32>>) -> u32 {
    reports
        .iter()
        .filter(|report| check_is_safe(&report))
        .count() as u32
}

fn check_is_safe(report: &Vec<u32>) -> bool {
    let diff_list = report
        .iter()
        .zip(report.get(1..).unwrap().iter())
        .map(|(l, r)| *l as i32 - *r as i32)
        .collect::<Vec<_>>();

    diff_list
        .iter()
        .zip(diff_list.get(1..).unwrap().iter())
        .all(|(l, r)| l.abs() >= 1 && l.abs() <= 3 && r.abs() >= 1 && r.abs() <= 3 && *l * *r >= 0)
}

fn count_safe_report_tolerable(reports: &Vec<Vec<u32>>) -> u32 {
    reports
        .iter()
        .filter(|report| {
            let new_list: Vec<Vec<_>> = report
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    report
                        .iter()
                        .enumerate()
                        .filter(|&(j, _)| i != j)
                        .map(|(_, &values)| values)
                        .collect::<Vec<_>>()
                })
                .collect();
            new_list.iter().any(|sr| check_is_safe(sr))
        })
        .count() as u32
}

fn main() {
    let reports = build(include_str!("../input"));
    println!("Safe: {}", count_safe_report(&reports));
    println!("Safe tolerable: {}", count_safe_report_tolerable(&reports));
}
