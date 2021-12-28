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

fn task2(_input_data: Vec<&str>) -> Result<u32> {
    Ok(0)
}

aoc_tests! {
    task1: {
        simple1 => 2,
        complex => 258,
    },
    task2: {
        simple2 => 2,
        complex => 0,
    }
}
