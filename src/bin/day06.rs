use std::ops::RangeInclusive;

use advent_of_code_helpers::*;

#[derive(Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Command {
    action: Action,
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

fn parse_input(input: &str) -> Result<Vec<Command>> {
    let re = Regex::new(r"^(turn (?:on|off)|toggle) (\d+),(\d+) through (\d+),(\d+)$")?;
    Ok(input
        .trim()
        .lines()
        .map(|line| {
            let captures = re
                .captures(line)
                .ok_or(anyhow!("Invalid line: '{}'", line))?;
            let x = captures[2].parse()?..=captures[4].parse()?;
            let y = captures[3].parse()?..=captures[5].parse()?;
            let action = match &captures[1] {
                "turn on" => Action::On,
                "turn off" => Action::Off,
                "toggle" => Action::Toggle,
                _ => bail!(
                    "Invalid action '{}' received, should never happen (should be prevented by regex)", &captures[1]
                ),
            };
            Ok(Command { x, y, action })
        })
        .collect::<Result<Vec<_>>>()?)
}

fn task1(input_data: Vec<Command>) -> Result<usize> {
    //println!("{:?}", input_data);
    let mut map = Array2::from_shape_simple_fn((1000, 1000), || false);

    for command in input_data {
        let mut slice = map.slice_mut(ndarray::s![command.y, command.x]);

        match command.action {
            Action::On => slice.fill(true),
            Action::Off => slice.fill(false),
            Action::Toggle => slice.iter_mut().for_each(|e| *e = !*e),
        }
    }

    Ok(map.into_iter().filter(|e| *e).count())
}

fn task2(input_data: Vec<Command>) -> Result<i32> {
    let mut map = Array2::from_shape_simple_fn((1000, 1000), || 0);

    for command in input_data {
        let mut slice = map.slice_mut(ndarray::s![command.y, command.x]);

        slice.iter_mut().for_each(|val| {
            *val = match command.action {
                Action::On => *val + 1,
                Action::Off => std::cmp::max(0, *val - 1),
                Action::Toggle => *val + 2,
            }
        })
    }

    Ok(map.into_iter().sum())
}

aoc_tests! {
    task1: {
        simple => 620000,
        complex => 377891,
    },
    task2: {
        complex => 14110788,
    }
}
