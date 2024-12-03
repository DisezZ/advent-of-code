use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Toggle(bool),
    Mul(u32, u32),
}

fn build(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
    re.captures_iter(&input)
        .map(|e| {
            e.extract::<2>()
                .1
                .iter()
                .map(|r| r.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32)>()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn build_part_2(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)|(do|don't)(\(\))").unwrap();
    let instructions: Vec<Instruction> = re
        .captures_iter(input)
        .map(|e| {
            let (_, caps) = e.extract::<2>();
            match caps[0] {
                "do" => Instruction::Toggle(true),
                "don't" => Instruction::Toggle(false),
                _ => {
                    let (l, r) = caps
                        .iter()
                        .map(|r| r.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Instruction::Mul(l, r)
                }
            }
        })
        .collect();
    instructions
}

fn main() {
    let input = include_str!("../input");
    let mul_list = build(&input);
    let instructions = build_part_2(&input);
    let sum1: u32 = mul_list.iter().map(|(l, r)| l * r).sum();

    let mut is_do = true;
    let sum2: u32 = instructions
        .iter()
        .map(|i| match i {
            Instruction::Toggle(to) => {
                is_do = *to;
                0
            }
            Instruction::Mul(l, r) if is_do => l * r,
            Instruction::Mul(_, _) => 0,
        })
        .sum();

    println!("Sum: {}", sum1);
    println!("Sum 2: {}", sum2);
}
