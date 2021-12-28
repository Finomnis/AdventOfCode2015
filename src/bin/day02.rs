use std::num::ParseIntError;

use advent_of_code_helpers::*;

fn parse_input(input: &str) -> Result<Vec<Vec<u32>>> {
    Ok(input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split("x")
                .map(str::parse::<u32>)
                .collect::<Result<Vec<_>, ParseIntError>>()
        })
        .collect::<Result<Vec<_>, ParseIntError>>()?)
}

fn task1(input_data: Vec<Vec<u32>>) -> Result<u32> {
    Ok(input_data
        .into_iter()
        .map(|present| {
            let (min, val) = present
                .iter()
                .combinations(2)
                .map(|e| e.into_iter().product())
                .fold((u32::MAX, 0), |(min, sum), val| {
                    (std::cmp::min(min, val), sum + 2 * val)
                });
            val + min
        })
        .sum())
}

fn task2(input_data: Vec<Vec<u32>>) -> Result<u32> {
    Ok(input_data
        .into_iter()
        .map(|mut present| {
            present.sort();
            present[0..2].iter().sum::<u32>() * 2 + present.into_iter().product::<u32>()
        })
        .sum())
}

aoc_tests! {
    task1: {
        simple => 58+43,
        complex => 1598415,
    },
    task2: {
        simple => 34+14,
        complex => 3812909,
    }
}
