/*
   this works, but i'm not really proud of this solution
*/

use itertools::Itertools;
use std::{collections::VecDeque, vec};

fn main() {
    let input = include_str!("day5.txt");

    let mut stacks: Vec<VecDeque<char>> = vec![];
    for line in input.lines() {
        if line.starts_with("move") {
            let (_, amount, _, from, _, to) = line.split(' ').collect_tuple().unwrap();
            let [amount, from, to] = [amount, from, to].map(|x| x.parse::<u8>().unwrap());
            for _ in 0..amount {
                let char = stacks[from as usize - 1].pop_back().unwrap();
                stacks[to as usize - 1].push_back(char);
            }
        } else {
            for (i, char) in line.chars().enumerate() {
                match char {
                    'A'..='Z' => {
                        let row = (i - 1) / 4;
                        match stacks.get_mut(row) {
                            Some(stack) => stack.push_front(char),
                            None => {
                                while stacks.len() <= row {
                                    stacks.push(VecDeque::new());
                                }
                                stacks[row].push_front(char);
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    print!("part 1: ");
    stacks
        .iter()
        .for_each(|stack| print!("{}", stack.back().unwrap()));
    println!();

    let mut stacks: Vec<VecDeque<char>> = vec![];
    for line in input.lines() {
        if line.starts_with("move") {
            let (_, amount, _, from, _, to) = line.split(' ').collect_tuple().unwrap();
            let [amount, from, to] = [amount, from, to].map(|x| x.parse::<u8>().unwrap());
            let mut popped = vec![];
            for _ in 0..amount {
                popped.push(stacks[from as usize - 1].pop_back().unwrap());
            }
            for char in popped.into_iter().rev() {
                stacks[to as usize - 1].push_back(char);
            }
        } else {
            for (i, char) in line.chars().enumerate() {
                match char {
                    'A'..='Z' => {
                        let row = (i - 1) / 4;
                        match stacks.get_mut(row) {
                            Some(stack) => stack.push_front(char),
                            None => {
                                while stacks.len() <= row {
                                    stacks.push(VecDeque::new());
                                }
                                stacks[row].push_front(char);
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    print!("part 2: ");
    stacks
        .iter()
        .for_each(|stack| print!("{}", stack.back().unwrap()));
    println!();
}
