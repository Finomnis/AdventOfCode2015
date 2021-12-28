use std::collections::HashSet;

use advent_of_code_helpers::*;

enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position(i32, i32);

impl Position {
    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.1 += 1,
            Direction::South => self.1 -= 1,
            Direction::West => self.0 -= 1,
            Direction::East => self.0 += 1,
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Direction>> {
    input
        .trim()
        .chars()
        .map(|ch| {
            Ok(match ch {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                _ => bail!("Invalid character: '{}'", ch),
            })
        })
        .collect()
}

fn task1(input_data: Vec<Direction>) -> Result<usize> {
    let (visited, _pos) = input_data.into_iter().fold(
        (HashSet::new(), Position(0, 0)),
        |(mut visited, mut pos), dir| {
            pos.step(dir);
            visited.insert(pos);
            (visited, pos)
        },
    );

    Ok(visited.len())
}

fn task2(input_data: Vec<Direction>) -> Result<usize> {
    let (visited, _pos1, _pos2) = input_data.into_iter().tuples().fold(
        (HashSet::new(), Position(0, 0), Position(0, 0)),
        |(mut visited, mut pos1, mut pos2), (dir1, dir2)| {
            pos1.step(dir1);
            pos2.step(dir2);
            visited.insert(pos1);
            visited.insert(pos2);
            (visited, pos1, pos2)
        },
    );

    Ok(visited.len())
}

aoc_tests! {
    task1: {
        simple => 4,
        complex => 2565,
    },
    task2: {
        simple => 3,
        complex => 2639,
    }
}
