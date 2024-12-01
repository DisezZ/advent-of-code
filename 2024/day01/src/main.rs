use itertools::Itertools;

fn build(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

fn total_distance(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .sorted_unstable()
        .zip(right_list.iter().sorted_unstable())
        .map(|(e1, e2)| e1.abs_diff(*e2))
        .sum()
}

fn similarity_score(left_list: &[u32], right_list: &[u32]) -> u32 {
    left_list
        .iter()
        .map(|left| left * right_list.iter().filter(|right| left == *right).count() as u32)
        .sum()
}

fn main() {
    let (left_list, right_list) = build(include_str!("../input.txt"));
    println!(
        "Total distance: {}",
        total_distance(&left_list, &right_list)
    );
    println!(
        "Similarity score: {}",
        similarity_score(&left_list, &right_list)
    );
}
