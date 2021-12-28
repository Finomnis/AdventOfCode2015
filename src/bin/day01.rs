use advent_of_code_helpers::*;

fn parse_input(input: &str) -> Result<&str> {
    Ok(input.trim())
}

fn task1(input_data: &str) -> Result<i32> {
    input_data.chars().try_fold(0, |floor, ch| match ch {
        '(' => Ok(floor + 1),
        ')' => Ok(floor - 1),
        _ => bail!("Invalid character '{}'!", ch),
    })
}

fn task2(input_data: &str) -> Result<usize> {
    let mut floor = 0;
    for (pos, ch) in input_data.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => bail!("Invalid character '{}'!", ch),
        };
        if floor < 0 {
            return Ok(pos + 1);
        }
    }

    bail!("Santa never reached the basement!")
}

aoc_tests! {
    task1: {
        simple => -3,
        complex => 232,
    },
    task2: {
        simple => 1,
        complex => 1783,
    }
}
