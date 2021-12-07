use aoc_2021::{puzzle_main, puzzle_tests, Puzzle};

fn solver_part1(input: Vec<String>) -> String {
    let solution = input
        .windows(2)
        .into_iter()
        .fold(0u32, |acc, window| {
            let first = window[0].parse::<i32>().ok().unwrap();
            let second = window[1].parse::<i32>().ok().unwrap();
            if first < second { 
                acc + 1
            } else { 
                acc
            }
        });
    
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let sums: Vec<i32> = input
        .windows(3)
        .into_iter()
        .map(|window| {
            let first = window[0].parse::<i32>().ok().unwrap();
            let second = window[1].parse::<i32>().ok().unwrap();
            let third = window[2].parse::<i32>().ok().unwrap();
            first + second + third
        })
        .collect();

    let solution = sums
        .windows(2)
        .into_iter()
        .fold(0u32, |acc, window| {
            let first = window[0];
            let second = window[1];
            if first < second { 
                acc + 1
            } else { 
                acc
            }
        });
    
    solution.to_string()
}

// fn solver_part2(input: Vec<String>) -> String {
//     let numbers: Vec<i32> = input
//         .into_iter()
//         .filter_map(|l| l.parse::<i32>().ok())
//         .collect();
        
//     let mut solution: u32 = 0;
//     for (i, number) in numbers.into_iter().enumerate() {
//         let sum = number + numbers[i+1] + numbers[i+2];
//     }

//     "".to_string()
// }

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("7", "5");