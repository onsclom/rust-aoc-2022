use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("day3.txt");

    let char_to_num = |c: u8| match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => panic!("invalid input"),
    };

    let part1 = input
        .lines()
        .map(|line| {
            let length = line.len();
            (&line[..length / 2], &line[length / 2..])
        })
        .map(|(first, second)| {
            let first_chars: HashSet<_> = first.bytes().collect();
            second.bytes().find(|c| first_chars.contains(c)).unwrap()
        })
        .map(char_to_num)
        .fold(0, |acc, x| acc + x as u64);
    println!("part 1: {part1:?}");

    let part2 = input
        .lines()
        .tuples()
        .map(|(first, second, third)| {
            let first_chars: HashSet<_> = first.bytes().collect();
            let second_chars: HashSet<_> = second.bytes().collect();
            third
                .bytes()
                .find(|c| first_chars.contains(c) && second_chars.contains(c))
                .unwrap()
        })
        .map(char_to_num)
        .fold(0, |acc, x| acc + x as u64);
    println!("part 2: {part2:?}");
}
