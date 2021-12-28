use advent_of_code_helpers::*;

use md5;

fn parse_input(input: &str) -> Result<&str> {
    Ok(input.trim())
}

fn task1(input_data: &str) -> Result<u32> {
    let mut context = md5::Context::new();
    context.consume(input_data);

    let mut num = 0;
    loop {
        num += 1;
        let mut context = context.clone();
        context.consume(format!("{}", num));
        let digest = context.compute();

        if digest[0..2] == [0; 2] && digest[2] & 0xf0 == 0 {
            //println!("{}{} {:x}", input_data, num, digest);
            return Ok(num);
        }
    }
}

fn task2(input_data: &str) -> Result<u32> {
    let mut context = md5::Context::new();
    context.consume(input_data);

    let mut num = 0;
    loop {
        num += 1;
        let mut context = context.clone();
        context.consume(format!("{}", num));
        let digest = context.compute();

        if digest[0..3] == [0; 3] {
            //println!("{}{} {:x}", input_data, num, digest);
            return Ok(num);
        }
    }
}

aoc_tests! {
    task1: {
        simple => 609043,
        complex => 254575,
    },
    task2: {
        complex => 1038736,
    }
}
