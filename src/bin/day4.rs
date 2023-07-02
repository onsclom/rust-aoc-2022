fn main() {
    let input = include_str!("day4.txt");
    let (part1, part2) = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let [[a, b], [c, d]] = [left, right].map(|range| {
                let nums = range.split_once('-').unwrap();
                [nums.0, nums.1].map(|x| x.parse::<u64>().unwrap())
            });
            let left_contains_right = a <= c && b >= d;
            let right_contains_left = c <= a && d >= b;
            let a_within_right = a >= c && a <= d;
            let b_within_right = b >= c && b <= d;
            let part1 = left_contains_right || right_contains_left;
            (part1, part1 || a_within_right || b_within_right)
        })
        .fold((0, 0), |(acc1, acc2), (x, y)| {
            (acc1 + x as u64, acc2 + y as u64)
        });
    println!("part1: {part1}, part2: {part2}");
}
