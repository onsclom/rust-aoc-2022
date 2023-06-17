fn main() {
    let input = include_str!("day1.txt");
    let lines = input.lines();

    let mut sum = 0;
    // make new vec of u32
    let mut sums: Vec<u32> = Vec::new();

    for line in lines {
        match line {
            "" => {
                sums.push(sum);
                sum = 0;
            }
            _ => {
                sum += line.parse::<u32>().unwrap();
            }
        }
    }

    sums.sort();

    println!("part1: {:?}", sums.last());
    // last 3 of sums
    println!("part2: {:?}", &sums[sums.len() - 3..].iter().sum::<u32>());
}
