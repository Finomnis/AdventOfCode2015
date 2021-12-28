use std::collections::HashMap;

use advent_of_code_helpers::*;

fn parse_input(input: &str) -> Result<Vec<&str>> {
    Ok(input.trim().lines().collect())
}

fn is_nice(s: &str) -> bool {
    let num_vowels = s
        .chars()
        .filter(|x| ['a', 'e', 'i', 'o', 'u'].contains(x))
        .count();
    let has_duplicate = s.chars().tuple_windows().any(|(a, b)| a == b);
    let has_forbidden = s
        .chars()
        .tuple_windows()
        .any(|x: (char, char)| [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')].contains(&x));

    num_vowels >= 3 && has_duplicate && !has_forbidden
}

fn task1(input_data: Vec<&str>) -> Result<usize> {
    Ok(input_data.into_iter().filter(|s| is_nice(*s)).count())
}

fn is_nicer(s: &str) -> bool {
    let mut pair_positions = HashMap::new();
    for (pos, pair) in s.chars().tuple_windows::<(char, char)>().enumerate() {
        pair_positions.entry(pair).or_insert(vec![]).push(pos);
    }
    let has_distinct_duplicate_pair =
        pair_positions
            .into_iter()
            .any(|(_, positions)| match positions.into_iter().minmax() {
                itertools::MinMaxResult::NoElements => false,
                itertools::MinMaxResult::OneElement(_) => false,
                itertools::MinMaxResult::MinMax(min, max) => max >= min + 2,
            });

    let has_separated_pair = s.chars().tuple_windows().any(|(a, _, c)| a == c);

    has_separated_pair && has_distinct_duplicate_pair
}

fn task2(input_data: Vec<&str>) -> Result<usize> {
    Ok(input_data.into_iter().filter(|s| is_nicer(*s)).count())
}

aoc_tests! {
    task1: {
        simple1 => 2,
        complex => 258,
    },
    task2: {
        simple2 => 2,
        complex => 53,
    }
}
