const TO_CHECK: &str = "XMAS";
const TO_CHECK_X_MAS: &str = "MAS";

fn build(input: &str) -> Vec<String> {
    input.lines().map(|f| f.to_string()).collect()
}

fn count_xmas(board: &Vec<String>) -> u32 {
    board
        .get(..board.len())
        .unwrap()
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.get(..line.len())
                .unwrap()
                .chars()
                .enumerate()
                .map(|(column, _)| check_xmas(&board, &row, &column))
                .sum::<u32>()
        })
        .sum()
}

fn check_xmas(board: &Vec<String>, row: &usize, column: &usize) -> u32 {
    let mut all_str: Vec<Vec<char>> = vec![Vec::new(); 8];
    let mut count = 0;
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    for i in 0..4 {
        for (index, direction) in directions.iter().enumerate() {
            let (cur_y, cur_x) = (
                (*row as i32 + direction.0 * i) as usize,
                (*column as i32 + direction.1 * i) as usize,
            );
            if !(0..board.len()).contains(&cur_y) || !(0..board[cur_y].len()).contains(&cur_x) {
                continue;
            }
            all_str[index].push(board[cur_y].chars().nth(cur_x).unwrap());
        }
    }
    for str_vec in all_str {
        let xmas = str_vec.iter().collect::<String>();
        if xmas == TO_CHECK {
            count += 1;
        }
    }
    count
}

fn check_x_mas(board: &Vec<String>, row: &usize, column: &usize) -> u32 {
    let directions = vec![
        vec![(1, -1), (0, 0), (-1, 1)],
        vec![(-1, -1), (0, 0), (1, 1)],
    ];
    let x_mas_lr: String = directions[0]
        .iter()
        .map(|(i, j)| {
            let (cur_y, cur_x) = ((*row as i32 + *i) as usize, (*column as i32 + *j) as usize);
            println!(
                "A: {:?} {:?} {:?} {:?} {:?}",
                *row, *column, *i, cur_x, cur_y
            );
            board[cur_y].chars().nth(cur_x).unwrap()
        })
        .collect();
    let x_mas_rl: String = directions[1]
        .iter()
        .map(|(i, j)| {
            let (cur_y, cur_x) = ((*row as i32 + *i) as usize, (*column as i32 + *j) as usize);
            println!("{:?} {:?}", cur_x, cur_y);
            board[cur_y].chars().nth(cur_x).unwrap()
        })
        .collect();
    let x_mas_rl_rev: String = x_mas_rl.chars().rev().collect();
    let x_mas_lr_rev: String = x_mas_lr.chars().rev().collect();
    println!("X_MAS: {} {}", x_mas_lr, x_mas_rl);
    println!("X_MAS: {} {}", x_mas_lr_rev, x_mas_rl_rev);
    if (x_mas_lr == TO_CHECK_X_MAS || x_mas_lr_rev == TO_CHECK_X_MAS)
        && (x_mas_rl == TO_CHECK_X_MAS || x_mas_rl_rev == TO_CHECK_X_MAS)
    {
        1
    } else {
        0
    }
}

fn count_x_mas(board: &Vec<String>) -> u32 {
    (1..board.len() - 1)
        .zip(board)
        .map(|(row, line)| {
            (1..line.len() - 1)
                .zip(line.chars())
                .map(|(column, _)| check_x_mas(&board, &row, &column))
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn main() {
    let input = include_str!("../input");
    let board = build(&input);
    let count_1: u32 = count_xmas(&board);
    let count_2: u32 = count_x_mas(&board);
    println!("Count 1: {}", count_1);
    println!("Count 2: {}", count_2);
}
